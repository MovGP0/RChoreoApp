use std::time::Duration;

use crossbeam_channel::{Receiver, Sender};
use slint::TimerMode;

use crate::behavior::{Behavior, CompositeDisposable, TimerDisposable};
use crate::logging::BehaviorLog;
use crate::preferences::Preferences;

use super::choreography_settings_view_model::ChoreographySettingsViewModel;
use super::messages::{RedrawFloorCommand, UpdateDrawPathFromCommand};
use nject::injectable;

#[injectable]
pub struct UpdateDrawPathFromBehavior<P: Preferences + Clone> {
    preferences: P,
    redraw_sender: Sender<RedrawFloorCommand>,
    receiver: Option<Receiver<UpdateDrawPathFromCommand>>,
}

impl<P: Preferences + Clone> UpdateDrawPathFromBehavior<P> {
    pub fn new(preferences: P, redraw_sender: Sender<RedrawFloorCommand>) -> Self {
        Self {
            preferences,
            redraw_sender,
            receiver: None,
        }
    }

    pub fn new_with_receiver(
        preferences: P,
        redraw_sender: Sender<RedrawFloorCommand>,
        receiver: Receiver<UpdateDrawPathFromCommand>,
    ) -> Self {
        Self {
            preferences,
            redraw_sender,
            receiver: Some(receiver),
        }
    }

    fn initialize(&self, view_model: &mut ChoreographySettingsViewModel) {
        view_model.draw_path_from = self
            .preferences
            .get_bool(choreo_models::SettingsPreferenceKeys::DRAW_PATH_FROM, false);
    }

    fn update_draw_path_from(&self, view_model: &mut ChoreographySettingsViewModel, value: bool) {
        view_model.draw_path_from = value;
        self.preferences
            .set_bool(choreo_models::SettingsPreferenceKeys::DRAW_PATH_FROM, value);
        let _ = self.redraw_sender.send(RedrawFloorCommand);
    }
}

impl<P: Preferences + Clone + 'static> Behavior<ChoreographySettingsViewModel> for UpdateDrawPathFromBehavior<P> {
    fn activate(
        &self,
        view_model: &mut ChoreographySettingsViewModel,
        disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated(
            "UpdateDrawPathFromBehavior",
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
            preferences: self.preferences.clone(),
            redraw_sender: self.redraw_sender.clone(),
            receiver: None,
        };
        let timer = slint::Timer::default();
        timer.start(TimerMode::Repeated, Duration::from_millis(16), move || {
            while let Ok(command) = receiver.try_recv() {
                let mut view_model = view_model_handle.borrow_mut();
                behavior.update_draw_path_from(&mut view_model, command.value);
                view_model.notify_changed();
            }
        });
        disposables.add(Box::new(TimerDisposable::new(timer)));
    }
}


