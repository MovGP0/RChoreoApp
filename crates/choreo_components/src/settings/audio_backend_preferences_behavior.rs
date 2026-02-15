use std::time::Duration;

use crossbeam_channel::Receiver;
use nject::injectable;
use slint::TimerMode;

use crate::behavior::{Behavior, CompositeDisposable, TimerDisposable};
use crate::logging::BehaviorLog;
use crate::preferences::Preferences;

use super::messages::UpdateAudioPlayerBackendCommand;
use super::settings_view_model::SettingsViewModel;

#[injectable]
#[inject(|preferences: P, receiver: Receiver<UpdateAudioPlayerBackendCommand>| {
    Self::new(preferences, receiver)
})]
pub struct AudioBackendPreferencesBehavior<P: Preferences + Clone + 'static> {
    preferences: P,
    receiver: Receiver<UpdateAudioPlayerBackendCommand>,
}

impl<P: Preferences + Clone + 'static> AudioBackendPreferencesBehavior<P> {
    pub fn new(preferences: P, receiver: Receiver<UpdateAudioPlayerBackendCommand>) -> Self {
        Self {
            preferences,
            receiver,
        }
    }
}

impl<P: Preferences + Clone + 'static> Behavior<SettingsViewModel>
    for AudioBackendPreferencesBehavior<P>
{
    fn activate(&self, view_model: &mut SettingsViewModel, disposables: &mut CompositeDisposable) {
        BehaviorLog::behavior_activated("AudioBackendPreferencesBehavior", "SettingsViewModel");
        let Some(view_model_handle) = view_model.self_handle().and_then(|handle| handle.upgrade())
        else {
            return;
        };

        let preferences = self.preferences.clone();
        let receiver = self.receiver.clone();
        let timer = slint::Timer::default();
        timer.start(TimerMode::Repeated, Duration::from_millis(16), move || {
            while let Ok(command) = receiver.try_recv() {
                let mut view_model = view_model_handle.borrow_mut();
                view_model.audio_player_backend = command.backend;
                preferences.set_string(
                    choreo_models::SettingsPreferenceKeys::AUDIO_PLAYER_BACKEND,
                    command.backend.as_preference().to_string(),
                );
            }
        });

        disposables.add(Box::new(TimerDisposable::new(timer)));
    }
}
