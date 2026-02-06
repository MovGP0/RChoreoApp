use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use crossbeam_channel::Receiver;
use nject::injectable;
use slint::TimerMode;

use crate::behavior::{Behavior, CompositeDisposable};
use crate::behavior::TimerDisposable;
use crate::logging::BehaviorLog;

use super::floor_view_model::FloorCanvasViewModel;
use super::messages::DrawFloorCommand;
use super::types::FloorRenderGate;

#[derive(Clone)]
#[injectable]
#[inject(
    |receiver: Receiver<DrawFloorCommand>,
     render_gate: Option<Box<dyn FloorRenderGate>>| {
        Self::new(receiver, render_gate.map(Rc::from))
    }
)]
pub struct DrawFloorBehavior {
    receiver: Receiver<DrawFloorCommand>,
    render_gate: Option<Rc<dyn FloorRenderGate>>,
    has_rendered: bool,
}

impl DrawFloorBehavior {
    pub fn new(
        receiver: Receiver<DrawFloorCommand>,
        render_gate: Option<Rc<dyn FloorRenderGate>>,
    ) -> Self
    {
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

    fn handle_draw_floor(&mut self) {
        if self.has_rendered {
            return;
        }

        self.has_rendered = true;
        if let Some(render_gate) = &self.render_gate {
            render_gate.mark_rendered();
        }
    }
}

impl Behavior<FloorCanvasViewModel> for DrawFloorBehavior {
    fn activate(
        &self,
        view_model: &mut FloorCanvasViewModel,
        disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated("DrawFloorBehavior", "FloorCanvasViewModel");
        let behavior = Rc::new(RefCell::new(self.clone()));
        let view_model_handle = view_model.self_handle().and_then(|handle| handle.upgrade());
        let receiver = self.receiver.clone();
        let timer = slint::Timer::default();
        timer.start(TimerMode::Repeated, Duration::from_millis(16), move || {
            let mut has_message = false;
            while receiver.try_recv().is_ok() {
                has_message = true;
            }

            if has_message {
                behavior.borrow_mut().handle_draw_floor();
                if let Some(view_model_handle) = view_model_handle.as_ref() {
                    view_model_handle.borrow().request_redraw();
                }
            }
        });
        disposables.add(Box::new(TimerDisposable::new(timer)));
    }
}
