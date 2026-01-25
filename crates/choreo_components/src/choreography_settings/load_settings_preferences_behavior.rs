use choreo_models::SettingsModel;

use crate::behavior::{Behavior, CompositeDisposable};
use crate::logging::BehaviorLog;
use crate::preferences::Preferences;

pub struct LoadSettingsPreferencesBehavior<P: Preferences> {
    preferences: P,
}

impl<P: Preferences> LoadSettingsPreferencesBehavior<P> {
    pub fn new(preferences: P) -> Self {
        Self { preferences }
    }

    pub fn load(&self, settings: &mut SettingsModel) {
        settings.show_timestamps = self
            .preferences
            .get_bool(choreo_models::SettingsPreferenceKeys::SHOW_TIMESTAMPS, true);
        settings.positions_at_side = self
            .preferences
            .get_bool(choreo_models::SettingsPreferenceKeys::POSITIONS_AT_SIDE, true);
        settings.snap_to_grid = self
            .preferences
            .get_bool(choreo_models::SettingsPreferenceKeys::SNAP_TO_GRID, true);
    }
}

impl<P: Preferences> Behavior<SettingsModel> for LoadSettingsPreferencesBehavior<P> {
    fn activate(&self, settings: &mut SettingsModel, _disposables: &mut CompositeDisposable) {
        BehaviorLog::behavior_activated(
            "LoadSettingsPreferencesBehavior",
            "SettingsModel",
        );
        self.load(settings);
    }
}


