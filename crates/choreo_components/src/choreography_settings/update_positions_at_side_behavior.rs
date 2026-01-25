use std::cell::RefCell;
use std::rc::Rc;

use crossbeam_channel::Sender;

use crate::behavior::{Behavior, CompositeDisposable};
use crate::global::GlobalStateModel;
use crate::logging::BehaviorLog;
use crate::preferences::Preferences;

use super::choreography_settings_view_model::ChoreographySettingsViewModel;
use super::messages::RedrawFloorCommand;
use nject::injectable;

#[injectable]
pub struct UpdatePositionsAtSideBehavior<P: Preferences> {
    global_state: Rc<RefCell<GlobalStateModel>>,
    preferences: P,
    redraw_sender: Sender<RedrawFloorCommand>,
}

impl<P: Preferences> UpdatePositionsAtSideBehavior<P> {
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

    pub fn initialize(&self) {
        let mut global_state = self.global_state.borrow_mut();
        global_state.choreography.settings.positions_at_side = self
            .preferences
            .get_bool(choreo_models::SettingsPreferenceKeys::POSITIONS_AT_SIDE, true);
    }

    pub fn update_positions_at_side(&self, value: bool) {
        let mut global_state = self.global_state.borrow_mut();
        global_state.choreography.settings.positions_at_side = value;
        let _ = self.redraw_sender.send(RedrawFloorCommand);
    }
}

impl<P: Preferences> Behavior<ChoreographySettingsViewModel> for UpdatePositionsAtSideBehavior<P> {
    fn activate(
        &self,
        _view_model: &mut ChoreographySettingsViewModel,
        _disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated(
            "UpdatePositionsAtSideBehavior",
            "ChoreographySettingsViewModel",
        );
        self.initialize();
    }
}


