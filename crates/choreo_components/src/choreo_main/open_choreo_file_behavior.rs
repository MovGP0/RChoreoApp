#[cfg(not(target_arch = "wasm32"))]
use std::fs;
#[cfg(not(target_arch = "wasm32"))]
use std::path::Path;
use std::rc::Rc;

use nject::injectable;

use crate::preferences::Preferences;

use super::actions::OpenChoreoRequested;
use super::state::ChoreoMainState;

const DEFAULT_CHOREO_FILE_NAME: &str = "default.choreo";
const DEFAULT_CHOREO_CONTENTS: &str = include_str!("../../assets/default.choreo");

#[injectable]
#[inject(|preferences: Rc<dyn Preferences>| Self::new(preferences))]
#[derive(Clone)]
pub struct OpenChoreoFileBehavior {
    preferences: Rc<dyn Preferences>,
}

impl OpenChoreoFileBehavior {
    #[must_use]
    pub fn new(preferences: Rc<dyn Preferences>) -> Self {
        Self { preferences }
    }

    pub fn initialize(&self, state: &mut ChoreoMainState) {
        if !state.scenes.is_empty() || !state.outgoing_open_choreo_requests.is_empty() {
            return;
        }

        let remembered_path = self.preferences.get_string(
            choreo_models::SettingsPreferenceKeys::LAST_OPENED_CHOREO_FILE,
            "",
        );
        if let Some(request) = self.restore_last_opened_choreo_request(remembered_path.as_str()) {
            request_open_choreo(state, request);
            return;
        }

        request_open_choreo(state, bundled_default_choreo_request());
    }

    pub fn sync_last_opened_choreo_preference(&self, request: &OpenChoreoRequested) {
        let Some(file_path) = request.file_path.as_ref().map(|value| value.trim()) else {
            self.preferences
                .remove(choreo_models::SettingsPreferenceKeys::LAST_OPENED_CHOREO_FILE);
            return;
        };

        if file_path.is_empty() {
            self.preferences
                .remove(choreo_models::SettingsPreferenceKeys::LAST_OPENED_CHOREO_FILE);
            return;
        }

        self.preferences.set_string(
            choreo_models::SettingsPreferenceKeys::LAST_OPENED_CHOREO_FILE,
            file_path.to_string(),
        );
    }

    fn restore_last_opened_choreo_request(&self, path: &str) -> Option<OpenChoreoRequested> {
        let normalized = path.trim();
        if normalized.is_empty() {
            return None;
        }

        if !Self::path_exists(normalized) {
            self.preferences
                .remove(choreo_models::SettingsPreferenceKeys::LAST_OPENED_CHOREO_FILE);
            return None;
        }

        let Some(request) = load_existing_choreo_request(normalized) else {
            self.preferences
                .remove(choreo_models::SettingsPreferenceKeys::LAST_OPENED_CHOREO_FILE);
            return None;
        };

        Some(request)
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn path_exists(path: &str) -> bool {
        Path::new(path).is_file()
    }

    #[cfg(target_arch = "wasm32")]
    fn path_exists(_path: &str) -> bool {
        false
    }
}

pub(super) fn request_open_choreo(state: &mut ChoreoMainState, request: OpenChoreoRequested) {
    state.outgoing_open_choreo_requests.push(request);
}

#[cfg(not(target_arch = "wasm32"))]
fn load_existing_choreo_request(path: &str) -> Option<OpenChoreoRequested> {
    let contents = fs::read_to_string(path).ok()?;
    let file_name = Path::new(path)
        .file_name()
        .map(|value| value.to_string_lossy().into_owned());
    Some(OpenChoreoRequested {
        file_path: Some(path.to_string()),
        file_name,
        contents,
    })
}

#[cfg(target_arch = "wasm32")]
fn load_existing_choreo_request(_path: &str) -> Option<OpenChoreoRequested> {
    None
}

fn bundled_default_choreo_request() -> OpenChoreoRequested {
    OpenChoreoRequested {
        file_path: None,
        file_name: Some(DEFAULT_CHOREO_FILE_NAME.to_string()),
        contents: DEFAULT_CHOREO_CONTENTS.to_string(),
    }
}
