use crate::behavior::{Behavior, CompositeDisposable};

use super::floor_view_model::FloorCanvasViewModel;

pub struct ScalePositionsBehavior;

impl Behavior<FloorCanvasViewModel> for ScalePositionsBehavior {
    fn activate(
        &self,
        _view_model: &mut FloorCanvasViewModel,
        _disposables: &mut CompositeDisposable,
    ) {
    }
}

