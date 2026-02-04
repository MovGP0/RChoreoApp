use std::rc::Rc;

use crossbeam_channel::Sender;

use crate::behavior::{Behavior, CompositeDisposable};
use crate::global::GlobalStateStore;
use crate::logging::BehaviorLog;

use super::choreography_settings_view_model::ChoreographySettingsViewModel;
use super::messages::RedrawFloorCommand;
use nject::injectable;

#[injectable]
pub struct UpdateGridLinesBehavior {
    global_state: Rc<GlobalStateStore>,
    redraw_sender: Sender<RedrawFloorCommand>,
}

impl UpdateGridLinesBehavior {
    pub fn new(
        global_state: Rc<GlobalStateStore>,
        redraw_sender: Sender<RedrawFloorCommand>,
    ) -> Self {
        Self {
            global_state,
            redraw_sender,
        }
    }

    pub fn update_grid_lines(&self, value: bool) {
        let updated = self.global_state.try_update(|global_state| {
            global_state.choreography.settings.grid_lines = value;
        });
        if !updated {
            return;
        }
        let _ = self.redraw_sender.send(RedrawFloorCommand);
    }
}

impl Behavior<ChoreographySettingsViewModel> for UpdateGridLinesBehavior {
    fn activate(
        &self,
        _view_model: &mut ChoreographySettingsViewModel,
        _disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated(
            "UpdateGridLinesBehavior",
            "ChoreographySettingsViewModel",
        );
    }
}


