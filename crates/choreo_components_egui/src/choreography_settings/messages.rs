use choreo_master_mobile_json::Color;
use choreo_models::ChoreographyModel;

use super::state::DateParts;
use super::state::SelectedSceneState;

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum UpdateSelectedSceneCommand {
    SyncFromSelected,
    SceneName(String),
    SceneText(String),
    SceneFixedPositions(bool),
    SceneColor(Color),
    SceneTimestamp { has_timestamp: bool, seconds: f64 },
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum ChoreographySettingsCommand {
    LoadChoreography {
        choreography: Box<ChoreographyModel>,
        selected_scene: Option<SelectedSceneState>,
    },
    LoadSettingsPreferences {
        show_timestamps: bool,
        positions_at_side: bool,
        snap_to_grid: bool,
    },
    UpdateComment(String),
    UpdateName(String),
    UpdateSubtitle(String),
    UpdateDate(DateParts),
    UpdateVariation(String),
    UpdateAuthor(String),
    UpdateDescription(String),
    UpdateFloorFront(i32),
    UpdateFloorBack(i32),
    UpdateFloorLeft(i32),
    UpdateFloorRight(i32),
    UpdateGridResolution(i32),
    UpdateDrawPathFrom(bool),
    UpdateDrawPathTo(bool),
    UpdateGridLines(bool),
    UpdateSnapToGrid(bool),
    UpdateShowTimestamps(bool),
    UpdateShowLegend(bool),
    UpdatePositionsAtSide(bool),
    UpdateTransparency(f64),
    UpdateFloorColor(Color),
    UpdateSelectedScene(UpdateSelectedSceneCommand),
    ClearEphemeralOutputs,
}
