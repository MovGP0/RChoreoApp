use std::rc::Rc;

use crossbeam_channel::Sender;

use crate::behavior::{Behavior, CompositeDisposable};
use crate::global::GlobalStateStore;
use crate::logging::BehaviorLog;

use super::choreography_settings_view_model::ChoreographySettingsViewModel;
use super::messages::RedrawFloorCommand;
use nject::injectable;

#[injectable]
pub struct UpdateGridResolutionBehavior {
    global_state: Rc<GlobalStateStore>,
    redraw_sender: Sender<RedrawFloorCommand>,
}

impl UpdateGridResolutionBehavior {
    pub fn new(
        global_state: Rc<GlobalStateStore>,
        redraw_sender: Sender<RedrawFloorCommand>,
    ) -> Self {
        Self {
            global_state,
            redraw_sender,
        }
    }

    pub fn update_grid_resolution(&self, value: i32) {
        let updated = self.global_state.try_update(|global_state| {
            global_state.choreography.settings.resolution = value.clamp(1, 16);
        });
        if !updated {
            return;
        }
        let _ = self.redraw_sender.send(RedrawFloorCommand);
    }
}

impl Behavior<ChoreographySettingsViewModel> for UpdateGridResolutionBehavior {
    fn activate(
        &self,
        _view_model: &mut ChoreographySettingsViewModel,
        _disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated(
            "UpdateGridResolutionBehavior",
            "ChoreographySettingsViewModel",
        );
    }
}


