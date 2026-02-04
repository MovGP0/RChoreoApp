use std::rc::Rc;

use crossbeam_channel::Sender;

use crate::behavior::{Behavior, CompositeDisposable};
use crate::global::GlobalStateStore;
use crate::logging::BehaviorLog;

use super::choreography_settings_view_model::ChoreographySettingsViewModel;
use super::messages::RedrawFloorCommand;
use nject::injectable;

#[injectable]
pub struct UpdateFloorFrontBehavior {
    global_state: Rc<GlobalStateStore>,
    redraw_sender: Sender<RedrawFloorCommand>,
}

impl UpdateFloorFrontBehavior {
    pub fn new(
        global_state: Rc<GlobalStateStore>,
        redraw_sender: Sender<RedrawFloorCommand>,
    ) -> Self {
        Self {
            global_state,
            redraw_sender,
        }
    }

    pub fn update_floor_front(&self, value: i32) {
        let updated = self.global_state.try_update(|global_state| {
            global_state.choreography.floor.size_front = value.clamp(0, 100);
        });
        if !updated {
            return;
        }
        let _ = self.redraw_sender.send(RedrawFloorCommand);
    }
}

impl Behavior<ChoreographySettingsViewModel> for UpdateFloorFrontBehavior {
    fn activate(
        &self,
        _view_model: &mut ChoreographySettingsViewModel,
        _disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated(
            "UpdateFloorFrontBehavior",
            "ChoreographySettingsViewModel",
        );
    }
}


