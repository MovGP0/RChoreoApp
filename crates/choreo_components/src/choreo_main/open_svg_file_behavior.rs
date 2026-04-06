#[cfg(not(target_arch = "wasm32"))]
use std::path::Path;
use std::rc::Rc;
use std::sync::mpsc::SyncSender;

use nject::injectable;

use crate::floor::DrawFloorCommand;
use crate::global::GlobalStateActor;
use crate::preferences::Preferences;

use super::messages::OpenSvgFileCommand;
use super::state::ChoreoMainState;

#[injectable]
#[inject(
    |global_state_store: Rc<GlobalStateActor>,
     preferences: Rc<dyn Preferences>,
     draw_floor_sender: SyncSender<DrawFloorCommand>| {
        Self::new(global_state_store, preferences, draw_floor_sender)
    }
)]
#[derive(Clone)]
pub struct OpenSvgFileBehavior {
    global_state_store: Rc<GlobalStateActor>,
    preferences: Rc<dyn Preferences>,
    draw_floor_sender: SyncSender<DrawFloorCommand>,
}

impl OpenSvgFileBehavior {
    pub fn new(
        global_state_store: Rc<GlobalStateActor>,
        preferences: Rc<dyn Preferences>,
        draw_floor_sender: SyncSender<DrawFloorCommand>,
    ) -> Self {
        Self {
            global_state_store,
            preferences,
            draw_floor_sender,
        }
    }

    pub fn initialize(&self, state: &mut ChoreoMainState) {
        let path = self.preferences.get_string(
            choreo_models::SettingsPreferenceKeys::LAST_OPENED_SVG_FILE,
            "",
        );
        let normalized = path.trim().to_string();
        if normalized.is_empty() {
            return;
        }

        if !Self::path_exists(normalized.as_str()) {
            self.preferences
                .remove(choreo_models::SettingsPreferenceKeys::LAST_OPENED_SVG_FILE);
            restore_last_opened_svg(state, Some(normalized), false);
            return;
        }

        restore_last_opened_svg(state, Some(normalized.clone()), true);
        self.global_state_store.dispatch({
            let normalized = normalized.clone();
            move |global_state| {
                global_state.svg_file_path = Some(normalized);
            }
        });
        let _ = self.draw_floor_sender.try_send(DrawFloorCommand);
    }

    pub fn apply_to_state(&self, state: &mut ChoreoMainState, command: OpenSvgFileCommand) {
        apply_open_svg_file(state, command.clone());
        self.sync_external_state(command.file_path.as_str());
    }

    pub fn sync_external_state(&self, file_path: &str) {
        let path = file_path.trim().to_string();
        if path.is_empty() {
            self.global_state_store.dispatch(|global_state| {
                global_state.svg_file_path = None;
            });
            self.preferences
                .remove(choreo_models::SettingsPreferenceKeys::LAST_OPENED_SVG_FILE);
            let _ = self.draw_floor_sender.try_send(DrawFloorCommand);
            return;
        }

        self.global_state_store.dispatch({
            let path = path.clone();
            move |global_state| {
                global_state.svg_file_path = Some(path);
            }
        });
        self.preferences.set_string(
            choreo_models::SettingsPreferenceKeys::LAST_OPENED_SVG_FILE,
            path,
        );
        let _ = self.draw_floor_sender.try_send(DrawFloorCommand);
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn path_exists(path: &str) -> bool {
        Path::new(path).exists()
    }

    #[cfg(target_arch = "wasm32")]
    fn path_exists(_path: &str) -> bool {
        false
    }
}

pub(super) fn apply_open_svg_file(state: &mut ChoreoMainState, command: OpenSvgFileCommand) {
    let path = command.file_path.trim().to_string();
    if path.is_empty() {
        state.svg_file_path = None;
        state.last_opened_svg_preference = None;
    } else {
        state.svg_file_path = Some(path.clone());
        state.last_opened_svg_preference = Some(path);
    }

    super::reducer::refresh_floor_projection(state);
    state.draw_floor_request_count += 1;
}

pub(super) fn restore_last_opened_svg(
    state: &mut ChoreoMainState,
    file_path: Option<String>,
    path_exists: bool,
) {
    let Some(path) = file_path.map(|value| value.trim().to_string()) else {
        return;
    };
    if path.is_empty() {
        return;
    }

    if !path_exists {
        state.last_opened_svg_preference = None;
        return;
    }

    state.svg_file_path = Some(path.clone());
    state.last_opened_svg_preference = Some(path);
    super::reducer::refresh_floor_projection(state);
    state.draw_floor_request_count += 1;
}
