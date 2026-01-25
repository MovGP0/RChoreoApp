use crossbeam_channel::Sender;

use crate::behavior::{Behavior, CompositeDisposable};
use crate::logging::BehaviorLog;
use crate::preferences::Preferences;

use super::choreography_settings_view_model::ChoreographySettingsViewModel;
use super::messages::RedrawFloorCommand;
use nject::injectable;

#[injectable]
pub struct UpdateShowLegendBehavior<P: Preferences> {
    preferences: P,
    redraw_sender: Sender<RedrawFloorCommand>,
}

impl<P: Preferences> UpdateShowLegendBehavior<P> {
    pub fn new(preferences: P, redraw_sender: Sender<RedrawFloorCommand>) -> Self {
        Self {
            preferences,
            redraw_sender,
        }
    }

    pub fn initialize(&self, view_model: &mut ChoreographySettingsViewModel) {
        view_model.show_legend = self
            .preferences
            .get_bool(choreo_models::SettingsPreferenceKeys::SHOW_LEGEND, false);
    }

    pub fn update_show_legend(&self, view_model: &mut ChoreographySettingsViewModel, value: bool) {
        view_model.show_legend = value;
        self.preferences
            .set_bool(choreo_models::SettingsPreferenceKeys::SHOW_LEGEND, value);
        let _ = self.redraw_sender.send(RedrawFloorCommand);
    }
}

impl<P: Preferences> Behavior<ChoreographySettingsViewModel> for UpdateShowLegendBehavior<P> {
    fn activate(
        &self,
        view_model: &mut ChoreographySettingsViewModel,
        _disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated(
            "UpdateShowLegendBehavior",
            "ChoreographySettingsViewModel",
        );
        self.initialize(view_model);
    }
}


