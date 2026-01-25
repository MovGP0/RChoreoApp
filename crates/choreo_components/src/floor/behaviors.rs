use crossbeam_channel::Receiver;

use crate::behavior::{Behavior, CompositeDisposable};

use super::types::{DrawFloorCommand, FloorRenderGate};
use super::view_model::FloorCanvasViewModel;

pub struct DrawFloorBehavior {
    receiver: Receiver<DrawFloorCommand>,
    render_gate: Option<Box<dyn FloorRenderGate>>,
    has_rendered: bool,
}

impl DrawFloorBehavior {
    pub fn new(receiver: Receiver<DrawFloorCommand>, render_gate: Option<Box<dyn FloorRenderGate>>) -> Self {
        Self {
            receiver,
            render_gate,
            has_rendered: false,
        }
    }

    pub fn try_handle(&mut self) -> bool {
        match self.receiver.try_recv() {
            Ok(_) => {
                if !self.has_rendered {
                    self.has_rendered = true;
                    if let Some(render_gate) = &self.render_gate {
                        render_gate.mark_rendered();
                    }
                }
                true
            }
            Err(_) => false,
        }
    }
}

impl Behavior<FloorCanvasViewModel> for DrawFloorBehavior {
    fn activate(&self, _view_model: &mut FloorCanvasViewModel, _disposables: &mut CompositeDisposable) {}
}

pub struct GestureHandlingBehavior;

impl GestureHandlingBehavior {
    pub const TOUCH_PAN_FACTOR: f32 = 0.5;
}

impl Behavior<FloorCanvasViewModel> for GestureHandlingBehavior {
    fn activate(&self, _view_model: &mut FloorCanvasViewModel, _disposables: &mut CompositeDisposable) {}
}

pub struct MovePositionsBehavior;

impl Behavior<FloorCanvasViewModel> for MovePositionsBehavior {
    fn activate(&self, _view_model: &mut FloorCanvasViewModel, _disposables: &mut CompositeDisposable) {}
}

pub struct PlacePositionBehavior;

impl Behavior<FloorCanvasViewModel> for PlacePositionBehavior {
    fn activate(&self, _view_model: &mut FloorCanvasViewModel, _disposables: &mut CompositeDisposable) {}
}

pub struct RedrawFloorBehavior;

impl Behavior<FloorCanvasViewModel> for RedrawFloorBehavior {
    fn activate(&self, _view_model: &mut FloorCanvasViewModel, _disposables: &mut CompositeDisposable) {}
}

pub struct RotateAroundCenterBehavior;

impl Behavior<FloorCanvasViewModel> for RotateAroundCenterBehavior {
    fn activate(&self, _view_model: &mut FloorCanvasViewModel, _disposables: &mut CompositeDisposable) {}
}

pub struct ScaleAroundDancerBehavior;

impl Behavior<FloorCanvasViewModel> for ScaleAroundDancerBehavior {
    fn activate(&self, _view_model: &mut FloorCanvasViewModel, _disposables: &mut CompositeDisposable) {}
}

pub struct ScalePositionsBehavior;

impl Behavior<FloorCanvasViewModel> for ScalePositionsBehavior {
    fn activate(&self, _view_model: &mut FloorCanvasViewModel, _disposables: &mut CompositeDisposable) {}
}
