use crate::behavior::{Behavior, CompositeDisposable};

use super::floor_view_model::FloorCanvasViewModel;

pub struct PlacePositionBehavior;

impl Behavior<FloorCanvasViewModel> for PlacePositionBehavior {
    fn activate(
        &self,
        _view_model: &mut FloorCanvasViewModel,
        _disposables: &mut CompositeDisposable,
    ) {
    }
}

