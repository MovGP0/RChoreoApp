use std::rc::Rc;
use std::time::Duration;

use crate::global::GlobalStateActor;
use crossbeam_channel::Receiver;
use nject::injectable;
use slint::TimerMode;

use crate::behavior::{Behavior, CompositeDisposable, TimerDisposable};
use crate::choreography_settings::ShowTimestampsChangedEvent;
use crate::logging::BehaviorLog;

use super::scenes_view_model::ScenesPaneViewModel;

#[injectable]
#[inject(
    |global_state: Rc<GlobalStateActor>,
     receiver: Receiver<ShowTimestampsChangedEvent>| {
        Self::new(global_state, receiver)
    }
)]
pub struct ShowSceneTimestampsBehavior {
    global_state: Rc<GlobalStateActor>,
    receiver: Receiver<ShowTimestampsChangedEvent>,
}

impl ShowSceneTimestampsBehavior {
    pub fn new(
        global_state: Rc<GlobalStateActor>,
        receiver: Receiver<ShowTimestampsChangedEvent>,
    ) -> Self {
        Self {
            global_state,
            receiver,
        }
    }

    fn update_from_choreography(&self, view_model: &mut ScenesPaneViewModel) {
        let Some(value) = self
            .global_state
            .try_with_state(|global_state| global_state.choreography.settings.show_timestamps)
        else {
            return;
        };
        view_model.show_timestamps = value;
    }
}

impl Behavior<ScenesPaneViewModel> for ShowSceneTimestampsBehavior {
    fn activate(
        &self,
        view_model: &mut ScenesPaneViewModel,
        disposables: &mut CompositeDisposable,
    ) {
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
                let updated = global_state.try_update(|global_state| {
                    global_state.choreography.settings.show_timestamps = value;
                });
                if !updated {
                    return;
                }
            }
        });
        disposables.add(Box::new(TimerDisposable::new(timer)));
    }
}
