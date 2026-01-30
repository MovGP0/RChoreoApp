use std::cell::RefCell;
use std::rc::{Rc, Weak};

use choreo_state_machine::ApplicationStateMachine;
use nject::injectable;

use crate::audio_player::AudioPlayerViewModel;
use crate::haptics::HapticFeedback;
use crate::behavior::{Behavior, CompositeDisposable};
use crate::global::{GlobalStateModel, InteractionMode};
use crate::scenes::SceneViewModel;

#[derive(Clone, Default)]
#[injectable]
#[inject(|| Self::default())]
pub struct MainViewModelActions {
    pub open_audio_requested: Option<Rc<dyn Fn() -> bool>>,
    pub open_image_requested: Option<Rc<dyn Fn()>>,
    pub interaction_mode_changed: Option<Rc<dyn Fn(InteractionMode)>>,
}

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
     state_machine: Rc<RefCell<ApplicationStateMachine>>,
     audio_player: Rc<RefCell<AudioPlayerViewModel>>,
     haptic_feedback: Option<Box<dyn HapticFeedback>>,
     behaviors: Vec<Box<dyn Behavior<MainViewModel>>>,
     actions: MainViewModelActions| {
        Self::new(
            global_state,
            state_machine,
            audio_player,
            haptic_feedback,
            behaviors,
            actions,
        )
    }
)]
pub struct MainViewModel {
    disposables: CompositeDisposable,
    global_state: Rc<RefCell<GlobalStateModel>>,
    _state_machine: Rc<RefCell<ApplicationStateMachine>>,
    haptic_feedback: Option<Box<dyn HapticFeedback>>,
    behaviors: Vec<Box<dyn Behavior<MainViewModel>>>,
    actions: MainViewModelActions,
    on_change: Option<Rc<dyn Fn()>>,
    self_handle: Option<Weak<RefCell<MainViewModel>>>,
    pub audio_player: Rc<RefCell<AudioPlayerViewModel>>,
    pub selected_mode: InteractionMode,
    pub is_mode_selection_enabled: bool,
    pub nav_width: f32,
    pub is_nav_open: bool,
    pub is_audio_player_open: bool,
    pub is_choreography_settings_open: bool,
    pub is_dialog_open: bool,
    pub dialog_content: Option<String>,
}

impl MainViewModel {
    pub const DEFAULT_NAV_WIDTH: f32 = 280.0;

    pub fn new(
        global_state: Rc<RefCell<GlobalStateModel>>,
        state_machine: Rc<RefCell<ApplicationStateMachine>>,
        audio_player: Rc<RefCell<AudioPlayerViewModel>>,
        haptic_feedback: Option<Box<dyn HapticFeedback>>,
        behaviors: Vec<Box<dyn Behavior<MainViewModel>>>,
        actions: MainViewModelActions,
    ) -> Self
    {
        let selected_mode = global_state.borrow().interaction_mode;

        Self {
            disposables: CompositeDisposable::new(),
            global_state,
            _state_machine: state_machine,
            haptic_feedback,
            behaviors,
            actions,
            on_change: None,
            self_handle: None,
            audio_player,
            selected_mode,
            is_mode_selection_enabled: true,
            nav_width: 0.0,
            is_nav_open: false,
            is_audio_player_open: false,
            is_choreography_settings_open: false,
            is_dialog_open: false,
            dialog_content: None,
        }
    }

    pub fn activate(view_model: &Rc<RefCell<MainViewModel>>) {
        let mut disposables = CompositeDisposable::new();
        {
            let mut view_model = view_model.borrow_mut();
            let behaviors = std::mem::take(&mut view_model.behaviors);
            for behavior in behaviors {
                behavior.activate(&mut view_model, &mut disposables);
            }
        }

        view_model.borrow_mut().disposables = disposables;
    }

    pub fn open_audio(&mut self) {
        self.perform_click();
        let opened = self
            .actions
            .open_audio_requested
            .as_ref()
            .map(|handler| handler())
            .unwrap_or(false);

        if opened {
            self.is_audio_player_open = true;
            self.notify_changed();
        }
    }

    pub fn open_image(&self) {
        self.perform_click();
        if let Some(handler) = self.actions.open_image_requested.as_ref() {
            handler();
        }
    }

    pub fn open_choreography_settings(&mut self) {
        self.perform_click();
        self.is_choreography_settings_open = !self.is_choreography_settings_open;
        self.notify_changed();
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

    pub fn update_place_mode(&mut self, scene: Option<&SceneViewModel>) {
        let mut global_state = self.global_state.borrow_mut();
        let Some(scene) = scene else {
            global_state.is_place_mode = false;
            self.is_mode_selection_enabled = true;
            return;
        };

        let dancer_count = global_state.choreography.dancers.len();
        let position_count = global_state
            .choreography
            .scenes
            .iter()
            .find(|model| model.scene_id == scene.scene_id)
            .map(|model| model.positions.len())
            .unwrap_or_default();

        global_state.is_place_mode = dancer_count > 0 && position_count < dancer_count;
        self.is_mode_selection_enabled = !global_state.is_place_mode;
        self.notify_changed();
    }

    pub fn set_selected_mode(&mut self, mode: InteractionMode) {
        self.selected_mode = mode;
        self.global_state.borrow_mut().interaction_mode = mode;
        if let Some(handler) = self.actions.interaction_mode_changed.as_ref() {
            handler(mode);
        }
        self.notify_changed();
    }

    pub fn show_dialog(&mut self, content: Option<String>) {
        self.dialog_content = content;
        self.is_dialog_open = self.dialog_content.is_some();
        self.notify_changed();
    }

    pub fn hide_dialog(&mut self) {
        self.is_dialog_open = false;
        self.dialog_content = None;
        self.notify_changed();
    }

    pub fn set_actions(&mut self, actions: MainViewModelActions) {
        self.actions = actions;
    }

    pub fn set_on_change(&mut self, handler: Option<Rc<dyn Fn()>>) {
        self.on_change = handler;
    }

    pub fn set_self_handle(&mut self, handle: Weak<RefCell<MainViewModel>>) {
        self.self_handle = Some(handle);
    }

    pub fn self_handle(&self) -> Option<Weak<RefCell<MainViewModel>>> {
        self.self_handle.clone()
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

impl Drop for MainViewModel {
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
