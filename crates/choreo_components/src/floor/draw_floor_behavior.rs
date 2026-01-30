use std::cell::RefCell;
use std::rc::Rc;

use crossbeam_channel::Receiver;
use nject::injectable;
use rxrust::observable::SubscribeNext;

use crate::behavior::{Behavior, CompositeDisposable};
use crate::behavior::SubscriptionDisposable;
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

    pub fn handle_draw_floor(&mut self) {
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
        let subject = view_model.draw_floor_subject();
        let subscription = subject.subscribe(move |_| {
            let mut behavior = behavior.borrow_mut();
            behavior.handle_draw_floor();
        });
        disposables.add(Box::new(SubscriptionDisposable::new(subscription)));
    }
}
