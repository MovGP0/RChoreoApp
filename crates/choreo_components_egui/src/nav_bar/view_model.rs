use super::actions::NavBarAction;
use super::messages::NavBarSenders;
use super::reducer::reduce;
use super::runtime::NavBarRuntimeHandlers;
use super::runtime::dispatch_effects;
use super::state::NavBarState;

pub trait NavBarHapticFeedback {
    fn is_supported(&self) -> bool;
    fn perform_click(&self);
}

pub struct NavBarViewModel {
    pub state: NavBarState,
    runtime_handlers: NavBarRuntimeHandlers,
    haptic_feedback: Option<Box<dyn NavBarHapticFeedback>>,
}

impl Default for NavBarViewModel {
    fn default() -> Self {
        Self::new(NavBarState::default(), NavBarRuntimeHandlers::default(), None)
    }
}

impl NavBarViewModel {
    #[must_use]
    pub fn new(
        state: NavBarState,
        runtime_handlers: NavBarRuntimeHandlers,
        haptic_feedback: Option<Box<dyn NavBarHapticFeedback>>,
    ) -> Self {
        Self {
            state,
            runtime_handlers,
            haptic_feedback,
        }
    }

    #[must_use]
    pub fn with_senders(senders: NavBarSenders) -> Self {
        let mut runtime_handlers = NavBarRuntimeHandlers::default();
        let open_audio_sender = senders.open_audio_requested;
        runtime_handlers.open_audio_requested = Some(Box::new(move |command| {
            let _ = open_audio_sender.send(command);
        }));

        let open_image_sender = senders.open_image_requested;
        runtime_handlers.open_image_requested = Some(Box::new(move |command| {
            let _ = open_image_sender.send(command);
        }));

        let reset_sender = senders.reset_floor_viewport_requested;
        runtime_handlers.reset_floor_viewport_requested = Some(Box::new(move |command| {
            let _ = reset_sender.send(command);
        }));

        let mode_sender = senders.interaction_mode_changed;
        runtime_handlers.interaction_mode_changed = Some(Box::new(move |command| {
            let _ = mode_sender.send(command);
        }));

        Self::new(NavBarState::default(), runtime_handlers, None)
    }

    pub fn dispatch(&mut self, action: NavBarAction) {
        if should_emit_click_feedback(&action) {
            self.perform_click();
        }
        let effects = reduce(&mut self.state, action);
        dispatch_effects(effects, &mut self.runtime_handlers);
    }

    fn perform_click(&self) {
        if let Some(haptic) = &self.haptic_feedback
            && haptic.is_supported()
        {
            haptic.perform_click();
        }
    }
}

fn should_emit_click_feedback(action: &NavBarAction) -> bool {
    matches!(
        action,
        NavBarAction::OpenAudio
            | NavBarAction::OpenImage
            | NavBarAction::ToggleChoreographySettings
            | NavBarAction::ResetFloorViewport
    )
}
