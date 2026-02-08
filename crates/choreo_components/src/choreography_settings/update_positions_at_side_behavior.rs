use std::rc::Rc;
use std::time::Duration;

use crossbeam_channel::{Receiver, Sender};
use slint::TimerMode;

use crate::behavior::{Behavior, CompositeDisposable, TimerDisposable};
use crate::global::GlobalStateActor;
use crate::logging::BehaviorLog;
use crate::preferences::Preferences;

use super::choreography_settings_view_model::ChoreographySettingsViewModel;
use super::messages::{RedrawFloorCommand, UpdatePositionsAtSideCommand};
use nject::injectable;

#[injectable]
pub struct UpdatePositionsAtSideBehavior<P: Preferences + Clone> {
    global_state: Rc<GlobalStateActor>,
    preferences: P,
    redraw_sender: Sender<RedrawFloorCommand>,
    receiver: Option<Receiver<UpdatePositionsAtSideCommand>>,
}

impl<P: Preferences + Clone> UpdatePositionsAtSideBehavior<P> {
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
        receiver: Receiver<UpdatePositionsAtSideCommand>,
    ) -> Self {
        Self {
            global_state,
            preferences,
            redraw_sender,
            receiver: Some(receiver),
        }
    }

    fn initialize(&self, view_model: &mut ChoreographySettingsViewModel) {
        let value = self.preferences.get_bool(
            choreo_models::SettingsPreferenceKeys::POSITIONS_AT_SIDE,
            true,
        );
        view_model.positions_at_side = value;
        self.global_state.try_update(|global_state| {
            global_state.choreography.settings.positions_at_side = self.preferences.get_bool(
                choreo_models::SettingsPreferenceKeys::POSITIONS_AT_SIDE,
                true,
            );
        });
    }

    fn update_positions_at_side(&self, value: bool) {
        let updated = self.global_state.try_update(|global_state| {
            global_state.choreography.settings.positions_at_side = value;
        });
        if !updated {
            return;
        }
        let _ = self.redraw_sender.send(RedrawFloorCommand);
    }
}

impl<P: Preferences + Clone + 'static> Behavior<ChoreographySettingsViewModel>
    for UpdatePositionsAtSideBehavior<P>
{
    fn activate(
        &self,
        view_model: &mut ChoreographySettingsViewModel,
        disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated(
            "UpdatePositionsAtSideBehavior",
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
                behavior.update_positions_at_side(command.value);
                let mut view_model = view_model_handle.borrow_mut();
                view_model.positions_at_side = command.value;
                view_model.notify_changed();
            }
        });
        disposables.add(Box::new(TimerDisposable::new(timer)));
    }
}
