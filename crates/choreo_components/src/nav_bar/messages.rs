use crossbeam_channel::Sender;

use crate::global::InteractionMode;

#[derive(Debug, Clone, PartialEq)]
pub struct OpenAudioRequestedCommand;

#[derive(Debug, Clone, PartialEq)]
pub struct OpenImageRequestedCommand;

#[derive(Debug, Clone, PartialEq)]
pub struct InteractionModeChangedCommand {
    pub mode: InteractionMode,
}

#[derive(Clone)]
pub struct NavBarSenders {
    pub open_audio_requested: Sender<OpenAudioRequestedCommand>,
    pub open_image_requested: Sender<OpenImageRequestedCommand>,
    pub interaction_mode_changed: Sender<InteractionModeChangedCommand>,
}
