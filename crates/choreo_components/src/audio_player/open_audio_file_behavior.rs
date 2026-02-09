use std::io;
use std::path::Path;
use std::rc::Rc;
use std::time::Duration;

use choreo_models::SettingsPreferenceKeys;
use crossbeam_channel::Receiver;
use nject::injectable;
use slint::TimerMode;

use crate::behavior::{Behavior, CompositeDisposable, TimerDisposable};
use crate::logging::BehaviorLog;
use crate::observability::start_internal_span;
use crate::preferences::Preferences;

use super::audio_player_view_model::AudioPlayerViewModel;
use super::create_platform_audio_player;
use super::messages::OpenAudioFileCommand;

#[injectable]
#[inject(|receiver: Receiver<OpenAudioFileCommand>, preferences: Rc<dyn Preferences>| {
    Self::new(receiver, preferences)
})]
pub struct OpenAudioFileBehavior {
    receiver: Receiver<OpenAudioFileCommand>,
    preferences: Rc<dyn Preferences>,
}

impl OpenAudioFileBehavior {
    pub fn new(receiver: Receiver<OpenAudioFileCommand>, preferences: Rc<dyn Preferences>) -> Self {
        Self {
            receiver,
            preferences,
        }
    }
}

impl Behavior<AudioPlayerViewModel> for OpenAudioFileBehavior {
    fn activate(
        &self,
        view_model: &mut AudioPlayerViewModel,
        disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated("OpenAudioFileBehavior", "AudioPlayerViewModel");
        let Some(view_model_handle) = view_model.self_handle().and_then(|handle| handle.upgrade())
        else {
            return;
        };
        let receiver = self.receiver.clone();
        let preferences = Rc::clone(&self.preferences);
        let timer = slint::Timer::default();
        timer.start(TimerMode::Repeated, Duration::from_millis(16), move || {
            let mut latest_command = None;
            while let Ok(command) = receiver.try_recv() {
                latest_command = Some(command);
            }
            let Some(command) = latest_command else {
                return;
            };

            let mut span = start_internal_span(
                "audio_player.open_audio_file",
                command.trace_context.as_ref(),
            );

            let extension = Path::new(command.file_path.as_str())
                .extension()
                .and_then(|value| value.to_str())
                .unwrap_or_default()
                .to_ascii_lowercase();
            let file_name = Path::new(command.file_path.as_str())
                .file_name()
                .and_then(|value| value.to_str())
                .unwrap_or_default()
                .to_string();
            span.set_string_attribute("choreo.command.type", "OpenAudioFileCommand".to_string());
            span.set_string_attribute("choreo.audio.file_extension", extension);
            span.set_string_attribute("choreo.audio.file_name", file_name);

            if command.file_path.trim().is_empty() {
                span.set_bool_attribute("choreo.success", false);
                return;
            }

            let file_path = command.file_path;
            let stream_path = file_path.clone();

            {
                let Ok(mut view_model) = view_model_handle.try_borrow_mut() else {
                    return;
                };
                view_model.stream_factory = Some(Box::new(move || {
                    let file = std::fs::File::open(&stream_path)?;
                    Ok(Box::new(file) as Box<dyn io::Read + Send>)
                }));
                view_model.set_player(create_platform_audio_player(file_path.clone()));
            }

            preferences.set_string(SettingsPreferenceKeys::LAST_OPENED_AUDIO_FILE, file_path);
            span.set_bool_attribute("choreo.success", true);
        });

        disposables.add(Box::new(TimerDisposable::new(timer)));
    }
}
