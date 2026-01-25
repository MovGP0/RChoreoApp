use crate::behavior::{Behavior, CompositeDisposable};
use crate::global::GlobalStateModel;
use crate::logging::BehaviorLog;
use nject::injectable;

use super::audio_player_link_scene_behavior::update_ticks;
use super::audio_player_view_model::AudioPlayerViewModel;

#[injectable]
#[inject(|| Self)]
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
