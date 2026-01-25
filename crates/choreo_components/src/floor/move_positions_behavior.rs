use crate::behavior::{Behavior, CompositeDisposable};

use super::floor_view_model::FloorCanvasViewModel;

pub struct MovePositionsBehavior;

impl Behavior<FloorCanvasViewModel> for MovePositionsBehavior {
    fn activate(
        &self,
        _view_model: &mut FloorCanvasViewModel,
        _disposables: &mut CompositeDisposable,
    ) {
    }
}

