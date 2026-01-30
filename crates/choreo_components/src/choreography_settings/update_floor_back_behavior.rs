use std::cell::RefCell;
use std::rc::Rc;

use crossbeam_channel::Sender;

use crate::behavior::{Behavior, CompositeDisposable};
use crate::global::GlobalStateModel;
use crate::logging::BehaviorLog;

use super::choreography_settings_view_model::ChoreographySettingsViewModel;
use super::messages::RedrawFloorCommand;
use nject::injectable;

#[injectable]
pub struct UpdateFloorBackBehavior {
    global_state: Rc<RefCell<GlobalStateModel>>,
    redraw_sender: Sender<RedrawFloorCommand>,
}

impl UpdateFloorBackBehavior {
    pub fn new(
        global_state: Rc<RefCell<GlobalStateModel>>,
        redraw_sender: Sender<RedrawFloorCommand>,
    ) -> Self {
        Self {
            global_state,
            redraw_sender,
        }
    }

    pub fn update_floor_back(&self, value: i32) {
        let mut global_state = self.global_state.borrow_mut();
        global_state.choreography.floor.size_back = value.clamp(0, 100);
        let _ = self.redraw_sender.send(RedrawFloorCommand);
    }
}

impl Behavior<ChoreographySettingsViewModel> for UpdateFloorBackBehavior {
    fn initialize(
        &self,
        _view_model: &mut ChoreographySettingsViewModel,
        _disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated(
            "UpdateFloorBackBehavior",
            "ChoreographySettingsViewModel",
        );
    }
}


