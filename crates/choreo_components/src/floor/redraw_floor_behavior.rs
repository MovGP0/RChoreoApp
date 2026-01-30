use std::time::Duration;

use crate::behavior::{Behavior, CompositeDisposable};
use crate::behavior::TimerDisposable;
use crate::logging::BehaviorLog;
use crossbeam_channel::Receiver;
use nject::injectable;
use slint::TimerMode;

use super::floor_view_model::FloorCanvasViewModel;
use crate::choreography_settings::RedrawFloorCommand;

#[derive(Default, Clone)]
#[injectable]
#[inject(
    |receiver: Receiver<RedrawFloorCommand>| {
        Self::new(receiver)
    }
)]
pub struct RedrawFloorBehavior {
    receiver: Option<Receiver<RedrawFloorCommand>>,
}

impl RedrawFloorBehavior {
    pub fn new(receiver: Receiver<RedrawFloorCommand>) -> Self
    {
        Self {
            receiver: Some(receiver),
        }
    }
}

impl Behavior<FloorCanvasViewModel> for RedrawFloorBehavior {
    fn activate(
        &self,
        view_model: &mut FloorCanvasViewModel,
        disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated("RedrawFloorBehavior", "FloorCanvasViewModel");
        let Some(receiver) = self.receiver.clone() else {
            return;
        };
        let Some(view_model_handle) = view_model.self_handle().and_then(|handle| handle.upgrade())
        else {
            return;
        };

        let view_model = std::rc::Rc::clone(&view_model_handle);
        let timer = slint::Timer::default();
        timer.start(TimerMode::Repeated, Duration::from_millis(16), move || {
            let mut has_message = false;
            while receiver.try_recv().is_ok() {
                has_message = true;
            }

            if has_message {
                view_model.borrow_mut().draw_floor();
            }
        });

        disposables.add(Box::new(TimerDisposable::new(timer)));
    }
}
