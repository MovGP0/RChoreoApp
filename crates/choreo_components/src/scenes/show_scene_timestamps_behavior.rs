use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use crossbeam_channel::Receiver;
use crate::global::GlobalStateModel;
use nject::injectable;
use slint::TimerMode;

use crate::behavior::{Behavior, CompositeDisposable, TimerDisposable};
use crate::logging::BehaviorLog;
use crate::choreography_settings::ShowTimestampsChangedEvent;

use super::scenes_view_model::ScenesPaneViewModel;

#[injectable]
#[inject(
    |global_state: Rc<RefCell<GlobalStateModel>>,
     receiver: Receiver<ShowTimestampsChangedEvent>| {
        Self::new(global_state, receiver)
    }
)]
pub struct ShowSceneTimestampsBehavior {
    global_state: Rc<RefCell<GlobalStateModel>>,
    receiver: Receiver<ShowTimestampsChangedEvent>,
}

impl ShowSceneTimestampsBehavior {
    pub fn new(
        global_state: Rc<RefCell<GlobalStateModel>>,
        receiver: Receiver<ShowTimestampsChangedEvent>,
    ) -> Self {
        Self { global_state, receiver }
    }

    fn update_from_choreography(&self, view_model: &mut ScenesPaneViewModel) {
        view_model.show_timestamps = self.global_state.borrow().choreography.settings.show_timestamps;
    }
}

impl Behavior<ScenesPaneViewModel> for ShowSceneTimestampsBehavior {
    fn activate(&self, view_model: &mut ScenesPaneViewModel, disposables: &mut CompositeDisposable) {
        BehaviorLog::behavior_activated("ShowSceneTimestampsBehavior", "ScenesPaneViewModel");
        self.update_from_choreography(view_model);
        let Some(view_model_handle) = view_model.self_handle().and_then(|handle| handle.upgrade())
        else {
            return;
        };
        let receiver = self.receiver.clone();
        let global_state = self.global_state.clone();
        let timer = slint::Timer::default();
        timer.start(TimerMode::Repeated, Duration::from_millis(16), move || {
            while let Ok(event) = receiver.try_recv() {
                let mut view_model = view_model_handle.borrow_mut();
                let value = event.is_enabled;
                view_model.show_timestamps = value;
                view_model.notify_changed();
                let mut global_state = global_state.borrow_mut();
                global_state.choreography.settings.show_timestamps = value;
            }
        });
        disposables.add(Box::new(TimerDisposable::new(timer)));
    }
}
