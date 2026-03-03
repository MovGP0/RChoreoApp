use crate::behavior::Behavior;
use crate::behavior::CompositeDisposable;
use nject::injectable;

use super::SliderWithTicksViewModel;

#[injectable]
#[inject(|| Self)]
pub struct SliderWithTicksBehavior;

impl Behavior<SliderWithTicksViewModel> for SliderWithTicksBehavior {
    fn activate(
        &self,
        _view_model: &mut SliderWithTicksViewModel,
        _disposables: &mut CompositeDisposable,
    ) {
    }
}
