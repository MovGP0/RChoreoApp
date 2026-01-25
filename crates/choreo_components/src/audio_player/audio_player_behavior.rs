use crate::behavior::{Behavior, CompositeDisposable};
use crate::logging::BehaviorLog;

use super::audio_player_view_model::AudioPlayerViewModel;
use super::types::AudioPlayer;

pub struct AudioPlayerBehavior;

impl AudioPlayerBehavior {
    pub fn attach_player(view_model: &mut AudioPlayerViewModel, mut player: Box<dyn AudioPlayer>) {
        sync_capabilities(view_model, player.as_ref());
        sync_parameters(view_model, player.as_mut());
        view_model.update_duration_label();
        view_model.player = Some(player);
    }

    pub fn sync_from_player(view_model: &mut AudioPlayerViewModel, player: &dyn AudioPlayer) {
        view_model.duration = player.duration();
        if player.is_playing() {
            view_model.position = player.current_position();
        }
        view_model.update_duration_label();
    }
}

impl Behavior<AudioPlayerViewModel> for AudioPlayerBehavior {
    fn activate(&self, _view_model: &mut AudioPlayerViewModel, _disposables: &mut CompositeDisposable) {
        BehaviorLog::behavior_activated("AudioPlayerBehavior", "AudioPlayerViewModel");
    }
}

fn sync_capabilities(view_model: &mut AudioPlayerViewModel, player: &dyn AudioPlayer) {
    view_model.can_seek = player.can_seek();
    view_model.can_set_speed = player.can_set_speed();
    view_model.duration = player.duration();
    view_model.update_duration_label();
}

fn sync_parameters(view_model: &AudioPlayerViewModel, player: &mut dyn AudioPlayer) {
    player.set_speed(view_model.speed);
    player.set_volume(view_model.volume);
    player.set_balance(view_model.balance);
    player.set_loop(view_model.loop_enabled);
}
