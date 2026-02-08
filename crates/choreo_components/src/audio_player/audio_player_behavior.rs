use nject::injectable;

use crate::behavior::{Behavior, CompositeDisposable};
use crate::logging::BehaviorLog;

use super::audio_player_view_model::AudioPlayerViewModel;

#[injectable]
#[inject(|| Self)]
pub struct AudioPlayerBehavior;

impl Behavior<AudioPlayerViewModel> for AudioPlayerBehavior {
    fn activate(
        &self,
        _view_model: &mut AudioPlayerViewModel,
        _disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated("AudioPlayerBehavior", "AudioPlayerViewModel");
    }
}
