use std::cell::RefCell;
use std::rc::Rc;

use crossbeam_channel::Sender;

use crate::behavior::{Behavior, CompositeDisposable};
use crate::global::GlobalStateModel;
use crate::logging::BehaviorLog;
use crate::preferences::Preferences;

use super::choreography_settings_view_model::ChoreographySettingsViewModel;
use super::messages::{RedrawFloorCommand, ShowTimestampsChangedEvent};
use nject::injectable;

#[injectable]
pub struct UpdateShowTimestampsBehavior<P: Preferences> {
    global_state: Rc<RefCell<GlobalStateModel>>,
    preferences: P,
    redraw_sender: Sender<RedrawFloorCommand>,
    show_timestamps_sender: Sender<ShowTimestampsChangedEvent>,
}

impl<P: Preferences> UpdateShowTimestampsBehavior<P> {
    pub fn new(
        global_state: Rc<RefCell<GlobalStateModel>>,
        preferences: P,
        redraw_sender: Sender<RedrawFloorCommand>,
        show_timestamps_sender: Sender<ShowTimestampsChangedEvent>,
    ) -> Self {
        Self {
            global_state,
            preferences,
            redraw_sender,
            show_timestamps_sender,
        }
    }

    pub fn initialize(&self, view_model: &mut ChoreographySettingsViewModel) {
        let value = self
            .preferences
            .get_bool(choreo_models::SettingsPreferenceKeys::SHOW_TIMESTAMPS, true);
        view_model.show_timestamps = value;
        self.global_state
            .borrow_mut()
            .choreography
            .settings
            .show_timestamps = value;
    }

    pub fn update_show_timestamps(&self, view_model: &mut ChoreographySettingsViewModel, value: bool) {
        view_model.show_timestamps = value;
        {
            let mut global_state = self.global_state.borrow_mut();
            global_state.choreography.settings.show_timestamps = value;
        }
        let _ = self.redraw_sender.send(RedrawFloorCommand);
        let _ = self
            .show_timestamps_sender
            .send(ShowTimestampsChangedEvent { is_enabled: value });
    }
}

impl<P: Preferences> Behavior<ChoreographySettingsViewModel> for UpdateShowTimestampsBehavior<P> {
    fn activate(
        &self,
        view_model: &mut ChoreographySettingsViewModel,
        _disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated(
            "UpdateShowTimestampsBehavior",
            "ChoreographySettingsViewModel",
        );
        self.initialize(view_model);
    }
}



