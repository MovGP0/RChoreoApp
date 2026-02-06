use std::rc::Rc;
use std::time::Duration;

use crossbeam_channel::{Receiver, Sender};
use slint::TimerMode;

use crate::behavior::{Behavior, CompositeDisposable, TimerDisposable};
use crate::global::GlobalStateActor;
use crate::logging::BehaviorLog;

use super::choreography_settings_view_model::ChoreographySettingsViewModel;
use super::messages::{RedrawFloorCommand, UpdateFloorRightCommand};
use nject::injectable;

#[injectable]
pub struct UpdateFloorRightBehavior {
    global_state: Rc<GlobalStateActor>,
    redraw_sender: Sender<RedrawFloorCommand>,
    receiver: Option<Receiver<UpdateFloorRightCommand>>,
}

impl UpdateFloorRightBehavior {
    pub fn new(
        global_state: Rc<GlobalStateActor>,
        redraw_sender: Sender<RedrawFloorCommand>,
    ) -> Self {
        Self {
            global_state,
            redraw_sender,
            receiver: None,
        }
    }

    pub fn new_with_receiver(
        global_state: Rc<GlobalStateActor>,
        redraw_sender: Sender<RedrawFloorCommand>,
        receiver: Receiver<UpdateFloorRightCommand>,
    ) -> Self {
        Self {
            global_state,
            redraw_sender,
            receiver: Some(receiver),
        }
    }

    fn update_floor_right(&self, value: i32) {
        let updated = self.global_state.try_update(|global_state| {
            global_state.choreography.floor.size_right = value.clamp(1, 100);
        });
        if !updated {
            return;
        }
        let _ = self.redraw_sender.send(RedrawFloorCommand);
    }
}

impl Behavior<ChoreographySettingsViewModel> for UpdateFloorRightBehavior {
    fn activate(
        &self,
        _view_model: &mut ChoreographySettingsViewModel,
        disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated(
            "UpdateFloorRightBehavior",
            "ChoreographySettingsViewModel",
        );
        let Some(receiver) = self.receiver.clone() else {
            return;
        };
        let behavior = Self {
            global_state: self.global_state.clone(),
            redraw_sender: self.redraw_sender.clone(),
            receiver: None,
        };
        let timer = slint::Timer::default();
        timer.start(TimerMode::Repeated, Duration::from_millis(16), move || {
            while let Ok(command) = receiver.try_recv() {
                behavior.update_floor_right(command.value);
            }
        });
        disposables.add(Box::new(TimerDisposable::new(timer)));
    }
}


