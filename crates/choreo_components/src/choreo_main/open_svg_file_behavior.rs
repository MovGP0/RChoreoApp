use std::cell::RefCell;
use std::rc::Rc;

use crossbeam_channel::Receiver;
use nject::injectable;

use crate::behavior::{Behavior, CompositeDisposable};
use crate::global::GlobalStateModel;
use crate::logging::BehaviorLog;
use crate::preferences::Preferences;

use super::main_view_model::MainViewModel;
use super::messages::OpenSvgFileCommand;

#[injectable]
#[inject(
    |global_state: Rc<RefCell<GlobalStateModel>>, preferences: Rc<dyn Preferences>, receiver: Receiver<OpenSvgFileCommand>| {
        Self::new(global_state, preferences, receiver)
    }
)]
pub struct OpenSvgFileBehavior {
    global_state: Rc<RefCell<GlobalStateModel>>,
    preferences: Rc<dyn Preferences>,
    receiver: Receiver<OpenSvgFileCommand>,
}

impl OpenSvgFileBehavior {
    pub fn new(
        global_state: Rc<RefCell<GlobalStateModel>>,
        preferences: Rc<dyn Preferences>,
        receiver: Receiver<OpenSvgFileCommand>,
    ) -> Self {
        Self {
            global_state,
            preferences,
            receiver,
        }
    }

    pub fn try_handle(&self) -> bool {
        match self.receiver.try_recv() {
            Ok(command) => {
                let mut global_state = self.global_state.borrow_mut();
                global_state.svg_file_path = Some(command.file_path.clone());
                self.preferences.set_string(
                    choreo_models::SettingsPreferenceKeys::LAST_OPENED_SVG_FILE,
                    command.file_path,
                );
                true
            }
            Err(_) => false,
        }
    }
}

impl Behavior<MainViewModel> for OpenSvgFileBehavior {
    fn activate(&self, _view_model: &mut MainViewModel, _disposables: &mut CompositeDisposable) {
        BehaviorLog::behavior_activated("OpenSvgFileBehavior", "MainViewModel");
    }
}
