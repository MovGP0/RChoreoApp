use crate::behavior::{Behavior, CompositeDisposable};

use super::floor_view_model::FloorCanvasViewModel;

pub struct ScaleAroundDancerBehavior;

impl Behavior<FloorCanvasViewModel> for ScaleAroundDancerBehavior {
    fn activate(
        &self,
        _view_model: &mut FloorCanvasViewModel,
        _disposables: &mut CompositeDisposable,
    ) {
    }
}

