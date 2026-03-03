use crate::behavior::Behavior;
use crate::behavior::CompositeDisposable;

use super::SliderWithTicksViewModel;

pub struct SliderWithTicksBehavior;

impl Behavior<SliderWithTicksViewModel> for SliderWithTicksBehavior {
    fn activate(
        &self,
        _view_model: &mut SliderWithTicksViewModel,
        _disposables: &mut CompositeDisposable,
    ) {
    }
}
