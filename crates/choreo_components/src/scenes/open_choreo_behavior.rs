use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::time::Duration;

use choreo_master_mobile_json::import;
use choreo_models::{ChoreographyModelMapper, SettingsPreferenceKeys};
use crossbeam_channel::{Receiver, Sender};
use nject::injectable;

use crate::audio_player::{CloseAudioFileCommand, OpenAudioFileCommand};
use crate::behavior::{Behavior, CompositeDisposable, TimerDisposable};
use crate::choreography_settings::ShowTimestampsChangedEvent;
use crate::global::GlobalStateActor;
use crate::logging::BehaviorLog;
use crate::observability::start_internal_span;
use crate::preferences::Preferences;

use super::messages::{OpenChoreoRequested, ReloadScenesCommand};
use super::scenes_view_model::ScenesPaneViewModel;

#[derive(Clone, Default)]
pub struct OpenChoreoActions {
    pub request_open_choreo: Option<Rc<dyn Fn(Sender<OpenChoreoRequested>)>>,
}

#[derive(Clone)]
pub struct OpenChoreoBehaviorDependencies {
    pub global_state: Rc<GlobalStateActor>,
    pub preferences: Rc<dyn Preferences>,
    pub open_audio_sender: Sender<OpenAudioFileCommand>,
    pub close_audio_sender: Sender<CloseAudioFileCommand>,
    pub reload_scenes_sender: Sender<ReloadScenesCommand>,
    pub show_timestamps_sender: Sender<ShowTimestampsChangedEvent>,
    pub actions: OpenChoreoActions,
    pub open_choreo_sender: Sender<OpenChoreoRequested>,
    pub open_choreo_receiver: Receiver<OpenChoreoRequested>,
}

#[derive(Clone)]
#[injectable]
#[inject(
    |deps: OpenChoreoBehaviorDependencies| {
        Self::new(deps)
    }
)]
pub struct OpenChoreoBehavior {
    global_state: Rc<GlobalStateActor>,
    preferences: Rc<dyn Preferences>,
    open_audio_sender: Sender<OpenAudioFileCommand>,
    close_audio_sender: Sender<CloseAudioFileCommand>,
    reload_scenes_sender: Sender<ReloadScenesCommand>,
    show_timestamps_sender: Sender<ShowTimestampsChangedEvent>,
    actions: OpenChoreoActions,
    open_choreo_sender: Sender<OpenChoreoRequested>,
    open_choreo_receiver: Receiver<OpenChoreoRequested>,
}

impl OpenChoreoBehavior {
    pub fn new(deps: OpenChoreoBehaviorDependencies) -> Self {
        Self {
            global_state: deps.global_state,
            preferences: deps.preferences,
            open_audio_sender: deps.open_audio_sender,
            close_audio_sender: deps.close_audio_sender,
            reload_scenes_sender: deps.reload_scenes_sender,
            show_timestamps_sender: deps.show_timestamps_sender,
            actions: deps.actions,
            open_choreo_sender: deps.open_choreo_sender,
            open_choreo_receiver: deps.open_choreo_receiver,
        }
    }

    fn open_choreo(&self) {
        let mut span = start_internal_span("scenes.open_choreo.requested", None);
        let Some(request_open) = self.actions.request_open_choreo.as_ref() else {
            span.set_bool_attribute("choreo.success", false);
            return;
        };

        request_open(self.open_choreo_sender.clone());
        span.set_bool_attribute("choreo.success", true);
    }

    fn load_last_opened(&self, view_model: &mut ScenesPaneViewModel) {
        let mut span = start_internal_span("scenes.open_choreo.load_last_opened", None);
        let path = self
            .preferences
            .get_string(SettingsPreferenceKeys::LAST_OPENED_CHOREO_FILE, "");
        if path.trim().is_empty() {
            span.set_bool_attribute("choreo.success", false);
            return;
        }

        let path_buf = PathBuf::from(path);
        if !path_buf.exists() {
            span.set_bool_attribute("choreo.success", false);
            return;
        }

        self.load_choreo(&path_buf, view_model);
        span.set_bool_attribute("choreo.success", true);
    }

    fn load_choreo(&self, path: &Path, view_model: &mut ScenesPaneViewModel) {
        let mut span = start_internal_span("scenes.open_choreo.load_path", None);
        span.set_string_attribute("choreo.file.path", path.to_string_lossy().into_owned());
        let Ok(contents) = std::fs::read_to_string(path) else {
            span.set_bool_attribute("choreo.success", false);
            return;
        };

        let Ok(json_model) = import(&contents) else {
            span.set_bool_attribute("choreo.success", false);
            return;
        };

        let mapper = ChoreographyModelMapper;
        let mapped = mapper.map_to_model(&json_model);

        let updated = self.global_state.try_update(|global_state| {
            global_state.choreography = mapped;
        });
        if !updated {
            span.set_bool_attribute("choreo.success", false);
            return;
        }

        view_model.update_can_save();
        let _ = self.reload_scenes_sender.send(ReloadScenesCommand);
        let Some(value) = self
            .global_state
            .try_with_state(|global_state| global_state.choreography.settings.show_timestamps)
        else {
            return;
        };
        let _ = self
            .show_timestamps_sender
            .send(ShowTimestampsChangedEvent { is_enabled: value });

        self.preferences.set_string(
            SettingsPreferenceKeys::LAST_OPENED_CHOREO_FILE,
            path.to_string_lossy().into_owned(),
        );

        self.try_load_audio(path);
        span.set_bool_attribute("choreo.success", true);
    }

    fn load_choreo_from_contents(
        &self,
        file_path: Option<String>,
        file_name: Option<String>,
        contents: String,
        view_model: &mut ScenesPaneViewModel,
    ) {
        let mut span = start_internal_span("scenes.open_choreo.load_contents", None);
        if let Some(file_path) = file_path.as_ref() {
            span.set_string_attribute("choreo.file.path", file_path.clone());
        }
        if let Some(file_name) = file_name.as_ref() {
            span.set_string_attribute("choreo.file.name", file_name.clone());
        }
        let Ok(json_model) = import(&contents) else {
            span.set_bool_attribute("choreo.success", false);
            return;
        };

        let mapper = ChoreographyModelMapper;
        let mapped = mapper.map_to_model(&json_model);

        let updated = self.global_state.try_update(|global_state| {
            global_state.choreography = mapped;
        });
        if !updated {
            span.set_bool_attribute("choreo.success", false);
            return;
        }

        view_model.update_can_save();
        let _ = self.reload_scenes_sender.send(ReloadScenesCommand);
        let Some(value) = self
            .global_state
            .try_with_state(|global_state| global_state.choreography.settings.show_timestamps)
        else {
            return;
        };
        let _ = self
            .show_timestamps_sender
            .send(ShowTimestampsChangedEvent { is_enabled: value });

        if let Some(path) = file_path {
            self.preferences.set_string(
                SettingsPreferenceKeys::LAST_OPENED_CHOREO_FILE,
                path.clone(),
            );
            self.try_load_audio(Path::new(&path));
        } else if let Some(name) = file_name {
            self.preferences
                .set_string(SettingsPreferenceKeys::LAST_OPENED_CHOREO_FILE, name);
            let _ = self.close_audio_sender.try_send(CloseAudioFileCommand {
                trace_context: None,
            });
        } else {
            let _ = self.close_audio_sender.try_send(CloseAudioFileCommand {
                trace_context: None,
            });
        }
        span.set_bool_attribute("choreo.success", true);
    }

    fn try_load_audio(&self, choreography_path: &Path) {
        let mut span = start_internal_span("scenes.open_choreo.try_load_audio", None);
        span.set_string_attribute(
            "choreo.file.path",
            choreography_path.to_string_lossy().into_owned(),
        );
        let Some(settings) = self
            .global_state
            .try_with_state(|global_state| global_state.choreography.settings.clone())
        else {
            span.set_bool_attribute("choreo.success", false);
            return;
        };
        let mut candidates = Vec::new();

        if let Some(path) = settings.music_path_absolute.as_ref()
            && !path.trim().is_empty()
        {
            candidates.push(path.clone());
        }

        if let Some(relative) = settings.music_path_relative.as_ref()
            && !relative.trim().is_empty()
        {
            let base_dir = choreography_path.parent().unwrap_or_else(|| Path::new(""));
            candidates.push(base_dir.join(relative).to_string_lossy().into_owned());
        }

        if candidates.is_empty() {
            let stored = self
                .preferences
                .get_string(SettingsPreferenceKeys::LAST_OPENED_AUDIO_FILE, "");
            if !stored.trim().is_empty() {
                candidates.push(stored);
            }
        }
        span.set_f64_attribute("choreo.audio.candidate_count", candidates.len() as f64);

        for candidate in candidates {
            let path = PathBuf::from(&candidate);
            if path.exists() {
                let _ = self.open_audio_sender.try_send(OpenAudioFileCommand {
                    file_path: candidate,
                    trace_context: None,
                });
                span.set_bool_attribute("choreo.success", true);
                return;
            }
        }

        let _ = self.close_audio_sender.try_send(CloseAudioFileCommand {
            trace_context: None,
        });
        span.set_bool_attribute("choreo.success", false);
    }
}

impl Behavior<ScenesPaneViewModel> for OpenChoreoBehavior {
    fn activate(
        &self,
        view_model: &mut ScenesPaneViewModel,
        disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated("OpenChoreoBehavior", "ScenesPaneViewModel");
        self.load_last_opened(view_model);
        let behavior = self.clone();
        view_model.set_open_choreo_handler(Some(Rc::new(move |_view_model| {
            behavior.open_choreo();
        })));

        let Some(view_model_handle) = view_model.self_handle().and_then(|handle| handle.upgrade())
        else {
            return;
        };

        let receiver = self.open_choreo_receiver.clone();
        let behavior = self.clone();
        let timer = slint::Timer::default();
        timer.start(
            slint::TimerMode::Repeated,
            Duration::from_millis(16),
            move || {
                while let Ok(request) = receiver.try_recv() {
                    let mut span = start_internal_span("scenes.open_choreo.command_handled", None);
                    span.set_bool_attribute("choreo.scenes.has_file_path", request.file_path.is_some());
                    span.set_bool_attribute("choreo.scenes.has_file_name", request.file_name.is_some());
                    let mut view_model = view_model_handle.borrow_mut();
                    behavior.load_choreo_from_contents(
                        request.file_path,
                        request.file_name,
                        request.contents,
                        &mut view_model,
                    );
                }
            },
        );
        disposables.add(Box::new(TimerDisposable::new(timer)));
    }
}
