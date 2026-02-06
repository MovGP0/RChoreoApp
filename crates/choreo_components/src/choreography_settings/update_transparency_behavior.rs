use std::rc::Rc;
use std::time::Duration;

use crossbeam_channel::{Receiver, Sender};
use slint::TimerMode;

use crate::behavior::{Behavior, CompositeDisposable, TimerDisposable};
use crate::global::GlobalStateActor;
use crate::logging::BehaviorLog;

use super::choreography_settings_view_model::ChoreographySettingsViewModel;
use super::messages::{RedrawFloorCommand, UpdateTransparencyCommand};
use nject::injectable;

#[injectable]
pub struct UpdateTransparencyBehavior {
    global_state: Rc<GlobalStateActor>,
    redraw_sender: Sender<RedrawFloorCommand>,
    receiver: Option<Receiver<UpdateTransparencyCommand>>,
}

impl UpdateTransparencyBehavior {
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
        receiver: Receiver<UpdateTransparencyCommand>,
    ) -> Self {
        Self {
            global_state,
            redraw_sender,
            receiver: Some(receiver),
        }
    }

    fn update_transparency(&self, value: f64) {
        let updated = self.global_state.try_update(|global_state| {
            global_state.choreography.settings.transparency = value.clamp(0.0, 1.0);
        });
        if !updated {
            return;
        }
        let _ = self.redraw_sender.send(RedrawFloorCommand);
    }
}

impl Behavior<ChoreographySettingsViewModel> for UpdateTransparencyBehavior {
    fn activate(
        &self,
        _view_model: &mut ChoreographySettingsViewModel,
        disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated(
            "UpdateTransparencyBehavior",
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
                behavior.update_transparency(command.value);
            }
        });
        disposables.add(Box::new(TimerDisposable::new(timer)));
    }
}


