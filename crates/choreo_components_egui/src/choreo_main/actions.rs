use super::state::InteractionMode;
use super::state::SceneState;

#[derive(Debug, Clone, PartialEq)]
pub struct OpenAudioRequested {
    pub file_path: String,
    pub trace_context: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenSvgFileCommand {
    pub file_path: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ChoreoMainAction {
    Initialize,
    NavigateToSettings,
    NavigateToMain,
    NavigateToDancers,
    ShowDialog { content: Option<String> },
    HideDialog,
    RequestOpenAudio(OpenAudioRequested),
    RequestOpenImage { file_path: String },
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
    SelectScene {
        index: usize,
    },
    UpdateAudioPosition {
        seconds: f64,
    },
    ClearOutgoingCommands,
}
