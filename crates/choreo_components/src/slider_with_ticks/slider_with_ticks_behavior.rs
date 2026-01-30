use crate::behavior::{Behavior, CompositeDisposable};
use crate::logging::BehaviorLog;
use nject::injectable;

use super::SliderWithTicksViewModel;

#[injectable]
#[inject(|| Self)]
pub struct SliderWithTicksBehavior;

impl Behavior<SliderWithTicksViewModel> for SliderWithTicksBehavior {
    fn initialize(
        &self,
        _view_model: &mut SliderWithTicksViewModel,
        _disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated(
            "SliderWithTicksBehavior",
            "SliderWithTicksViewModel",
        );
    }
}
