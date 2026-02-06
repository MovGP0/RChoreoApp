use std::time::Duration;

use crossbeam_channel::{Receiver, Sender};
use slint::TimerMode;

use crate::behavior::{Behavior, CompositeDisposable, TimerDisposable};
use crate::logging::BehaviorLog;
use crate::preferences::Preferences;

use super::choreography_settings_view_model::ChoreographySettingsViewModel;
use super::messages::{RedrawFloorCommand, UpdateShowLegendCommand};
use nject::injectable;

#[injectable]
pub struct UpdateShowLegendBehavior<P: Preferences + Clone> {
    preferences: P,
    redraw_sender: Sender<RedrawFloorCommand>,
    receiver: Option<Receiver<UpdateShowLegendCommand>>,
}

impl<P: Preferences + Clone> UpdateShowLegendBehavior<P> {
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
        receiver: Receiver<UpdateShowLegendCommand>,
    ) -> Self {
        Self {
            preferences,
            redraw_sender,
            receiver: Some(receiver),
        }
    }

    fn initialize(&self, view_model: &mut ChoreographySettingsViewModel) {
        view_model.show_legend = self
            .preferences
            .get_bool(choreo_models::SettingsPreferenceKeys::SHOW_LEGEND, false);
    }

    fn update_show_legend(&self, view_model: &mut ChoreographySettingsViewModel, value: bool) {
        view_model.show_legend = value;
        self.preferences
            .set_bool(choreo_models::SettingsPreferenceKeys::SHOW_LEGEND, value);
        let _ = self.redraw_sender.send(RedrawFloorCommand);
    }
}

impl<P: Preferences + Clone + 'static> Behavior<ChoreographySettingsViewModel> for UpdateShowLegendBehavior<P> {
    fn activate(
        &self,
        view_model: &mut ChoreographySettingsViewModel,
        disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated(
            "UpdateShowLegendBehavior",
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
                behavior.update_show_legend(&mut view_model, command.value);
                view_model.notify_changed();
            }
        });
        disposables.add(Box::new(TimerDisposable::new(timer)));
    }
}


