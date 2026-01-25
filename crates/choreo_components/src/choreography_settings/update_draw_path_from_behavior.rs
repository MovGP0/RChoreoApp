use crossbeam_channel::Sender;

use crate::behavior::{Behavior, CompositeDisposable};
use crate::logging::BehaviorLog;
use crate::preferences::Preferences;

use super::choreography_settings_view_model::ChoreographySettingsViewModel;
use super::messages::RedrawFloorCommand;

pub struct UpdateDrawPathFromBehavior<P: Preferences> {
    preferences: P,
    redraw_sender: Sender<RedrawFloorCommand>,
}

impl<P: Preferences> UpdateDrawPathFromBehavior<P> {
    pub fn new(preferences: P, redraw_sender: Sender<RedrawFloorCommand>) -> Self {
        Self {
            preferences,
            redraw_sender,
        }
    }

    pub fn initialize(&self, view_model: &mut ChoreographySettingsViewModel) {
        view_model.draw_path_from = self
            .preferences
            .get_bool(choreo_models::SettingsPreferenceKeys::DRAW_PATH_FROM, false);
    }

    pub fn update_draw_path_from(&self, view_model: &mut ChoreographySettingsViewModel, value: bool) {
        view_model.draw_path_from = value;
        self.preferences
            .set_bool(choreo_models::SettingsPreferenceKeys::DRAW_PATH_FROM, value);
        let _ = self.redraw_sender.send(RedrawFloorCommand);
    }
}

impl<P: Preferences> Behavior<ChoreographySettingsViewModel> for UpdateDrawPathFromBehavior<P> {
    fn activate(
        &self,
        view_model: &mut ChoreographySettingsViewModel,
        _disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated(
            "UpdateDrawPathFromBehavior",
            "ChoreographySettingsViewModel",
        );
        self.initialize(view_model);
    }
}


