use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

use crate::observability::start_internal_span;

use super::AudioPlayerBackend;
use super::actions::AudioPlayerAction;
use super::messages::AudioPlayerPositionChangedEvent;
use super::messages::CloseAudioFileCommand;
use super::messages::LinkSceneToPositionCommand;
use super::messages::OpenAudioFileCommand;
use super::reducer::AudioPlayerEffect;
use super::reducer::reduce;
use super::runtime::AudioPlayerRuntime;
use super::runtime::apply_player_sample;
use super::state::AudioPlayerState;

pub struct AudioPlayerBehaviorDependencies {
    pub open_audio_receiver: Receiver<OpenAudioFileCommand>,
    pub close_audio_receiver: Receiver<CloseAudioFileCommand>,
    pub position_changed_senders: Vec<Sender<AudioPlayerPositionChangedEvent>>,
    pub link_scene_receiver: Receiver<LinkSceneToPositionCommand>,
    pub backend: AudioPlayerBackend,
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
        open_audio_file: OpenAudioFileBehavior::new(deps.open_audio_receiver, deps.backend),
        close_audio_file: CloseAudioFileBehavior::new(deps.close_audio_receiver),
        ticks: AudioPlayerTicksBehavior::new(),
        link_scene: AudioPlayerLinkSceneBehavior::new(deps.link_scene_receiver),
        position_changed: AudioPlayerPositionChangedBehavior::new(deps.position_changed_senders),
        haptic_feedback: deps.haptic_feedback,
    }
}

pub trait AudioPlayerHapticFeedback {
    fn is_supported(&self) -> bool;
    fn perform_click(&self);
}

pub struct OpenAudioFileBehavior {
    receiver: Receiver<OpenAudioFileCommand>,
    backend: AudioPlayerBackend,
}

impl OpenAudioFileBehavior {
    pub fn new(receiver: Receiver<OpenAudioFileCommand>, backend: AudioPlayerBackend) -> Self {
        Self { receiver, backend }
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

        if !runtime.has_player() {
            *runtime = AudioPlayerRuntime::new(self.backend);
        }
        runtime.open_file(command.file_path.clone());
        state.last_opened_audio_file_path = Some(command.file_path);
        state.last_trace_context = command.trace_context;
        state.has_stream_factory = true;
        if let Some(sample) = runtime.sample() {
            apply_player_sample(state, sample);
        }
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
        let mut latest = None;
        while let Ok(command) = self.receiver.try_recv() {
            latest = Some(command);
        }
        let Some(command) = latest else {
            return;
        };
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

pub struct AudioPlayerTicksBehavior;

impl Default for AudioPlayerTicksBehavior {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioPlayerTicksBehavior {
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    pub fn poll(&self, state: &mut AudioPlayerState, runtime: &mut AudioPlayerRuntime) {
        if let Some(sample) = runtime.sample() {
            apply_player_sample(state, sample);
        }
        let _ = reduce(state, AudioPlayerAction::UpdateTicksAndLinkState);
    }
}

pub struct AudioPlayerLinkSceneBehavior {
    receiver: Receiver<LinkSceneToPositionCommand>,
}

impl AudioPlayerLinkSceneBehavior {
    pub fn new(receiver: Receiver<LinkSceneToPositionCommand>) -> Self {
        Self { receiver }
    }

    pub fn poll(
        &self,
        state: &mut AudioPlayerState,
        haptic_feedback: Option<&dyn AudioPlayerHapticFeedback>,
    ) {
        let mut latest = None;
        while let Ok(command) = self.receiver.try_recv() {
            latest = Some(command);
        }
        let Some(command) = latest else {
            return;
        };
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
        if state.can_link_scene_to_position {
            let _ = reduce(state, AudioPlayerAction::LinkSceneToPosition);
            span.set_bool_attribute("choreo.success", true);
        } else {
            span.set_bool_attribute("choreo.success", false);
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
