use choreo_master_mobile_json::SceneId;

use super::scenes_view_model::SceneViewModel;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DialogRequest {
    DeleteScene { scene_id: SceneId },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShowDialogCommand {
    pub dialog: DialogRequest,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CloseDialogCommand;

#[derive(Debug, Clone, PartialEq)]
pub struct SceneSelectedEvent {
    pub selected_scene: SceneViewModel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CopyScenePositionsDecision {
    CopyPositions,
    KeepPositions,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CopyScenePositionsDecisionEvent {
    pub decision: CopyScenePositionsDecision,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SelectedSceneChangedEvent {
    pub selected_scene: Option<SceneViewModel>,
}
