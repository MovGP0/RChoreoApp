use crate::preferences::Preferences;
use nject::injectable;

#[injectable]
#[inject(|preferences: P| Self::new(preferences))]
pub struct OpenChoreoBehavior<P: Preferences> {
    preferences: P,
}

impl<P: Preferences> OpenChoreoBehavior<P> {
    pub fn new(preferences: P) -> Self {
        Self { preferences }
    }

    pub fn set_last_opened(&self, path: &str) {
        self.preferences.set_string(
            choreo_models::SettingsPreferenceKeys::LAST_OPENED_CHOREO_FILE,
            path.to_string(),
        );
    }
}
