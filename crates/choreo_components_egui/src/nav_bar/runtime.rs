use super::messages::InteractionModeChangedCommand;
use super::messages::OpenAudioRequestedCommand;
use super::messages::OpenImageRequestedCommand;
use super::messages::ResetFloorViewportRequestedCommand;
use super::reducer::NavBarEffect;

#[derive(Default)]
pub struct NavBarRuntimeHandlers {
    pub open_audio_requested: Option<Box<dyn FnMut(OpenAudioRequestedCommand)>>,
    pub open_image_requested: Option<Box<dyn FnMut(OpenImageRequestedCommand)>>,
    pub reset_floor_viewport_requested: Option<Box<dyn FnMut(ResetFloorViewportRequestedCommand)>>,
    pub interaction_mode_changed: Option<Box<dyn FnMut(InteractionModeChangedCommand)>>,
}

pub fn dispatch_effect(effect: NavBarEffect, handlers: &mut NavBarRuntimeHandlers) {
    match effect {
        NavBarEffect::OpenAudioRequested => {
            if let Some(handler) = handlers.open_audio_requested.as_mut() {
                handler(OpenAudioRequestedCommand);
            }
        }
        NavBarEffect::OpenImageRequested => {
            if let Some(handler) = handlers.open_image_requested.as_mut() {
                handler(OpenImageRequestedCommand);
            }
        }
        NavBarEffect::ResetFloorViewportRequested => {
            if let Some(handler) = handlers.reset_floor_viewport_requested.as_mut() {
                handler(ResetFloorViewportRequestedCommand);
            }
        }
        NavBarEffect::InteractionModeChanged { mode } => {
            if let Some(handler) = handlers.interaction_mode_changed.as_mut() {
                handler(InteractionModeChangedCommand { mode });
            }
        }
    }
}

pub fn dispatch_effects(effects: Vec<NavBarEffect>, handlers: &mut NavBarRuntimeHandlers) {
    for effect in effects {
        dispatch_effect(effect, handlers);
    }
}
