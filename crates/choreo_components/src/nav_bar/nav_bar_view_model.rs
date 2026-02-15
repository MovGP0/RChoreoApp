use std::cell::RefCell;
use std::rc::Rc;

use nject::injectable;

use crate::behavior::{Behavior, CompositeDisposable};
use crate::global::{GlobalStateModel, InteractionMode};
use crate::haptics::HapticFeedback;

use super::{
    InteractionModeChangedCommand, NavBarSenders, OpenAudioRequestedCommand,
    OpenImageRequestedCommand, ResetFloorViewportRequestedCommand,
};

const MODE_OPTIONS: [InteractionMode; 6] = [
    InteractionMode::View,
    InteractionMode::Move,
    InteractionMode::RotateAroundCenter,
    InteractionMode::RotateAroundDancer,
    InteractionMode::Scale,
    InteractionMode::LineOfSight,
];

#[injectable]
#[inject(
    |global_state: Rc<RefCell<GlobalStateModel>>,
     haptic_feedback: Option<Box<dyn HapticFeedback>>,
     behaviors: Vec<Box<dyn Behavior<NavBarViewModel>>>,
     senders: NavBarSenders| {
        Self::new(global_state, haptic_feedback, behaviors, senders)
    }
)]
pub struct NavBarViewModel {
    global_state: Rc<RefCell<GlobalStateModel>>,
    haptic_feedback: Option<Box<dyn HapticFeedback>>,
    senders: NavBarSenders,
    on_change: Option<Rc<dyn Fn()>>,
    disposables: CompositeDisposable,
    pub selected_mode: InteractionMode,
    pub is_mode_selection_enabled: bool,
    pub nav_width: f32,
    pub is_nav_open: bool,
    pub is_audio_player_open: bool,
    pub is_choreography_settings_open: bool,
}

impl NavBarViewModel {
    pub const DEFAULT_NAV_WIDTH: f32 = 280.0;

    pub fn new(
        global_state: Rc<RefCell<GlobalStateModel>>,
        haptic_feedback: Option<Box<dyn HapticFeedback>>,
        behaviors: Vec<Box<dyn Behavior<NavBarViewModel>>>,
        senders: NavBarSenders,
    ) -> Self {
        let selected_mode = global_state.borrow().interaction_mode;
        let mut view_model = Self {
            global_state,
            haptic_feedback,
            senders,
            on_change: None,
            disposables: CompositeDisposable::new(),
            selected_mode,
            is_mode_selection_enabled: true,
            nav_width: 0.0,
            is_nav_open: false,
            is_audio_player_open: false,
            is_choreography_settings_open: false,
        };

        let mut disposables = CompositeDisposable::new();
        for behavior in behaviors.iter() {
            behavior.activate(&mut view_model, &mut disposables);
        }

        view_model.disposables = disposables;
        view_model
    }

    pub fn set_on_change(&mut self, handler: Option<Rc<dyn Fn()>>) {
        self.on_change = handler;
    }

    pub fn set_mode_selection_enabled(&mut self, enabled: bool) {
        if self.is_mode_selection_enabled == enabled {
            return;
        }

        self.is_mode_selection_enabled = enabled;
        self.notify_changed();
    }

    pub fn open_audio(&mut self) {
        self.perform_click();
        let _ = self
            .senders
            .open_audio_requested
            .try_send(OpenAudioRequestedCommand);
    }

    pub fn open_image(&self) {
        self.perform_click();
        let _ = self
            .senders
            .open_image_requested
            .try_send(OpenImageRequestedCommand);
    }

    pub fn open_choreography_settings(&mut self) {
        self.perform_click();
        self.is_choreography_settings_open = !self.is_choreography_settings_open;
        self.notify_changed();
    }

    pub fn reset_floor_viewport(&self) {
        self.perform_click();
        let _ = self
            .senders
            .reset_floor_viewport_requested
            .try_send(ResetFloorViewportRequestedCommand);
    }

    pub fn close_choreography_settings(&mut self) {
        if !self.is_choreography_settings_open {
            return;
        }

        self.is_choreography_settings_open = false;
        self.notify_changed();
    }

    pub fn toggle_navigation(&mut self) {
        self.is_nav_open = !self.is_nav_open;
        self.nav_width = if self.is_nav_open {
            Self::DEFAULT_NAV_WIDTH
        } else {
            0.0
        };
        self.notify_changed();
    }

    pub fn close_navigation(&mut self) {
        if !self.is_nav_open {
            return;
        }

        self.is_nav_open = false;
        self.nav_width = 0.0;
        self.notify_changed();
    }

    pub fn set_selected_mode(&mut self, mode: InteractionMode) {
        self.selected_mode = mode;
        self.global_state.borrow_mut().interaction_mode = mode;
        let _ = self
            .senders
            .interaction_mode_changed
            .try_send(InteractionModeChangedCommand { mode });
        self.notify_changed();
    }

    pub fn set_audio_player_opened(&mut self, is_open: bool) {
        if self.is_audio_player_open == is_open {
            return;
        }

        self.is_audio_player_open = is_open;
        self.notify_changed();
    }

    fn perform_click(&self) {
        if let Some(haptic) = &self.haptic_feedback
            && haptic.is_supported()
        {
            haptic.perform_click();
        }
    }

    fn notify_changed(&self) {
        if let Some(handler) = self.on_change.as_ref() {
            handler();
        }
    }
}

impl Drop for NavBarViewModel {
    fn drop(&mut self) {
        self.disposables.dispose_all();
    }
}

pub fn mode_option_from_index(index: i32) -> Option<InteractionMode> {
    if index < 0 {
        return None;
    }

    let index = index as usize;
    MODE_OPTIONS.get(index).copied()
}

pub fn mode_index(mode: InteractionMode) -> i32 {
    MODE_OPTIONS
        .iter()
        .position(|option| *option == mode)
        .map(|index| index as i32)
        .unwrap_or(-1)
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use crossbeam_channel::unbounded;

    use crate::global::GlobalStateModel;
    use crate::nav_bar::NavBarSenders;
    use crate::nav_bar::NavBarViewModel;

    #[test]
    fn reset_floor_viewport_sends_command() {
        let (open_audio_sender, _open_audio_receiver) = unbounded();
        let (open_image_sender, _open_image_receiver) = unbounded();
        let (reset_sender, reset_receiver) = unbounded();
        let (mode_sender, _mode_receiver) = unbounded();
        let senders = NavBarSenders {
            open_audio_requested: open_audio_sender,
            open_image_requested: open_image_sender,
            reset_floor_viewport_requested: reset_sender,
            interaction_mode_changed: mode_sender,
        };
        let global_state = Rc::new(RefCell::new(GlobalStateModel::default()));
        let view_model = NavBarViewModel::new(global_state, None, Vec::new(), senders);

        view_model.reset_floor_viewport();

        assert!(
            reset_receiver.try_recv().is_ok(),
            "reset floor viewport command should be emitted",
        );
    }
}
