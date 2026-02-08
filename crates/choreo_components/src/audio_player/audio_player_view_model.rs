use std::cell::RefCell;
use std::rc::{Rc, Weak};

use crossbeam_channel::Sender;
use nject::injectable;

use crate::behavior::{Behavior, CompositeDisposable};

use super::messages::LinkSceneToPositionCommand;
use super::types::{AudioPlayer, StreamFactory};
use crate::haptics::HapticFeedback;

#[injectable]
#[inject(
    |haptic_feedback: Option<Box<dyn HapticFeedback>>,
     link_scene_sender: Sender<LinkSceneToPositionCommand>,
     behaviors: Vec<Box<dyn Behavior<AudioPlayerViewModel>>>| {
        Self::new(haptic_feedback, link_scene_sender, behaviors)
    }
)]
pub struct AudioPlayerViewModel {
    link_scene_sender: Sender<LinkSceneToPositionCommand>,
    behaviors: Vec<Box<dyn Behavior<AudioPlayerViewModel>>>,
    disposables: CompositeDisposable,
    self_handle: Option<Weak<RefCell<AudioPlayerViewModel>>>,
    pub speed: f64,
    pub minimum_speed: f64,
    pub maximum_speed: f64,
    pub volume: f64,
    pub balance: f64,
    pub duration: f64,
    pub position: f64,
    pub tick_values: Vec<f64>,
    pub speed_label: String,
    pub duration_label: String,
    pub can_link_scene_to_position: bool,
    pub is_playing: bool,
    pub loop_enabled: bool,
    pub can_seek: bool,
    pub can_set_speed: bool,
    pub stream_factory: Option<StreamFactory>,
    pub preparation_seconds: f64,
    pub pause_seconds: f64,
    pub player: Option<Box<dyn AudioPlayer>>,
    pub haptic_feedback: Option<Box<dyn HapticFeedback>>,
}

impl AudioPlayerViewModel {
    pub fn new(
        haptic_feedback: Option<Box<dyn HapticFeedback>>,
        link_scene_sender: Sender<LinkSceneToPositionCommand>,
        behaviors: Vec<Box<dyn Behavior<AudioPlayerViewModel>>>,
    ) -> Self {
        Self {
            link_scene_sender,
            behaviors,
            disposables: CompositeDisposable::new(),
            self_handle: None,
            speed: 1.0,
            minimum_speed: 0.8,
            maximum_speed: 1.1,
            volume: 1.0,
            balance: 0.0,
            duration: 0.0,
            position: 0.0,
            tick_values: Vec::new(),
            speed_label: speed_to_percent_text(1.0),
            duration_label: duration_to_time_text(0.0),
            can_link_scene_to_position: false,
            is_playing: false,
            loop_enabled: false,
            can_seek: false,
            can_set_speed: false,
            stream_factory: None,
            preparation_seconds: 4.0,
            pause_seconds: 0.0,
            player: None,
            haptic_feedback,
        }
    }

    pub fn activate(view_model: &Rc<RefCell<AudioPlayerViewModel>>) {
        let mut disposables = CompositeDisposable::new();
        {
            let mut view_model = view_model.borrow_mut();
            let behaviors = std::mem::take(&mut view_model.behaviors);
            for behavior in behaviors {
                behavior.activate(&mut view_model, &mut disposables);
            }
        }

        view_model.borrow_mut().disposables = disposables;
    }

    pub fn set_self_handle(&mut self, handle: Weak<RefCell<AudioPlayerViewModel>>) {
        self.self_handle = Some(handle);
    }

    pub fn self_handle(&self) -> Option<Weak<RefCell<AudioPlayerViewModel>>> {
        self.self_handle.clone()
    }

    pub fn set_player(&mut self, mut player: Box<dyn AudioPlayer>) {
        self.sync_capabilities(player.as_ref());
        self.sync_parameters(player.as_mut());
        self.update_duration_label();
        self.player = Some(player);
    }

    pub fn sync_from_player(&mut self) {
        let Some(player) = self.player.as_ref() else {
            return;
        };

        self.duration = player.duration();
        self.position = player.current_position();
        self.is_playing = player.is_playing();
        self.update_duration_label();
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

    pub fn seek_and_play(&mut self, position: f64) {
        let Some(player) = self.player.as_mut() else {
            return;
        };

        if !player.can_seek() {
            return;
        }

        player.seek_and_play(position);
        self.position = player.current_position();
        self.is_playing = true;
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
        let _ = self.link_scene_sender.try_send(LinkSceneToPositionCommand);
    }

    pub fn update_speed_label(&mut self) {
        self.speed_label = speed_to_percent_text(self.speed);
    }

    pub fn update_duration_label(&mut self) {
        self.duration_label = format!(
            "{} / {}",
            duration_to_time_text(self.position),
            duration_to_time_text(self.duration),
        );
    }

    fn sync_capabilities(&mut self, player: &dyn AudioPlayer) {
        self.can_seek = player.can_seek();
        self.can_set_speed = player.can_set_speed();
        self.duration = player.duration();
        self.update_duration_label();
    }

    fn sync_parameters(&self, player: &mut dyn AudioPlayer) {
        player.set_speed(self.speed);
        player.set_volume(self.volume);
        player.set_balance(self.balance);
        player.set_loop(self.loop_enabled);
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

        if self.was_playing {
            view_model.seek_and_play(position);
        } else {
            view_model.seek(position);
        }
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
        view_model.update_speed_label();
        self.is_adjusting_speed = false;
    }

    pub fn is_user_dragging(&self) -> bool {
        self.is_user_dragging
    }
}

impl Drop for AudioPlayerViewModel {
    fn drop(&mut self) {
        self.disposables.dispose_all();
    }
}

fn snap_speed(value: f64, min: f64, max: f64, step: f64) -> f64 {
    let clamped = value.clamp(min, max);
    let snapped = (clamped / step).round() * step;
    snapped.clamp(min, max)
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
