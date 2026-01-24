use std::io;
use std::path::Path;

use crossbeam_channel::{Receiver, Sender};
use choreo_models::SettingsPreferenceKeys;

use crate::behavior::{Behavior, CompositeDisposable};
use crate::logging::BehaviorLog;
use crate::global::GlobalStateModel;
use crate::preferences::Preferences;
use crate::scenes::SceneViewModel;

pub trait HapticFeedback {
    fn is_supported(&self) -> bool;
    fn perform_click(&self);
}

pub trait AudioPlayer {
    fn is_playing(&self) -> bool;
    fn can_seek(&self) -> bool;
    fn can_set_speed(&self) -> bool;
    fn duration(&self) -> f64;
    fn current_position(&self) -> f64;
    fn play(&mut self);
    fn pause(&mut self);
    fn stop(&mut self);
    fn seek(&mut self, position: f64);
    fn set_speed(&mut self, speed: f64);
    fn set_volume(&mut self, volume: f64);
    fn set_balance(&mut self, balance: f64);
    fn set_loop(&mut self, loop_enabled: bool);
}

pub type StreamFactory =
    Box<dyn Fn() -> io::Result<Box<dyn io::Read + Send>> + Send + Sync>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AudioPlayerPositionChangedEvent {
    pub position_seconds: f64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CloseAudioFileCommand;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenAudioFileCommand {
    pub file_path: String,
}

pub struct AudioPlayerViewModel {
    pub speed: f64,
    pub minimum_speed: f64,
    pub maximum_speed: f64,
    pub volume: f64,
    pub balance: f64,
    pub duration: f64,
    pub position: f64,
    pub tick_values: String,
    pub can_link_scene_to_position: bool,
    pub is_playing: bool,
    pub loop_enabled: bool,
    pub can_seek: bool,
    pub can_set_speed: bool,
    pub stream_factory: Option<StreamFactory>,
    pub preparation_seconds: f64,
    pub pause_seconds: f64,
    pub title: String,
    pub player: Option<Box<dyn AudioPlayer>>,
    pub haptic_feedback: Option<Box<dyn HapticFeedback>>,
}

impl AudioPlayerViewModel {
    pub fn new(haptic_feedback: Option<Box<dyn HapticFeedback>>) -> Self {
        Self {
            speed: 1.0,
            minimum_speed: 0.8,
            maximum_speed: 1.1,
            volume: 1.0,
            balance: 0.0,
            duration: 0.0,
            position: 0.0,
            tick_values: String::new(),
            can_link_scene_to_position: false,
            is_playing: false,
            loop_enabled: false,
            can_seek: false,
            can_set_speed: false,
            stream_factory: None,
            preparation_seconds: 4.0,
            pause_seconds: 0.0,
            title: "Audio".to_string(),
            player: None,
            haptic_feedback,
        }
    }

    pub fn toggle_play_pause(&mut self) {
        if let Some(haptic) = &self.haptic_feedback
            && haptic.is_supported()
        {
            haptic.perform_click();
        }

        let Some(player) = self.player.as_mut() else {
            return;
        };

        if player.is_playing() {
            player.pause();
            self.is_playing = false;
            return;
        }

        player.play();
        self.is_playing = true;
    }

    pub fn stop(&mut self) {
        let Some(player) = self.player.as_mut() else {
            return;
        };

        player.stop();
        self.is_playing = false;
        self.position = 0.0;
    }

    pub fn seek(&mut self, position: f64) {
        let Some(player) = self.player.as_mut() else {
            return;
        };

        if !player.can_seek() {
            return;
        }

        player.seek(position);
        self.position = player.current_position();
    }

    pub fn reload(&mut self) {
        // handled by the stream factory integration
    }

    pub fn link_scene_to_position(&mut self) {
        if let Some(haptic) = &self.haptic_feedback
            && haptic.is_supported()
        {
            haptic.perform_click();
        }
        // handled by behavior integration
    }
}

pub struct AudioPlayerViewState {
    is_user_dragging: bool,
    was_playing: bool,
    is_adjusting_speed: bool,
}

impl Default for AudioPlayerViewState {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioPlayerViewState {
    const SPEED_SNAP_STEP: f64 = 0.05;

    pub fn new() -> Self {
        Self {
            is_user_dragging: false,
            was_playing: false,
            is_adjusting_speed: false,
        }
    }

    pub fn on_position_drag_started(&mut self, view_model: &mut AudioPlayerViewModel) {
        self.is_user_dragging = true;
        self.was_playing = false;

        let Some(player) = view_model.player.as_mut() else {
            return;
        };

        if !player.is_playing() {
            return;
        }

        player.pause();
        view_model.is_playing = false;
        self.was_playing = true;
    }

    pub fn on_position_drag_completed(
        &mut self,
        view_model: &mut AudioPlayerViewModel,
        position: f64,
    ) {
        self.is_user_dragging = false;

        if !view_model.can_seek {
            return;
        }

        view_model.seek(position);

        if !self.was_playing {
            return;
        }

        let Some(player) = view_model.player.as_mut() else {
            return;
        };

        player.play();
        view_model.is_playing = true;
    }

    pub fn on_speed_changed(&mut self, view_model: &mut AudioPlayerViewModel, new_value: f64) {
        if self.is_adjusting_speed {
            return;
        }

        let snapped = snap_speed(
            new_value,
            view_model.minimum_speed,
            view_model.maximum_speed,
            Self::SPEED_SNAP_STEP,
        );

        if (snapped - new_value).abs() < 0.0001 {
            return;
        }

        self.is_adjusting_speed = true;
        view_model.speed = snapped;
        self.is_adjusting_speed = false;
    }

    pub fn is_user_dragging(&self) -> bool {
        self.is_user_dragging
    }
}

pub struct AudioPlayerBehavior;

impl AudioPlayerBehavior {
    pub fn attach_player(view_model: &mut AudioPlayerViewModel, mut player: Box<dyn AudioPlayer>) {
        sync_capabilities(view_model, player.as_ref());
        sync_parameters(view_model, player.as_mut());
        view_model.player = Some(player);
    }

    pub fn sync_from_player(view_model: &mut AudioPlayerViewModel, player: &dyn AudioPlayer) {
        view_model.duration = player.duration();
        if player.is_playing() {
            view_model.position = player.current_position();
        }
    }
}

impl Behavior<AudioPlayerViewModel> for AudioPlayerBehavior {
    fn activate(&self, _view_model: &mut AudioPlayerViewModel, _disposables: &mut CompositeDisposable) {
        BehaviorLog::behavior_activated("AudioPlayerBehavior", "AudioPlayerViewModel");
    }
}

pub struct AudioPlayerPositionChangedBehavior {
    publisher: Sender<AudioPlayerPositionChangedEvent>,
}

impl AudioPlayerPositionChangedBehavior {
    pub fn new(publisher: Sender<AudioPlayerPositionChangedEvent>) -> Self {
        Self { publisher }
    }

    pub fn publish(&self, position_seconds: f64) {
        let _ = self
            .publisher
            .send(AudioPlayerPositionChangedEvent { position_seconds });
    }
}

impl Behavior<AudioPlayerViewModel> for AudioPlayerPositionChangedBehavior {
    fn activate(&self, _view_model: &mut AudioPlayerViewModel, _disposables: &mut CompositeDisposable) {
        BehaviorLog::behavior_activated(
            "AudioPlayerPositionChangedBehavior",
            "AudioPlayerViewModel",
        );
    }
}

pub struct AudioPlayerTicksBehavior;

impl AudioPlayerTicksBehavior {
    pub fn update_ticks(view_model: &mut AudioPlayerViewModel, global_state: &GlobalStateModel) {
        update_ticks(view_model, &global_state.scenes);
    }
}

impl Behavior<AudioPlayerViewModel> for AudioPlayerTicksBehavior {
    fn activate(&self, _view_model: &mut AudioPlayerViewModel, _disposables: &mut CompositeDisposable) {
        BehaviorLog::behavior_activated("AudioPlayerTicksBehavior", "AudioPlayerViewModel");
    }
}

pub struct CloseAudioFileBehavior {
    receiver: Receiver<CloseAudioFileCommand>,
}

impl CloseAudioFileBehavior {
    pub fn new(receiver: Receiver<CloseAudioFileCommand>) -> Self {
        Self { receiver }
    }

    pub fn try_handle(&self, view_model: &mut AudioPlayerViewModel) -> bool {
        match self.receiver.try_recv() {
            Ok(_) => {
                Self::handle_close(view_model);
                true
            }
            Err(_) => false,
        }
    }

    fn handle_close(view_model: &mut AudioPlayerViewModel) {
        view_model.player = None;
        view_model.stream_factory = None;
        view_model.title = "Audio".to_string();
        view_model.position = 0.0;
        view_model.duration = 0.0;
        view_model.is_playing = false;
        view_model.can_seek = false;
        view_model.can_set_speed = false;
    }
}

impl Behavior<AudioPlayerViewModel> for CloseAudioFileBehavior {
    fn activate(&self, _view_model: &mut AudioPlayerViewModel, _disposables: &mut CompositeDisposable) {
        BehaviorLog::behavior_activated("CloseAudioFileBehavior", "AudioPlayerViewModel");
    }
}

pub struct OpenAudioFileBehavior<P: Preferences> {
    receiver: Receiver<OpenAudioFileCommand>,
    preferences: P,
}

impl<P: Preferences> OpenAudioFileBehavior<P> {
    pub fn new(receiver: Receiver<OpenAudioFileCommand>, preferences: P) -> Self {
        Self {
            receiver,
            preferences,
        }
    }

    pub fn try_handle(&self, view_model: &mut AudioPlayerViewModel) -> bool {
        match self.receiver.try_recv() {
            Ok(command) => {
                self.handle_open(view_model, command);
                true
            }
            Err(_) => false,
        }
    }

    fn handle_open(&self, view_model: &mut AudioPlayerViewModel, command: OpenAudioFileCommand) {
        if command.file_path.trim().is_empty() {
            return;
        }

        let file_path = command.file_path;
        let file_name = Path::new(&file_path)
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("Audio");
        view_model.title = file_name.to_string();

        let stream_path = file_path.clone();
        view_model.stream_factory = Some(Box::new(move || {
            let file = std::fs::File::open(&stream_path)?;
            Ok(Box::new(file) as Box<dyn io::Read + Send>)
        }));

        self.preferences
            .set_string(SettingsPreferenceKeys::LAST_OPENED_AUDIO_FILE, file_path);
    }
}

impl<P: Preferences> Behavior<AudioPlayerViewModel> for OpenAudioFileBehavior<P> {
    fn activate(&self, _view_model: &mut AudioPlayerViewModel, _disposables: &mut CompositeDisposable) {
        BehaviorLog::behavior_activated("OpenAudioFileBehavior", "AudioPlayerViewModel");
    }
}

pub struct AudioPlayerDependencies<P: Preferences> {
    pub open_audio_receiver: Receiver<OpenAudioFileCommand>,
    pub close_audio_receiver: Receiver<CloseAudioFileCommand>,
    pub position_changed_publisher: Sender<AudioPlayerPositionChangedEvent>,
    pub preferences: P,
}

pub fn build_audio_player_behaviors<P: Preferences + 'static>(
    deps: AudioPlayerDependencies<P>,
) -> Vec<Box<dyn Behavior<AudioPlayerViewModel>>> {
    vec![
        Box::new(AudioPlayerBehavior),
        Box::new(OpenAudioFileBehavior::new(
            deps.open_audio_receiver,
            deps.preferences,
        )),
        Box::new(CloseAudioFileBehavior::new(deps.close_audio_receiver)),
        Box::new(AudioPlayerTicksBehavior),
        Box::new(AudioPlayerLinkSceneBehavior),
        Box::new(AudioPlayerPositionChangedBehavior::new(
            deps.position_changed_publisher,
        )),
    ]
}

fn sync_capabilities(view_model: &mut AudioPlayerViewModel, player: &dyn AudioPlayer) {
    view_model.can_seek = player.can_seek();
    view_model.can_set_speed = player.can_set_speed();
    view_model.duration = player.duration();
}

fn sync_parameters(view_model: &AudioPlayerViewModel, player: &mut dyn AudioPlayer) {
    player.set_speed(view_model.speed);
    player.set_volume(view_model.volume);
    player.set_balance(view_model.balance);
    player.set_loop(view_model.loop_enabled);
}

pub struct AudioPlayerLinkSceneBehavior;

impl AudioPlayerLinkSceneBehavior {
    pub fn link_scene_to_position(
        view_model: &mut AudioPlayerViewModel,
        global_state: &mut GlobalStateModel,
    ) {
        let Some(selected_scene) = global_state.selected_scene.as_mut() else {
            return;
        };

        let Some(rounded_timestamp) =
            try_get_linked_timestamp(view_model, selected_scene, &global_state.scenes)
        else {
            return;
        };

        selected_scene.timestamp = Some(rounded_timestamp);

        if let Some(model_scene) = global_state
            .choreography
            .scenes
            .iter_mut()
            .find(|scene| scene.scene_id == selected_scene.scene_id)
        {
            model_scene.timestamp = Some(format_seconds(rounded_timestamp));
        }

        update_ticks(view_model, &global_state.scenes);
    }

    pub fn update_can_link(view_model: &mut AudioPlayerViewModel, global_state: &GlobalStateModel) {
        let can_link = global_state
            .selected_scene
            .as_ref()
            .and_then(|scene| try_get_linked_timestamp(view_model, scene, &global_state.scenes))
            .is_some();
        view_model.can_link_scene_to_position = can_link;
    }

    pub fn update_ticks(view_model: &mut AudioPlayerViewModel, global_state: &GlobalStateModel) {
        update_ticks(view_model, &global_state.scenes);
    }
}

impl Behavior<AudioPlayerViewModel> for AudioPlayerLinkSceneBehavior {
    fn activate(&self, _view_model: &mut AudioPlayerViewModel, _disposables: &mut CompositeDisposable) {
        BehaviorLog::behavior_activated(
            "AudioPlayerLinkSceneBehavior",
            "AudioPlayerViewModel",
        );
    }
}

fn try_get_linked_timestamp(
    view_model: &AudioPlayerViewModel,
    selected_scene: &SceneViewModel,
    scenes: &[SceneViewModel],
) -> Option<f64> {
    let selected_index = scenes
        .iter()
        .position(|scene| scene.scene_id == selected_scene.scene_id)?;

    let before_timestamp = selected_index
        .checked_sub(1)
        .and_then(|index| scenes[index].timestamp);
    let after_timestamp = scenes
        .get(selected_index + 1)
        .and_then(|scene| scene.timestamp);

    let rounded = round_to_100_millis(view_model.position);

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

fn update_ticks(view_model: &mut AudioPlayerViewModel, scenes: &[SceneViewModel]) {
    let max = view_model.duration;
    let mut ticks: Vec<f64> = scenes
        .iter()
        .filter_map(|scene| scene.timestamp)
        .filter(|value| max <= 0.0 || *value <= max)
        .collect();
    ticks.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    ticks.dedup_by(|a, b| (*a - *b).abs() < 0.000_5);

    let formatted: Vec<String> = ticks.into_iter().map(format_seconds).collect();
    view_model.tick_values = if formatted.is_empty() {
        String::new()
    } else {
        formatted.join(",")
    };
}

fn round_to_100_millis(seconds: f64) -> f64 {
    let milliseconds = seconds * 1000.0;
    let rounded = (milliseconds / 100.0).round() * 100.0;
    rounded / 1000.0
}

fn format_seconds(value: f64) -> String {
    let mut text = format!("{:.3}", value);
    if let Some(dot) = text.find('.') {
        while text.ends_with('0') {
            text.pop();
        }
        if text.ends_with('.') {
            text.pop();
        }
        if text.len() == dot {
            text.push('0');
        }
    }
    text
}

pub enum PlayPauseGlyph {
    Play,
    Pause,
}

impl PlayPauseGlyph {
    pub fn as_icon_name(&self) -> &'static str {
        match self {
            PlayPauseGlyph::Play => "play",
            PlayPauseGlyph::Pause => "pause",
        }
    }
}

pub fn play_pause_glyph(is_playing: bool) -> PlayPauseGlyph {
    if is_playing {
        PlayPauseGlyph::Pause
    } else {
        PlayPauseGlyph::Play
    }
}

pub fn duration_to_time_text(seconds: f64) -> String {
    if !seconds.is_finite() || seconds < 0.0 {
        return "0:00".to_string();
    }

    let total_seconds = seconds.round() as i64;
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    if hours >= 1 {
        format!("{hours}:{minutes:02}:{seconds:02}")
    } else {
        format!("{minutes}:{seconds:02}")
    }
}

pub fn speed_to_percent_text(speed: f64) -> String {
    if !speed.is_finite() {
        return "0%".to_string();
    }

    let percent = (speed * 100.0).round() as i64;
    format!("{percent}%")
}

fn snap_speed(value: f64, min: f64, max: f64, step: f64) -> f64 {
    let snapped = (value / step).round() * step;
    snapped.clamp(min, max)
}
