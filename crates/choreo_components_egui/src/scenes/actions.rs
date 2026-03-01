use choreo_models::ChoreographyModel;

#[derive(Debug, Clone, PartialEq)]
pub enum ScenesAction {
    LoadScenes {
        choreography: Box<ChoreographyModel>,
    },
    ReloadScenes,
    UpdateSearchText(String),
    InsertScene {
        insert_after: bool,
    },
    SelectScene {
        index: usize,
    },
    SelectSceneFromAudioPosition {
        position_seconds: f64,
    },
    ApplyPlacementModeForSelected,
    SyncShowTimestampsFromChoreography,
    UpdateShowTimestamps(bool),
    OpenChoreography {
        choreography: Box<ChoreographyModel>,
        file_path: Option<String>,
        file_name: Option<String>,
        audio_path: Option<String>,
    },
    SaveChoreography,
    ClearEphemeralOutputs,
}
