use crossbeam_channel::Receiver;
use crate::behavior::{Behavior, CompositeDisposable};
use super::messages::DrawFloorCommand;
use super::types::FloorRenderGate;
use super::floor_view_model::FloorCanvasViewModel;

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


