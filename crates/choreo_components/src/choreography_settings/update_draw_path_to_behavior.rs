use crossbeam_channel::Sender;

use crate::behavior::{Behavior, CompositeDisposable};
use crate::logging::BehaviorLog;
use crate::preferences::Preferences;

use super::choreography_settings_view_model::ChoreographySettingsViewModel;
use super::messages::RedrawFloorCommand;

pub struct UpdateDrawPathToBehavior<P: Preferences> {
    preferences: P,
    redraw_sender: Sender<RedrawFloorCommand>,
}

impl<P: Preferences> UpdateDrawPathToBehavior<P> {
    pub fn new(preferences: P, redraw_sender: Sender<RedrawFloorCommand>) -> Self {
        Self {
            preferences,
            redraw_sender,
        }
    }

    pub fn initialize(&self, view_model: &mut ChoreographySettingsViewModel) {
        view_model.draw_path_to = self
            .preferences
            .get_bool(choreo_models::SettingsPreferenceKeys::DRAW_PATH_TO, false);
    }

    pub fn update_draw_path_to(&self, view_model: &mut ChoreographySettingsViewModel, value: bool) {
        view_model.draw_path_to = value;
        self.preferences
            .set_bool(choreo_models::SettingsPreferenceKeys::DRAW_PATH_TO, value);
        let _ = self.redraw_sender.send(RedrawFloorCommand);
    }
}

impl<P: Preferences> Behavior<ChoreographySettingsViewModel> for UpdateDrawPathToBehavior<P> {
    fn activate(
        &self,
        view_model: &mut ChoreographySettingsViewModel,
        _disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated(
            "UpdateDrawPathToBehavior",
            "ChoreographySettingsViewModel",
        );
        self.initialize(view_model);
    }
}


