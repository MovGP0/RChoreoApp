use super::state::InteractionMode;
use super::state::SceneState;
use crate::audio_player::actions::AudioPlayerAction;
use crate::choreography_settings::actions::ChoreographySettingsAction;
use crate::dancers::actions::DancersAction;
use crate::floor::actions::FloorAction;
use crate::observability::TraceContext;
use crate::settings::actions::SettingsAction;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenChoreoRequested {
    pub file_path: Option<String>,
    pub file_name: Option<String>,
    pub contents: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SaveChoreoRequested {
    pub file_path: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OpenAudioRequested {
    pub file_path: String,
    pub trace_context: Option<TraceContext>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenSvgFileCommand {
    pub file_path: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ChoreoMainAction {
    Initialize,
    ToggleNav,
    CloseNav,
    OpenSettings,
    CloseSettings,
    OpenAudioPanel,
    CloseAudioPanel,
    SelectMode {
        index: i32,
    },
    ResetFloorViewport,
    NavigateToSettings,
    NavigateToMain,
    NavigateToDancers,
    ShowDialog {
        content: Option<String>,
    },
    HideDialog,
    RequestOpenChoreo(OpenChoreoRequested),
    RequestSaveChoreo,
    RequestOpenAudio(OpenAudioRequested),
    RequestOpenImage {
        file_path: String,
    },
    ApplyOpenSvgFile(OpenSvgFileCommand),
    RestoreLastOpenedSvg {
        file_path: Option<String>,
        path_exists: bool,
    },
    ApplyInteractionMode {
        mode: InteractionMode,
        selected_positions_count: usize,
    },
    SetScenes {
        scenes: Vec<SceneState>,
    },
    UpdateSceneSearchText(String),
    InsertScene {
        insert_after: bool,
    },
    DeleteSelectedScene,
    SelectScene {
        index: usize,
    },
    UpdateAudioPosition {
        seconds: f64,
    },
    LinkSelectedSceneToAudioPosition,
    FloorAction(FloorAction),
    AudioPlayerAction(AudioPlayerAction),
    ChoreographySettingsAction(ChoreographySettingsAction),
    SettingsAction(SettingsAction),
    DancersAction(DancersAction),
    ClearOutgoingCommands,
}
