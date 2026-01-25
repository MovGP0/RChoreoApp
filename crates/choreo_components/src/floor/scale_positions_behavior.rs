use crate::behavior::{Behavior, CompositeDisposable};
use nject::injectable;

use super::floor_view_model::FloorCanvasViewModel;

#[injectable]
#[inject(|| Self)]
pub struct ScalePositionsBehavior;

impl Behavior<FloorCanvasViewModel> for ScalePositionsBehavior {
    fn activate(
        &self,
        _view_model: &mut FloorCanvasViewModel,
        _disposables: &mut CompositeDisposable,
    ) {
    }
}

