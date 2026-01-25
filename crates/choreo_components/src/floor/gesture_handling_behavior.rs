use crate::behavior::{Behavior, CompositeDisposable};
use nject::injectable;

use super::floor_view_model::FloorCanvasViewModel;

#[injectable]
#[inject(|| Self)]
pub struct GestureHandlingBehavior;

impl GestureHandlingBehavior {
    pub const TOUCH_PAN_FACTOR: f32 = 0.5;
}

impl Behavior<FloorCanvasViewModel> for GestureHandlingBehavior {
    fn activate(
        &self,
        _view_model: &mut FloorCanvasViewModel,
        _disposables: &mut CompositeDisposable,
    ) {
    }
}

