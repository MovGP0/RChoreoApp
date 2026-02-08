use std::rc::Rc;
use std::time::Duration;

use crossbeam_channel::Receiver;
use slint::TimerMode;

use crate::behavior::{Behavior, CompositeDisposable, TimerDisposable};
use crate::global::GlobalStateActor;
use crate::logging::BehaviorLog;

use super::choreography_settings_view_model::ChoreographySettingsViewModel;
use super::mapper::{map_from_choreography, reset_view_model, update_selected_scene};
use super::messages::ReloadChoreographySettingsCommand;
use nject::injectable;

#[injectable]
pub struct LoadChoreographySettingsBehavior {
    global_state: Rc<GlobalStateActor>,
    receiver: Option<Receiver<ReloadChoreographySettingsCommand>>,
}

impl LoadChoreographySettingsBehavior {
    pub fn new(global_state: Rc<GlobalStateActor>) -> Self {
        Self {
            global_state,
            receiver: None,
        }
    }

    pub fn new_with_receiver(
        global_state: Rc<GlobalStateActor>,
        receiver: Receiver<ReloadChoreographySettingsCommand>,
    ) -> Self {
        Self {
            global_state,
            receiver: Some(receiver),
        }
    }

    fn load(&self, view_model: &mut ChoreographySettingsViewModel) {
        let Some(snapshot) = self.global_state.try_with_state(|global_state| {
            (
                global_state.choreography.clone(),
                global_state.selected_scene.clone(),
            )
        }) else {
            return;
        };
        let choreography = &snapshot.0;
        if choreography.name.is_empty()
            && choreography.scenes.is_empty()
            && choreography.comment.is_none()
        {
            reset_view_model(view_model);
            return;
        }

        map_from_choreography(choreography, view_model);
        update_selected_scene(view_model, &snapshot.1, choreography);
    }
}

impl Behavior<ChoreographySettingsViewModel> for LoadChoreographySettingsBehavior {
    fn activate(
        &self,
        view_model: &mut ChoreographySettingsViewModel,
        disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated(
            "LoadChoreographySettingsBehavior",
            "ChoreographySettingsViewModel",
        );
        self.load(view_model);
        let Some(receiver) = self.receiver.clone() else {
            return;
        };
        let Some(view_model_handle) = view_model.self_handle().and_then(|handle| handle.upgrade())
        else {
            return;
        };
        let behavior = Self {
            global_state: self.global_state.clone(),
            receiver: None,
        };
        let timer = slint::Timer::default();
        timer.start(TimerMode::Repeated, Duration::from_millis(16), move || {
            while receiver.try_recv().is_ok() {
                let mut view_model = view_model_handle.borrow_mut();
                behavior.load(&mut view_model);
                view_model.notify_changed();
            }
        });
        disposables.add(Box::new(TimerDisposable::new(timer)));
    }
}
