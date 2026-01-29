use std::cell::RefCell;
use std::rc::Rc;
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
    |view_model: Rc<RefCell<FloorCanvasViewModel>>, receiver: Receiver<RedrawFloorCommand>| {
        Self::new(view_model, receiver)
    }
)]
pub struct RedrawFloorBehavior {
    view_model: Option<Rc<RefCell<FloorCanvasViewModel>>>,
    receiver: Option<Receiver<RedrawFloorCommand>>,
}

impl RedrawFloorBehavior {
    pub fn new(
        view_model: Rc<RefCell<FloorCanvasViewModel>>,
        receiver: Receiver<RedrawFloorCommand>,
    ) -> Self
    {
        Self {
            view_model: Some(view_model),
            receiver: Some(receiver),
        }
    }

    pub fn handle_choreography_changed(&self, view_model: &mut FloorCanvasViewModel) {
        view_model.draw_floor();
    }

    pub fn handle_selected_scene_changed(&self, view_model: &mut FloorCanvasViewModel) {
        view_model.draw_floor();
    }

    pub fn handle_redraw_command(&self, view_model: &mut FloorCanvasViewModel) {
        view_model.draw_floor();
    }
}

impl Behavior<FloorCanvasViewModel> for RedrawFloorBehavior {
    fn activate(&self, _view_model: &mut FloorCanvasViewModel, disposables: &mut CompositeDisposable) {
        BehaviorLog::behavior_activated("RedrawFloorBehavior", "FloorCanvasViewModel");
        let Some(view_model) = self.view_model.clone() else {
            return;
        };
        let Some(receiver) = self.receiver.clone() else {
            return;
        };

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

