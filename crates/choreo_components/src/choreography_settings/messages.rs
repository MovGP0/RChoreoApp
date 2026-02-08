use choreo_master_mobile_json::Color;
use time::Date;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedrawFloorCommand;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShowTimestampsChangedEvent {
    pub is_enabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReloadChoreographySettingsCommand;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReloadSettingsPreferencesCommand;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateCommentCommand {
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateNameCommand {
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateSubtitleCommand {
    pub value: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UpdateDateCommand {
    pub value: Date,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateVariationCommand {
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateAuthorCommand {
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateDescriptionCommand {
    pub value: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UpdateFloorFrontCommand {
    pub value: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UpdateFloorBackCommand {
    pub value: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UpdateFloorLeftCommand {
    pub value: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UpdateFloorRightCommand {
    pub value: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UpdateGridResolutionCommand {
    pub value: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UpdateDrawPathFromCommand {
    pub value: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UpdateDrawPathToCommand {
    pub value: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UpdateGridLinesCommand {
    pub value: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UpdateSnapToGridCommand {
    pub value: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UpdateShowTimestampsCommand {
    pub value: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UpdateShowLegendCommand {
    pub value: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UpdatePositionsAtSideCommand {
    pub value: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UpdateTransparencyCommand {
    pub value: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UpdateFloorColorCommand {
    pub value: Color,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UpdateSelectedSceneCommand {
    SyncFromSelected,
    SceneName(String),
    SceneText(String),
    SceneFixedPositions(bool),
    SceneColor(Color),
    SceneTimestamp { has_timestamp: bool, seconds: f64 },
}
