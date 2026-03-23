#[cfg(not(target_arch = "wasm32"))]
use std::path::Path;
use std::rc::Rc;
use std::sync::mpsc::SyncSender;

use nject::injectable;

use crate::behavior::Behavior;
use crate::behavior::CompositeDisposable;
use crate::floor::DrawFloorCommand;
use crate::global::GlobalStateActor;
use crate::logging::BehaviorLog;
use crate::preferences::Preferences;

use super::actions::ChoreoMainAction;
use super::main_view_model::MainViewModel;
use super::messages::OpenSvgFileCommand;

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

    pub fn apply(&self, view_model: &mut MainViewModel, command: OpenSvgFileCommand) {
        let path = command.file_path.trim().to_string();
        if path.is_empty() {
            view_model.dispatch(ChoreoMainAction::ApplyOpenSvgFile(OpenSvgFileCommand {
                file_path: String::new(),
            }));
            self.global_state_store.dispatch(|state| {
                state.svg_file_path = None;
            });
            self.preferences
                .remove(choreo_models::SettingsPreferenceKeys::LAST_OPENED_SVG_FILE);
            let _ = self.draw_floor_sender.try_send(DrawFloorCommand);
            return;
        }

        view_model.dispatch(ChoreoMainAction::ApplyOpenSvgFile(OpenSvgFileCommand {
            file_path: path.clone(),
        }));
        self.global_state_store.dispatch({
            let path = path.clone();
            move |state| {
                state.svg_file_path = Some(path);
            }
        });
        self.preferences.set_string(
            choreo_models::SettingsPreferenceKeys::LAST_OPENED_SVG_FILE,
            path,
        );
        let _ = self.draw_floor_sender.try_send(DrawFloorCommand);
    }

    fn restore_last_opened_svg(&self, view_model: &mut MainViewModel) {
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
            view_model.dispatch(ChoreoMainAction::RestoreLastOpenedSvg {
                file_path: Some(normalized),
                path_exists: false,
            });
            return;
        }

        view_model.dispatch(ChoreoMainAction::RestoreLastOpenedSvg {
            file_path: Some(normalized.clone()),
            path_exists: true,
        });
        self.global_state_store.dispatch({
            let normalized = normalized.clone();
            move |state| {
                state.svg_file_path = Some(normalized);
            }
        });
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

impl Behavior<MainViewModel> for OpenSvgFileBehavior {
    fn activate(&self, view_model: &mut MainViewModel, _disposables: &mut CompositeDisposable) {
        BehaviorLog::behavior_activated("OpenSvgFileBehavior", "MainViewModel");
        self.restore_last_opened_svg(view_model);
    }
}
