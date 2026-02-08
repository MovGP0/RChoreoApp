use std::rc::Rc;
use std::time::Duration;

use crossbeam_channel::{Receiver, Sender};
use slint::TimerMode;

use crate::behavior::{Behavior, CompositeDisposable, TimerDisposable};
use crate::global::GlobalStateActor;
use crate::logging::BehaviorLog;
use crate::preferences::Preferences;

use super::choreography_settings_view_model::ChoreographySettingsViewModel;
use super::messages::{
    RedrawFloorCommand, ShowTimestampsChangedEvent, UpdateShowTimestampsCommand,
};
use nject::injectable;

#[injectable]
pub struct UpdateShowTimestampsBehavior<P: Preferences + Clone> {
    global_state: Rc<GlobalStateActor>,
    preferences: P,
    redraw_sender: Sender<RedrawFloorCommand>,
    show_timestamps_sender: Sender<ShowTimestampsChangedEvent>,
    receiver: Option<Receiver<UpdateShowTimestampsCommand>>,
}

impl<P: Preferences + Clone> UpdateShowTimestampsBehavior<P> {
    pub fn new(
        global_state: Rc<GlobalStateActor>,
        preferences: P,
        redraw_sender: Sender<RedrawFloorCommand>,
        show_timestamps_sender: Sender<ShowTimestampsChangedEvent>,
    ) -> Self {
        Self {
            global_state,
            preferences,
            redraw_sender,
            show_timestamps_sender,
            receiver: None,
        }
    }

    pub fn new_with_receiver(
        global_state: Rc<GlobalStateActor>,
        preferences: P,
        redraw_sender: Sender<RedrawFloorCommand>,
        show_timestamps_sender: Sender<ShowTimestampsChangedEvent>,
        receiver: Receiver<UpdateShowTimestampsCommand>,
    ) -> Self {
        Self {
            global_state,
            preferences,
            redraw_sender,
            show_timestamps_sender,
            receiver: Some(receiver),
        }
    }

    fn initialize(&self, view_model: &mut ChoreographySettingsViewModel) {
        let value = self
            .preferences
            .get_bool(choreo_models::SettingsPreferenceKeys::SHOW_TIMESTAMPS, true);
        view_model.show_timestamps = value;
        self.global_state.try_update(|global_state| {
            global_state.choreography.settings.show_timestamps = value;
        });
    }

    fn update_show_timestamps(&self, view_model: &mut ChoreographySettingsViewModel, value: bool) {
        view_model.show_timestamps = value;
        let updated = self.global_state.try_update(|global_state| {
            global_state.choreography.settings.show_timestamps = value;
        });
        if !updated {
            return;
        }
        let _ = self.redraw_sender.send(RedrawFloorCommand);
        let _ = self
            .show_timestamps_sender
            .send(ShowTimestampsChangedEvent { is_enabled: value });
    }
}

impl<P: Preferences + Clone + 'static> Behavior<ChoreographySettingsViewModel>
    for UpdateShowTimestampsBehavior<P>
{
    fn activate(
        &self,
        view_model: &mut ChoreographySettingsViewModel,
        disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated(
            "UpdateShowTimestampsBehavior",
            "ChoreographySettingsViewModel",
        );
        self.initialize(view_model);
        let Some(receiver) = self.receiver.clone() else {
            return;
        };
        let Some(view_model_handle) = view_model.self_handle().and_then(|handle| handle.upgrade())
        else {
            return;
        };
        let behavior = Self {
            global_state: self.global_state.clone(),
            preferences: self.preferences.clone(),
            redraw_sender: self.redraw_sender.clone(),
            show_timestamps_sender: self.show_timestamps_sender.clone(),
            receiver: None,
        };
        let timer = slint::Timer::default();
        timer.start(TimerMode::Repeated, Duration::from_millis(16), move || {
            while let Ok(command) = receiver.try_recv() {
                let mut view_model = view_model_handle.borrow_mut();
                behavior.update_show_timestamps(&mut view_model, command.value);
                view_model.notify_changed();
            }
        });
        disposables.add(Box::new(TimerDisposable::new(timer)));
    }
}
