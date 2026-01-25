use crate::behavior::{Behavior, CompositeDisposable};
use nject::injectable;

use super::floor_view_model::FloorCanvasViewModel;

#[injectable]
#[inject(|| Self)]
pub struct RedrawFloorBehavior;

impl Behavior<FloorCanvasViewModel> for RedrawFloorBehavior {
    fn activate(
        &self,
        _view_model: &mut FloorCanvasViewModel,
        _disposables: &mut CompositeDisposable,
    ) {
    }
}

