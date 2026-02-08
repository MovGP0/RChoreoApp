#[cfg(not(target_arch = "wasm32"))]
use std::path::Path;
use std::rc::Rc;
use std::time::Duration;

use crossbeam_channel::{Receiver, Sender};
use nject::injectable;
use slint::TimerMode;

use crate::behavior::TimerDisposable;
use crate::behavior::{Behavior, CompositeDisposable};
use crate::floor::DrawFloorCommand;
use crate::global::GlobalStateActor;
use crate::logging::BehaviorLog;
use crate::preferences::Preferences;

use super::main_view_model::MainViewModel;
use super::messages::OpenSvgFileCommand;

#[injectable]
#[inject(
    |global_state: Rc<GlobalStateActor>,
     preferences: Rc<dyn Preferences>,
     receiver: Receiver<OpenSvgFileCommand>,
     draw_floor_sender: Sender<DrawFloorCommand>| {
        Self::new(global_state, preferences, receiver, draw_floor_sender)
    }
)]
pub struct OpenSvgFileBehavior {
    global_state: Rc<GlobalStateActor>,
    preferences: Rc<dyn Preferences>,
    receiver: Receiver<OpenSvgFileCommand>,
    draw_floor_sender: Sender<DrawFloorCommand>,
}

impl OpenSvgFileBehavior {
    pub fn new(
        global_state: Rc<GlobalStateActor>,
        preferences: Rc<dyn Preferences>,
        receiver: Receiver<OpenSvgFileCommand>,
        draw_floor_sender: Sender<DrawFloorCommand>,
    ) -> Self {
        Self {
            global_state,
            preferences,
            receiver,
            draw_floor_sender,
        }
    }

    fn restore_last_opened_svg(&self) {
        let path = self.preferences.get_string(
            choreo_models::SettingsPreferenceKeys::LAST_OPENED_SVG_FILE,
            "",
        );

        if path.trim().is_empty() {
            return;
        }

        if !Self::path_exists(path.as_str()) {
            self.preferences
                .remove(choreo_models::SettingsPreferenceKeys::LAST_OPENED_SVG_FILE);
            return;
        }

        let updated = self.global_state.try_update(|global_state| {
            global_state.svg_file_path = Some(path.clone());
        });
        if !updated {
            return;
        }

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
    fn activate(&self, _view_model: &mut MainViewModel, disposables: &mut CompositeDisposable) {
        BehaviorLog::behavior_activated("OpenSvgFileBehavior", "MainViewModel");
        self.restore_last_opened_svg();
        let receiver = self.receiver.clone();
        let global_state = Rc::clone(&self.global_state);
        let preferences = Rc::clone(&self.preferences);
        let draw_floor_sender = self.draw_floor_sender.clone();
        let timer = slint::Timer::default();
        timer.start(TimerMode::Repeated, Duration::from_millis(16), move || {
            while let Ok(command) = receiver.try_recv() {
                let path = command.file_path.trim().to_string();

                if path.is_empty() {
                    let updated = global_state.try_update(|global_state| {
                        global_state.svg_file_path = None;
                    });
                    if !updated {
                        return;
                    }

                    preferences.remove(choreo_models::SettingsPreferenceKeys::LAST_OPENED_SVG_FILE);
                    let _ = draw_floor_sender.try_send(DrawFloorCommand);
                    continue;
                }

                let updated = global_state.try_update(|global_state| {
                    global_state.svg_file_path = Some(path.clone());
                });
                if !updated {
                    return;
                }

                preferences.set_string(
                    choreo_models::SettingsPreferenceKeys::LAST_OPENED_SVG_FILE,
                    path,
                );
                let _ = draw_floor_sender.try_send(DrawFloorCommand);
            }
        });
        disposables.add(Box::new(TimerDisposable::new(timer)));
    }
}
