use std::sync::mpsc::Sender;

use super::state::InteractionMode;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenAudioRequestedCommand;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenImageRequestedCommand;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResetFloorViewportRequestedCommand;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InteractionModeChangedCommand {
    pub mode: InteractionMode,
}

#[derive(Clone)]
pub struct NavBarSenders {
    pub open_audio_requested: Sender<OpenAudioRequestedCommand>,
    pub open_image_requested: Sender<OpenImageRequestedCommand>,
    pub reset_floor_viewport_requested: Sender<ResetFloorViewportRequestedCommand>,
    pub interaction_mode_changed: Sender<InteractionModeChangedCommand>,
}
