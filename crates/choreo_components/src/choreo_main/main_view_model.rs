use std::cell::RefCell;
use std::rc::{Rc, Weak};

use choreo_state_machine::ApplicationStateMachine;
use nject::injectable;

use crate::behavior::{Behavior, CompositeDisposable};
use crate::global::GlobalStateModel;
use crate::nav_bar::NavBarViewModel;
use crate::scenes::SceneViewModel;

#[injectable]
#[inject(
    |global_state: Rc<RefCell<GlobalStateModel>>,
     state_machine: Rc<RefCell<ApplicationStateMachine>>,
     behaviors: Vec<Box<dyn Behavior<MainViewModel>>>,
     nav_bar: Rc<RefCell<NavBarViewModel>>| {
        Self::new(
            global_state,
            state_machine,
            behaviors,
            nav_bar,
        )
    }
)]
pub struct MainViewModel {
    disposables: CompositeDisposable,
    global_state: Rc<RefCell<GlobalStateModel>>,
    _state_machine: Rc<RefCell<ApplicationStateMachine>>,
    behaviors: Vec<Box<dyn Behavior<MainViewModel>>>,
    on_change: Option<Rc<dyn Fn()>>,
    self_handle: Option<Weak<RefCell<MainViewModel>>>,
    pub nav_bar: Rc<RefCell<NavBarViewModel>>,
    pub is_dialog_open: bool,
    pub dialog_content: Option<String>,
}

impl MainViewModel {
    pub fn new(
        global_state: Rc<RefCell<GlobalStateModel>>,
        state_machine: Rc<RefCell<ApplicationStateMachine>>,
        behaviors: Vec<Box<dyn Behavior<MainViewModel>>>,
        nav_bar: Rc<RefCell<NavBarViewModel>>,
    ) -> Self
    {
        Self {
            disposables: CompositeDisposable::new(),
            global_state,
            _state_machine: state_machine,
            behaviors,
            on_change: None,
            self_handle: None,
            nav_bar,
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

    pub fn update_place_mode(&mut self, scene: Option<&SceneViewModel>) {
        let mut global_state = self.global_state.borrow_mut();
        let Some(scene) = scene else {
            global_state.is_place_mode = false;
            self.nav_bar
                .borrow_mut()
                .set_mode_selection_enabled(true);
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
        self.nav_bar
            .borrow_mut()
            .set_mode_selection_enabled(!global_state.is_place_mode);
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

    pub fn set_on_change(&mut self, handler: Option<Rc<dyn Fn()>>) {
        self.nav_bar.borrow_mut().set_on_change(handler.clone());
        self.on_change = handler;
    }

    pub fn set_self_handle(&mut self, handle: Weak<RefCell<MainViewModel>>) {
        self.self_handle = Some(handle);
    }

    pub fn self_handle(&self) -> Option<Weak<RefCell<MainViewModel>>> {
        self.self_handle.clone()
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
