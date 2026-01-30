use std::cell::RefCell;
use std::rc::Rc;

use choreo_master_mobile_json::Color;
use crossbeam_channel::Sender;

use crate::behavior::{Behavior, CompositeDisposable};
use crate::global::GlobalStateModel;
use crate::logging::BehaviorLog;

use super::choreography_settings_view_model::ChoreographySettingsViewModel;
use super::messages::RedrawFloorCommand;
use nject::injectable;

#[injectable]
pub struct UpdateFloorColorBehavior {
    global_state: Rc<RefCell<GlobalStateModel>>,
    redraw_sender: Sender<RedrawFloorCommand>,
}

impl UpdateFloorColorBehavior {
    pub fn new(
        global_state: Rc<RefCell<GlobalStateModel>>,
        redraw_sender: Sender<RedrawFloorCommand>,
    ) -> Self {
        Self {
            global_state,
            redraw_sender,
        }
    }

    pub fn update_floor_color(&self, color: Color) {
        let mut global_state = self.global_state.borrow_mut();
        global_state.choreography.settings.floor_color = color;
        let _ = self.redraw_sender.send(RedrawFloorCommand);
    }
}

impl Behavior<ChoreographySettingsViewModel> for UpdateFloorColorBehavior {
    fn initialize(
        &self,
        _view_model: &mut ChoreographySettingsViewModel,
        _disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated(
            "UpdateFloorColorBehavior",
            "ChoreographySettingsViewModel",
        );
    }
}


