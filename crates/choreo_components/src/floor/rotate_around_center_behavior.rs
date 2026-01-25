use crate::behavior::{Behavior, CompositeDisposable};

use super::floor_view_model::FloorCanvasViewModel;

pub struct RotateAroundCenterBehavior;

impl Behavior<FloorCanvasViewModel> for RotateAroundCenterBehavior {
    fn activate(
        &self,
        _view_model: &mut FloorCanvasViewModel,
        _disposables: &mut CompositeDisposable,
    ) {
    }
}

