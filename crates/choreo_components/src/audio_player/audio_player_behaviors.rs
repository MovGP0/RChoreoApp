use std::path::Path;
use std::rc::Rc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

use choreo_master_mobile_json::SceneId;
use choreo_models::SceneModel;
use choreo_models::SettingsPreferenceKeys;

use crate::global::GlobalStateActor;
use crate::global::GlobalStateModel;
use crate::global::SceneViewModel;
use crate::observability::start_internal_span;
use crate::preferences::Preferences;
use crate::time::format_seconds;

use super::actions::AudioPlayerAction;
use super::audio_player_backend::AudioPlayerBackend;
use super::messages::AudioPlayerPositionChangedEvent;
use super::messages::CloseAudioFileCommand;
use super::messages::LinkSceneToPositionCommand;
use super::messages::OpenAudioFileCommand;
use super::reducer::AudioPlayerEffect;
use super::reducer::reduce;
use super::runtime::AudioPlayerRuntime;
use super::runtime::apply_player_sample;
use super::state::AudioPlayerChoreographyScene;
use super::state::AudioPlayerScene;
use super::state::AudioPlayerState;

pub struct AudioPlayerBehaviorDependencies {
    pub global_state_store: Rc<GlobalStateActor>,
    pub open_audio_receiver: Receiver<OpenAudioFileCommand>,
    pub close_audio_receiver: Receiver<CloseAudioFileCommand>,
    pub position_changed_senders: Vec<Sender<AudioPlayerPositionChangedEvent>>,
    pub link_scene_receiver: Receiver<LinkSceneToPositionCommand>,
    pub preferences: Rc<dyn Preferences>,
    pub haptic_feedback: Option<Box<dyn AudioPlayerHapticFeedback>>,
}

pub struct AudioPlayerBehaviorPipeline {
    pub open_audio_file: OpenAudioFileBehavior,
    pub close_audio_file: CloseAudioFileBehavior,
    pub ticks: AudioPlayerTicksBehavior,
    pub link_scene: AudioPlayerLinkSceneBehavior,
    pub position_changed: AudioPlayerPositionChangedBehavior,
    pub haptic_feedback: Option<Box<dyn AudioPlayerHapticFeedback>>,
}

pub fn build_audio_player_behaviors(
    deps: AudioPlayerBehaviorDependencies,
) -> AudioPlayerBehaviorPipeline {
    AudioPlayerBehaviorPipeline {
        open_audio_file: OpenAudioFileBehavior::new(
            deps.open_audio_receiver,
            Rc::clone(&deps.preferences),
        ),
        close_audio_file: CloseAudioFileBehavior::new(deps.close_audio_receiver),
        ticks: AudioPlayerTicksBehavior::new(Rc::clone(&deps.global_state_store)),
        link_scene: AudioPlayerLinkSceneBehavior::new(
            Rc::clone(&deps.global_state_store),
            deps.link_scene_receiver,
        ),
        position_changed: AudioPlayerPositionChangedBehavior::new(deps.position_changed_senders),
        haptic_feedback: deps.haptic_feedback,
    }
}

pub use crate::haptics::HapticFeedback as AudioPlayerHapticFeedback;

pub struct OpenAudioFileBehavior {
    receiver: Receiver<OpenAudioFileCommand>,
    preferences: Rc<dyn Preferences>,
}

impl OpenAudioFileBehavior {
    pub fn new(receiver: Receiver<OpenAudioFileCommand>, preferences: Rc<dyn Preferences>) -> Self {
        Self {
            receiver,
            preferences,
        }
    }

    pub fn poll(&self, state: &mut AudioPlayerState, runtime: &mut AudioPlayerRuntime) {
        let mut latest = None;
        while let Ok(command) = self.receiver.try_recv() {
            latest = Some(command);
        }
        let Some(command) = latest else {
            return;
        };

        let mut span = start_internal_span(
            "audio_player.open_audio_file",
            command.trace_context.as_ref(),
        );
        span.set_string_attribute("choreo.command.type", "OpenAudioFileCommand".to_string());

        if command.file_path.trim().is_empty() {
            span.set_bool_attribute("choreo.success", false);
            return;
        }

        let file_path = command.file_path;
        let has_audio_file = Path::new(file_path.as_str()).is_file();
        let selected_backend = AudioPlayerBackend::from_preference(
            self.preferences
                .get_string(
                    SettingsPreferenceKeys::AUDIO_PLAYER_BACKEND,
                    AudioPlayerBackend::RODIO_KEY,
                )
                .as_str(),
        );

        runtime.set_backend(selected_backend);
        if has_audio_file {
            runtime.open_file(file_path.clone());
        } else {
            runtime.close();
        }

        state.last_trace_context = command.trace_context;
        let _ = reduce(
            state,
            AudioPlayerAction::OpenAudioFile {
                file_path: file_path.clone(),
                file_exists: has_audio_file,
            },
        );

        if has_audio_file && let Some(sample) = runtime.sample() {
            apply_player_sample(state, sample);
        }

        self.preferences
            .set_string(SettingsPreferenceKeys::LAST_OPENED_AUDIO_FILE, file_path);
        span.set_bool_attribute("choreo.success", true);
    }
}

pub struct CloseAudioFileBehavior {
    receiver: Receiver<CloseAudioFileCommand>,
}

impl CloseAudioFileBehavior {
    pub fn new(receiver: Receiver<CloseAudioFileCommand>) -> Self {
        Self { receiver }
    }

    pub fn poll(&self, state: &mut AudioPlayerState, runtime: &mut AudioPlayerRuntime) {
        while let Ok(command) = self.receiver.try_recv() {
            let mut span = start_internal_span(
                "audio_player.close_audio_file",
                command.trace_context.as_ref(),
            );
            span.set_string_attribute("choreo.command.type", "CloseAudioFileCommand".to_string());

            runtime.close();
            state.last_trace_context = command.trace_context;
            let _ = reduce(state, AudioPlayerAction::CloseAudioFile);
            span.set_bool_attribute("choreo.success", true);
        }
    }
}

pub struct AudioPlayerTicksBehavior {
    global_state: Rc<GlobalStateActor>,
}

impl AudioPlayerTicksBehavior {
    #[must_use]
    pub fn new(global_state: Rc<GlobalStateActor>) -> Self {
        Self { global_state }
    }

    pub fn poll(&self, state: &mut AudioPlayerState, runtime: &mut AudioPlayerRuntime) {
        if let Some(sample) = runtime.sample() {
            apply_player_sample(state, sample);
        }
        sync_scenes_from_global_state(state, &self.global_state);
        let _ = reduce(state, AudioPlayerAction::UpdateTicksAndLinkState);
    }
}

impl Default for AudioPlayerTicksBehavior {
    fn default() -> Self {
        Self::new(GlobalStateActor::new())
    }
}

pub struct AudioPlayerLinkSceneBehavior {
    global_state: Rc<GlobalStateActor>,
    receiver: Receiver<LinkSceneToPositionCommand>,
}

impl AudioPlayerLinkSceneBehavior {
    pub fn new(
        global_state: Rc<GlobalStateActor>,
        receiver: Receiver<LinkSceneToPositionCommand>,
    ) -> Self {
        Self {
            global_state,
            receiver,
        }
    }

    pub fn poll(
        &self,
        state: &mut AudioPlayerState,
        haptic_feedback: Option<&dyn AudioPlayerHapticFeedback>,
    ) {
        while let Ok(command) = self.receiver.try_recv() {
            let mut span = start_internal_span(
                "audio_player.link_scene_to_position",
                command.trace_context.as_ref(),
            );
            span.set_string_attribute(
                "choreo.command.type",
                "LinkSceneToPositionCommand".to_string(),
            );
            state.last_trace_context = command.trace_context;
            if let Some(haptic) = haptic_feedback
                && haptic.is_supported()
            {
                haptic.perform_click();
            }

            let updated = self.global_state.try_update(|global_state| {
                handle_link_scene_to_position(state.position, global_state);
            });
            if !updated {
                span.set_bool_attribute("choreo.success", false);
                continue;
            }

            sync_scenes_from_global_state(state, &self.global_state);
            let _ = reduce(state, AudioPlayerAction::UpdateTicksAndLinkState);
            span.set_bool_attribute("choreo.success", true);
        }
    }
}

pub struct AudioPlayerPositionChangedBehavior {
    senders: Vec<Sender<AudioPlayerPositionChangedEvent>>,
}

impl AudioPlayerPositionChangedBehavior {
    pub fn new(senders: Vec<Sender<AudioPlayerPositionChangedEvent>>) -> Self {
        Self { senders }
    }

    pub fn poll(&self, state: &mut AudioPlayerState) {
        let effects = reduce(state, AudioPlayerAction::PublishPositionIfChanged);
        for effect in effects {
            match effect {
                AudioPlayerEffect::PositionChangedPublished { position_seconds } => {
                    let event = AudioPlayerPositionChangedEvent {
                        position_seconds,
                        trace_context: state.last_trace_context.clone(),
                    };
                    for sender in &self.senders {
                        let _ = sender.send(event.clone());
                    }
                }
            }
        }
    }
}

pub fn reduce_with_haptics(
    state: &mut AudioPlayerState,
    action: AudioPlayerAction,
    haptic_feedback: Option<&dyn AudioPlayerHapticFeedback>,
) {
    let should_click = matches!(
        action,
        AudioPlayerAction::TogglePlayPause | AudioPlayerAction::LinkSceneToPosition
    );
    if should_click
        && let Some(haptic) = haptic_feedback
        && haptic.is_supported()
    {
        haptic.perform_click();
    }
    let _ = reduce(state, action);
}

fn sync_scenes_from_global_state(state: &mut AudioPlayerState, global_state: &GlobalStateActor) {
    let Some((scenes, selected_scene_id, choreography_scenes)) =
        global_state.try_with_state(|state| {
            let scenes = state.scenes.iter().map(map_scene_model).collect();
            let selected_scene_id = state.selected_scene.as_ref().map(|scene| scene.scene_id.0);
            let choreography_scenes = state
                .choreography
                .scenes
                .iter()
                .map(map_choreography_scene)
                .collect();
            (scenes, selected_scene_id, choreography_scenes)
        })
    else {
        return;
    };

    let _ = reduce(
        state,
        AudioPlayerAction::SetScenes {
            scenes,
            selected_scene_id,
            choreography_scenes,
        },
    );
}

fn map_scene_model(scene: &SceneViewModel) -> AudioPlayerScene {
    AudioPlayerScene {
        scene_id: scene.scene_id.0,
        name: scene.name.clone(),
        timestamp: scene.timestamp,
    }
}

fn map_choreography_scene(scene: &SceneModel) -> AudioPlayerChoreographyScene {
    AudioPlayerChoreographyScene {
        scene_id: scene.scene_id.0,
        timestamp: scene.timestamp.clone(),
    }
}

fn handle_link_scene_to_position(position: f64, global_state: &mut GlobalStateModel) {
    let Some(selected_scene_id) = global_state
        .selected_scene
        .as_ref()
        .map(|scene| scene.scene_id)
    else {
        return;
    };
    let Some(linked_timestamp) =
        try_get_linked_timestamp(position, selected_scene_id, &global_state.scenes)
    else {
        return;
    };

    let formatted_timestamp = format_seconds(linked_timestamp);

    if let Some(selected_scene) = global_state.selected_scene.as_mut() {
        selected_scene.timestamp = Some(linked_timestamp);
    }

    if let Some(scene) = global_state
        .scenes
        .iter_mut()
        .find(|scene| scene.scene_id == selected_scene_id)
    {
        scene.timestamp = Some(linked_timestamp);
    }

    if let Some(scene) = global_state
        .choreography
        .scenes
        .iter_mut()
        .find(|scene| scene.scene_id == selected_scene_id)
    {
        scene.timestamp = Some(formatted_timestamp);
    }
}

fn try_get_linked_timestamp(
    position: f64,
    selected_scene_id: SceneId,
    scenes: &[SceneViewModel],
) -> Option<f64> {
    let selected_index = scenes
        .iter()
        .position(|scene| scene.scene_id == selected_scene_id)?;

    let before_timestamp = selected_index
        .checked_sub(1)
        .and_then(|index| scenes[index].timestamp);
    let after_timestamp = scenes
        .get(selected_index + 1)
        .and_then(|scene| scene.timestamp);

    let rounded = round_to_100_millis(position);

    if let Some(before) = before_timestamp
        && rounded <= before
    {
        return None;
    }

    if let Some(after) = after_timestamp
        && rounded >= after
    {
        return None;
    }

    Some(rounded)
}

fn round_to_100_millis(seconds: f64) -> f64 {
    let milliseconds = seconds * 1000.0;
    let rounded = (milliseconds / 100.0).round() * 100.0;
    rounded / 1000.0
}
