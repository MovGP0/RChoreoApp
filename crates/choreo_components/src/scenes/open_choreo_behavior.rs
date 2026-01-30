use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use crossbeam_channel::Sender;
use nject::injectable;
use choreo_master_mobile_json::import_from_file;
use choreo_models::{ChoreographyModelMapper, SettingsPreferenceKeys};

use crate::audio_player::{CloseAudioFileCommand, OpenAudioFileCommand};
use crate::behavior::{Behavior, CompositeDisposable};
use crate::choreography_settings::ShowTimestampsChangedEvent;
use crate::global::GlobalStateModel;
use crate::logging::BehaviorLog;
use crate::preferences::Preferences;

use super::messages::ReloadScenesCommand;
use super::scenes_view_model::ScenesPaneViewModel;

#[derive(Clone, Default)]
pub struct OpenChoreoActions {
    pub pick_choreo_path: Option<Rc<dyn Fn() -> Option<String>>>,
}

#[derive(Clone)]
#[injectable]
#[inject(
    |global_state: Rc<RefCell<GlobalStateModel>>,
     preferences: Rc<dyn Preferences>,
     open_audio_sender: Sender<OpenAudioFileCommand>,
     close_audio_sender: Sender<CloseAudioFileCommand>,
     reload_scenes_sender: Sender<ReloadScenesCommand>,
     show_timestamps_sender: Sender<ShowTimestampsChangedEvent>,
     actions: OpenChoreoActions| {
        Self::new(
            global_state,
            preferences,
            open_audio_sender,
            close_audio_sender,
            reload_scenes_sender,
            show_timestamps_sender,
            actions,
        )
    }
)]
pub struct OpenChoreoBehavior {
    global_state: Rc<RefCell<GlobalStateModel>>,
    preferences: Rc<dyn Preferences>,
    open_audio_sender: Sender<OpenAudioFileCommand>,
    close_audio_sender: Sender<CloseAudioFileCommand>,
    reload_scenes_sender: Sender<ReloadScenesCommand>,
    show_timestamps_sender: Sender<ShowTimestampsChangedEvent>,
    actions: OpenChoreoActions,
}

impl OpenChoreoBehavior {
    pub fn new(
        global_state: Rc<RefCell<GlobalStateModel>>,
        preferences: Rc<dyn Preferences>,
        open_audio_sender: Sender<OpenAudioFileCommand>,
        close_audio_sender: Sender<CloseAudioFileCommand>,
        reload_scenes_sender: Sender<ReloadScenesCommand>,
        show_timestamps_sender: Sender<ShowTimestampsChangedEvent>,
        actions: OpenChoreoActions,
    ) -> Self {
        Self {
            global_state,
            preferences,
            open_audio_sender,
            close_audio_sender,
            reload_scenes_sender,
            show_timestamps_sender,
            actions,
        }
    }

    fn open_choreo(&self, view_model: &mut ScenesPaneViewModel) {
        let Some(picker) = self.actions.pick_choreo_path.as_ref() else {
            return;
        };

        let Some(path) = picker() else {
            return;
        };

        if !is_choreo_file(&path) {
            return;
        }

        self.load_choreo(Path::new(&path), view_model);
    }

    fn load_last_opened(&self, view_model: &mut ScenesPaneViewModel) {
        let path = self
            .preferences
            .get_string(SettingsPreferenceKeys::LAST_OPENED_CHOREO_FILE, "");
        if path.trim().is_empty() {
            return;
        }

        let path_buf = PathBuf::from(path);
        if !path_buf.exists() {
            return;
        }

        self.load_choreo(&path_buf, view_model);
    }

    fn load_choreo(&self, path: &Path, view_model: &mut ScenesPaneViewModel) {
        let Ok(json_model) = import_from_file(path) else {
            return;
        };

        let mapper = ChoreographyModelMapper;
        let mapped = mapper.map_to_model(&json_model);

        {
            let mut global_state = self.global_state.borrow_mut();
            global_state.choreography = mapped;
        }

        view_model.update_can_save();
        let _ = self.reload_scenes_sender.send(ReloadScenesCommand);
        let value = self
            .global_state
            .borrow()
            .choreography
            .settings
            .show_timestamps;
        let _ = self.show_timestamps_sender.send(ShowTimestampsChangedEvent {
            is_enabled: value,
        });

        self.preferences.set_string(
            SettingsPreferenceKeys::LAST_OPENED_CHOREO_FILE,
            path.to_string_lossy().into_owned(),
        );

        self.try_load_audio(path);
    }

    fn try_load_audio(&self, choreography_path: &Path) {
        let settings = self.global_state.borrow().choreography.settings.clone();
        let mut candidates = Vec::new();

        if let Some(path) = settings.music_path_absolute.as_ref()
            && !path.trim().is_empty()
        {
            candidates.push(path.clone());
        }

        if let Some(relative) = settings.music_path_relative.as_ref()
            && !relative.trim().is_empty()
        {
            let base_dir = choreography_path
                .parent()
                .unwrap_or_else(|| Path::new(""));
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

        for candidate in candidates {
            let path = PathBuf::from(&candidate);
            if path.exists() {
                let _ = self.open_audio_sender.send(OpenAudioFileCommand {
                    file_path: candidate,
                });
                return;
            }
        }

        let _ = self.close_audio_sender.send(CloseAudioFileCommand);
    }
}

impl Behavior<ScenesPaneViewModel> for OpenChoreoBehavior {
    fn activate(&self, view_model: &mut ScenesPaneViewModel, _disposables: &mut CompositeDisposable) {
        BehaviorLog::behavior_activated("OpenChoreoBehavior", "ScenesPaneViewModel");
        self.load_last_opened(view_model);
        let behavior = self.clone();
        view_model.set_open_choreo_handler(Some(Rc::new(move |view_model| {
            behavior.open_choreo(view_model);
        })));
    }
}

fn is_choreo_file(path: &str) -> bool {
    Path::new(path)
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.eq_ignore_ascii_case("choreo"))
        .unwrap_or(false)
}
