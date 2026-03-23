#[derive(Debug, Clone, PartialEq)]
pub enum AudioPlayerAction {
    Initialize,
    TogglePlayPause,
    Stop,
    SeekToPosition {
        position: f64,
    },
    PositionDragStarted,
    PositionPreviewChanged {
        position: f64,
    },
    PositionDragCompleted {
        position: f64,
    },
    PlayerPositionSampled {
        position: f64,
    },
    SpeedChanged {
        speed: f64,
    },
    SetScenes {
        scenes: Vec<super::state::AudioPlayerScene>,
        selected_scene_id: Option<i32>,
        choreography_scenes: Vec<super::state::AudioPlayerChoreographyScene>,
    },
    UpdateTicksAndLinkState,
    LinkSceneToPosition,
    OpenAudioFile {
        file_path: String,
        file_exists: bool,
    },
    CloseAudioFile,
    PublishPositionIfChanged,
}
