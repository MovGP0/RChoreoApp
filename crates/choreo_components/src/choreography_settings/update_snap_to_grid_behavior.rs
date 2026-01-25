use std::cell::RefCell;
use std::rc::Rc;

use crossbeam_channel::Sender;

use crate::behavior::{Behavior, CompositeDisposable};
use crate::global::GlobalStateModel;
use crate::logging::BehaviorLog;
use crate::preferences::Preferences;

use super::choreography_settings_view_model::ChoreographySettingsViewModel;
use super::messages::RedrawFloorCommand;

pub struct UpdateSnapToGridBehavior<P: Preferences> {
    global_state: Rc<RefCell<GlobalStateModel>>,
    preferences: P,
    redraw_sender: Sender<RedrawFloorCommand>,
}

impl<P: Preferences> UpdateSnapToGridBehavior<P> {
    pub fn new(
        global_state: Rc<RefCell<GlobalStateModel>>,
        preferences: P,
        redraw_sender: Sender<RedrawFloorCommand>,
    ) -> Self {
        Self {
            global_state,
            preferences,
            redraw_sender,
        }
    }

    pub fn initialize(&self, view_model: &mut ChoreographySettingsViewModel) {
        let snap_to_grid = self
            .preferences
            .get_bool(choreo_models::SettingsPreferenceKeys::SNAP_TO_GRID, true);
        view_model.snap_to_grid = snap_to_grid;
        self.global_state
            .borrow_mut()
            .choreography
            .settings
            .snap_to_grid = snap_to_grid;
    }

    pub fn update_snap_to_grid(&self, view_model: &mut ChoreographySettingsViewModel, value: bool) {
        view_model.snap_to_grid = value;
        {
            let mut global_state = self.global_state.borrow_mut();
            global_state.choreography.settings.snap_to_grid = value;
        }
        self.preferences
            .set_bool(choreo_models::SettingsPreferenceKeys::SNAP_TO_GRID, value);
        let _ = self.redraw_sender.send(RedrawFloorCommand);
    }
}

impl<P: Preferences> Behavior<ChoreographySettingsViewModel> for UpdateSnapToGridBehavior<P> {
    fn activate(
        &self,
        view_model: &mut ChoreographySettingsViewModel,
        _disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated(
            "UpdateSnapToGridBehavior",
            "ChoreographySettingsViewModel",
        );
        self.initialize(view_model);
    }
}


