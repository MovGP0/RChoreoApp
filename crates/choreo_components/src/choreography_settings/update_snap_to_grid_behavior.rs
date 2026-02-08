use std::rc::Rc;
use std::time::Duration;

use crossbeam_channel::{Receiver, Sender};
use slint::TimerMode;

use crate::behavior::{Behavior, CompositeDisposable, TimerDisposable};
use crate::global::GlobalStateActor;
use crate::logging::BehaviorLog;
use crate::preferences::Preferences;

use super::choreography_settings_view_model::ChoreographySettingsViewModel;
use super::messages::{RedrawFloorCommand, UpdateSnapToGridCommand};
use nject::injectable;

#[injectable]
pub struct UpdateSnapToGridBehavior<P: Preferences + Clone> {
    global_state: Rc<GlobalStateActor>,
    preferences: P,
    redraw_sender: Sender<RedrawFloorCommand>,
    receiver: Option<Receiver<UpdateSnapToGridCommand>>,
}

impl<P: Preferences + Clone> UpdateSnapToGridBehavior<P> {
    pub fn new(
        global_state: Rc<GlobalStateActor>,
        preferences: P,
        redraw_sender: Sender<RedrawFloorCommand>,
    ) -> Self {
        Self {
            global_state,
            preferences,
            redraw_sender,
            receiver: None,
        }
    }

    pub fn new_with_receiver(
        global_state: Rc<GlobalStateActor>,
        preferences: P,
        redraw_sender: Sender<RedrawFloorCommand>,
        receiver: Receiver<UpdateSnapToGridCommand>,
    ) -> Self {
        Self {
            global_state,
            preferences,
            redraw_sender,
            receiver: Some(receiver),
        }
    }

    fn initialize(&self, view_model: &mut ChoreographySettingsViewModel) {
        let snap_to_grid = self
            .preferences
            .get_bool(choreo_models::SettingsPreferenceKeys::SNAP_TO_GRID, true);
        view_model.snap_to_grid = snap_to_grid;
        self.global_state.try_update(|global_state| {
            global_state.choreography.settings.snap_to_grid = snap_to_grid;
        });
    }

    fn update_snap_to_grid(&self, view_model: &mut ChoreographySettingsViewModel, value: bool) {
        view_model.snap_to_grid = value;
        let updated = self.global_state.try_update(|global_state| {
            global_state.choreography.settings.snap_to_grid = value;
        });
        if !updated {
            return;
        }
        self.preferences
            .set_bool(choreo_models::SettingsPreferenceKeys::SNAP_TO_GRID, value);
        let _ = self.redraw_sender.send(RedrawFloorCommand);
    }
}

impl<P: Preferences + Clone + 'static> Behavior<ChoreographySettingsViewModel>
    for UpdateSnapToGridBehavior<P>
{
    fn activate(
        &self,
        view_model: &mut ChoreographySettingsViewModel,
        disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated(
            "UpdateSnapToGridBehavior",
            "ChoreographySettingsViewModel",
        );
        self.initialize(view_model);
        let Some(receiver) = self.receiver.clone() else {
            return;
        };
        let Some(view_model_handle) = view_model.self_handle().and_then(|handle| handle.upgrade())
        else {
            return;
        };
        let behavior = Self {
            global_state: self.global_state.clone(),
            preferences: self.preferences.clone(),
            redraw_sender: self.redraw_sender.clone(),
            receiver: None,
        };
        let timer = slint::Timer::default();
        timer.start(TimerMode::Repeated, Duration::from_millis(16), move || {
            while let Ok(command) = receiver.try_recv() {
                let mut view_model = view_model_handle.borrow_mut();
                behavior.update_snap_to_grid(&mut view_model, command.value);
                view_model.notify_changed();
            }
        });
        disposables.add(Box::new(TimerDisposable::new(timer)));
    }
}
