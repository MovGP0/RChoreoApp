use crate::behavior::Behavior;
use crate::behavior::CompositeDisposable;
use nject::injectable;

use super::actions::ChoreographySettingsAction;
use super::actions::UpdateSelectedSceneAction;
use super::messages::ChoreographySettingsCommand;
use super::messages::UpdateSelectedSceneCommand;
use super::reducer;
use super::state::ChoreographySettingsState;

#[injectable]
#[inject(|behaviors: Vec<Box<dyn Behavior<ChoreographySettingsViewModel>>>| Self::new(behaviors))]
pub struct ChoreographySettingsViewModel {
    pub state: ChoreographySettingsState,
    disposables: CompositeDisposable,
}

impl ChoreographySettingsViewModel {
    pub fn new(behaviors: Vec<Box<dyn Behavior<ChoreographySettingsViewModel>>>) -> Self {
        let mut view_model = Self {
            state: ChoreographySettingsState::default(),
            disposables: CompositeDisposable::new(),
        };
        let mut disposables = CompositeDisposable::new();
        for behavior in &behaviors {
            behavior.activate(&mut view_model, &mut disposables);
        }
        view_model.disposables = disposables;
        view_model
    }

    pub fn dispatch(&mut self, command: ChoreographySettingsCommand) {
        reducer::reduce(&mut self.state, map_command(command));
    }
}

impl Drop for ChoreographySettingsViewModel {
    fn drop(&mut self) {
        self.disposables.dispose_all();
    }
}

impl Default for ChoreographySettingsViewModel {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

#[must_use]
pub fn map_command(command: ChoreographySettingsCommand) -> ChoreographySettingsAction {
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
            ChoreographySettingsAction::UpdateSelectedScene(map_selected_scene_command(command))
        }
        ChoreographySettingsCommand::ClearEphemeralOutputs => {
            ChoreographySettingsAction::ClearEphemeralOutputs
        }
    }
}

#[must_use]
pub fn map_selected_scene_command(command: UpdateSelectedSceneCommand) -> UpdateSelectedSceneAction {
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
