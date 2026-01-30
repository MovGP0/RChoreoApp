use std::cell::RefCell;
use std::rc::Rc;

use crossbeam_channel::Sender;

use crate::behavior::{Behavior, CompositeDisposable};
use crate::global::GlobalStateModel;
use crate::logging::BehaviorLog;

use super::mapper::normalize_text;
use super::choreography_settings_view_model::ChoreographySettingsViewModel;
use super::messages::RedrawFloorCommand;
use nject::injectable;

#[injectable]
pub struct UpdateDescriptionBehavior {
    global_state: Rc<RefCell<GlobalStateModel>>,
    redraw_sender: Sender<RedrawFloorCommand>,
}

impl UpdateDescriptionBehavior {
    pub fn new(
        global_state: Rc<RefCell<GlobalStateModel>>,
        redraw_sender: Sender<RedrawFloorCommand>,
    ) -> Self {
        Self {
            global_state,
            redraw_sender,
        }
    }

    pub fn update_description(&self, value: &str) {
        let mut global_state = self.global_state.borrow_mut();
        global_state.choreography.description = normalize_text(value);
        let _ = self.redraw_sender.send(RedrawFloorCommand);
    }
}

impl Behavior<ChoreographySettingsViewModel> for UpdateDescriptionBehavior {
    fn activate(
        &self,
        _view_model: &mut ChoreographySettingsViewModel,
        _disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated(
            "UpdateDescriptionBehavior",
            "ChoreographySettingsViewModel",
        );
    }
}
