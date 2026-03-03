use choreo_master_mobile_json::Color;
use choreo_models::ChoreographyModel;

use super::messages::ChoreographySettingsCommand;
use super::messages::UpdateSelectedSceneCommand;
use super::state::SelectedSceneState;

#[derive(Debug, Clone, PartialEq)]
pub enum UpdateSelectedSceneAction {
    SyncFromSelected,
    SceneName(String),
    SceneText(String),
    SceneFixedPositions(bool),
    SceneColor(Color),
    SceneTimestamp { has_timestamp: bool, seconds: f64 },
}

#[derive(Debug, Clone, PartialEq)]
pub enum ChoreographySettingsAction {
    LoadChoreography {
        choreography: Box<ChoreographyModel>,
        selected_scene: Option<SelectedSceneState>,
    },
    LoadSettingsPreferences {
        show_timestamps: bool,
        positions_at_side: bool,
        snap_to_grid: bool,
    },
    InitializeDrawPathFrom(bool),
    InitializeDrawPathTo(bool),
    InitializeShowLegend(bool),
    InitializeShowTimestamps(bool),
    InitializePositionsAtSide(bool),
    InitializeSnapToGrid(bool),
    UpdateComment(String),
    UpdateName(String),
    UpdateSubtitle(String),
    UpdateDate {
        year: i32,
        month: u8,
        day: u8,
    },
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
    UpdateSelectedScene(UpdateSelectedSceneAction),
    ClearEphemeralOutputs,
}

#[must_use]
pub fn from_command(command: ChoreographySettingsCommand) -> ChoreographySettingsAction {
    match command {
        ChoreographySettingsCommand::LoadChoreography {
            choreography,
            selected_scene,
        } => ChoreographySettingsAction::LoadChoreography {
            choreography,
            selected_scene,
        },
        ChoreographySettingsCommand::LoadSettingsPreferences {
            show_timestamps,
            positions_at_side,
            snap_to_grid,
        } => ChoreographySettingsAction::LoadSettingsPreferences {
            show_timestamps,
            positions_at_side,
            snap_to_grid,
        },
        ChoreographySettingsCommand::UpdateComment(value) => {
            ChoreographySettingsAction::UpdateComment(value)
        }
        ChoreographySettingsCommand::UpdateName(value) => ChoreographySettingsAction::UpdateName(value),
        ChoreographySettingsCommand::UpdateSubtitle(value) => {
            ChoreographySettingsAction::UpdateSubtitle(value)
        }
        ChoreographySettingsCommand::UpdateDate(value) => ChoreographySettingsAction::UpdateDate {
            year: value.year,
            month: value.month,
            day: value.day,
        },
        ChoreographySettingsCommand::UpdateVariation(value) => {
            ChoreographySettingsAction::UpdateVariation(value)
        }
        ChoreographySettingsCommand::UpdateAuthor(value) => {
            ChoreographySettingsAction::UpdateAuthor(value)
        }
        ChoreographySettingsCommand::UpdateDescription(value) => {
            ChoreographySettingsAction::UpdateDescription(value)
        }
        ChoreographySettingsCommand::UpdateFloorFront(value) => {
            ChoreographySettingsAction::UpdateFloorFront(value)
        }
        ChoreographySettingsCommand::UpdateFloorBack(value) => {
            ChoreographySettingsAction::UpdateFloorBack(value)
        }
        ChoreographySettingsCommand::UpdateFloorLeft(value) => {
            ChoreographySettingsAction::UpdateFloorLeft(value)
        }
        ChoreographySettingsCommand::UpdateFloorRight(value) => {
            ChoreographySettingsAction::UpdateFloorRight(value)
        }
        ChoreographySettingsCommand::UpdateGridResolution(value) => {
            ChoreographySettingsAction::UpdateGridResolution(value)
        }
        ChoreographySettingsCommand::UpdateDrawPathFrom(value) => {
            ChoreographySettingsAction::UpdateDrawPathFrom(value)
        }
        ChoreographySettingsCommand::UpdateDrawPathTo(value) => {
            ChoreographySettingsAction::UpdateDrawPathTo(value)
        }
        ChoreographySettingsCommand::UpdateGridLines(value) => {
            ChoreographySettingsAction::UpdateGridLines(value)
        }
        ChoreographySettingsCommand::UpdateSnapToGrid(value) => {
            ChoreographySettingsAction::UpdateSnapToGrid(value)
        }
        ChoreographySettingsCommand::UpdateShowTimestamps(value) => {
            ChoreographySettingsAction::UpdateShowTimestamps(value)
        }
        ChoreographySettingsCommand::UpdateShowLegend(value) => {
            ChoreographySettingsAction::UpdateShowLegend(value)
        }
        ChoreographySettingsCommand::UpdatePositionsAtSide(value) => {
            ChoreographySettingsAction::UpdatePositionsAtSide(value)
        }
        ChoreographySettingsCommand::UpdateTransparency(value) => {
            ChoreographySettingsAction::UpdateTransparency(value)
        }
        ChoreographySettingsCommand::UpdateFloorColor(value) => {
            ChoreographySettingsAction::UpdateFloorColor(value)
        }
        ChoreographySettingsCommand::UpdateSelectedScene(command) => {
            ChoreographySettingsAction::UpdateSelectedScene(from_selected_scene_command(command))
        }
        ChoreographySettingsCommand::ClearEphemeralOutputs => {
            ChoreographySettingsAction::ClearEphemeralOutputs
        }
    }
}

#[must_use]
pub fn from_selected_scene_command(command: UpdateSelectedSceneCommand) -> UpdateSelectedSceneAction {
    match command {
        UpdateSelectedSceneCommand::SyncFromSelected => UpdateSelectedSceneAction::SyncFromSelected,
        UpdateSelectedSceneCommand::SceneName(value) => UpdateSelectedSceneAction::SceneName(value),
        UpdateSelectedSceneCommand::SceneText(value) => UpdateSelectedSceneAction::SceneText(value),
        UpdateSelectedSceneCommand::SceneFixedPositions(value) => {
            UpdateSelectedSceneAction::SceneFixedPositions(value)
        }
        UpdateSelectedSceneCommand::SceneColor(value) => UpdateSelectedSceneAction::SceneColor(value),
        UpdateSelectedSceneCommand::SceneTimestamp {
            has_timestamp,
            seconds,
        } => UpdateSelectedSceneAction::SceneTimestamp {
            has_timestamp,
            seconds,
        },
    }
}
