use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use crossbeam_channel::{Receiver, Sender};
use slint::{ComponentFactory, ComponentHandle, ModelRc, SharedString, Timer, TimerMode, VecModel};

use choreo_state_machine::ApplicationStateMachine;

use crate::audio_player::{
    AudioPlayerViewModel,
    HapticFeedback,
    OpenAudioFileCommand,
};
use crate::global::GlobalStateModel;
use crate::preferences::Preferences;
use crate::{MenuItem, ShellHost};

use super::{
    build_main_behaviors,
    build_main_view_model,
    CloseDialogCommand,
    InteractionModeOption,
    MainBehaviorDependencies,
    MainBehaviors,
    MainDependencies,
    MainViewModel,
    OpenSvgFileCommand,
    ShowDialogCommand,
};

#[derive(Clone, Default)]
pub struct MainPageActionHandlers {
    pub pick_audio_path: Option<Rc<dyn Fn() -> Option<String>>>,
    pub pick_image_path: Option<Rc<dyn Fn() -> Option<String>>>,
}

pub struct MainPageDependencies<P: Preferences> {
    pub global_state: Rc<RefCell<GlobalStateModel>>,
    pub state_machine: Rc<RefCell<ApplicationStateMachine>>,
    pub audio_player: AudioPlayerViewModel,
    pub haptic_feedback: Option<Box<dyn HapticFeedback>>,
    pub open_audio_sender: Sender<OpenAudioFileCommand>,
    pub open_svg_sender: Sender<OpenSvgFileCommand>,
    pub open_svg_receiver: Receiver<OpenSvgFileCommand>,
    pub show_dialog_receiver: Receiver<ShowDialogCommand>,
    pub close_dialog_receiver: Receiver<CloseDialogCommand>,
    pub preferences: P,
    pub actions: MainPageActionHandlers,
}

pub struct MainPageBinding<P: Preferences> {
    view: ShellHost,
    view_model: Rc<RefCell<MainViewModel>>,
    behaviors: Rc<MainBehaviors<P>>,
    _timer: Timer,
}

impl<P: Preferences + 'static> MainPageBinding<P> {
    pub fn new(view: ShellHost, deps: MainPageDependencies<P>) -> Self {
        let view_model = Rc::new(RefCell::new(build_main_view_model(MainDependencies {
            global_state: deps.global_state.clone(),
            state_machine: deps.state_machine.clone(),
            audio_player: deps.audio_player,
            haptic_feedback: deps.haptic_feedback,
        })));

        let behaviors = Rc::new(build_main_behaviors(MainBehaviorDependencies {
            global_state: deps.global_state,
            state_machine: deps.state_machine,
            open_audio_sender: deps.open_audio_sender,
            open_svg_sender: deps.open_svg_sender,
            open_svg_receiver: deps.open_svg_receiver,
            show_dialog_receiver: deps.show_dialog_receiver,
            close_dialog_receiver: deps.close_dialog_receiver,
            preferences: deps.preferences,
        }));

        apply_view_model(&view, &view_model.borrow());

        let actions = deps.actions;
        let view_weak = view.as_weak();

        {
            let view_model = Rc::clone(&view_model);
            let view_weak = view_weak.clone();
            view.on_toggle_nav(move || {
                let mut view_model = view_model.borrow_mut();
                view_model.toggle_navigation();
                if let Some(view) = view_weak.upgrade() {
                    apply_view_model(&view, &view_model);
                }
            });
        }

        {
            let view_model = Rc::clone(&view_model);
            let behaviors = Rc::clone(&behaviors);
            let view_weak = view_weak.clone();
            let actions = actions.clone();
            view.on_open_audio(move || {
                let mut view_model = view_model.borrow_mut();
                view_model.open_audio();
                if let Some(picker) = actions.pick_audio_path.as_ref()
                    && let Some(path) = picker()
                {
                    behaviors.open_audio.open_audio(&mut view_model, path);
                }
                if let Some(view) = view_weak.upgrade() {
                    apply_view_model(&view, &view_model);
                }
            });
        }

        {
            let view_model = Rc::clone(&view_model);
            let behaviors = Rc::clone(&behaviors);
            let view_weak = view_weak.clone();
            let actions = actions.clone();
            view.on_open_image(move || {
                let view_model = view_model.borrow();
                view_model.open_image();
                if let Some(picker) = actions.pick_image_path.as_ref()
                    && let Some(path) = picker()
                {
                    behaviors.open_image.open_svg(path);
                }
                if let Some(view) = view_weak.upgrade() {
                    apply_view_model(&view, &view_model);
                }
            });
        }

        {
            let view_model = Rc::clone(&view_model);
            let view_weak = view_weak.clone();
            view.on_open_settings(move || {
                let mut view_model = view_model.borrow_mut();
                view_model.open_choreography_settings();
                if let Some(view) = view_weak.upgrade() {
                    apply_view_model(&view, &view_model);
                }
            });
        }

        {
            let view_model = Rc::clone(&view_model);
            let behaviors = Rc::clone(&behaviors);
            let view_weak = view_weak.clone();
            view.on_select_mode(move |index| {
                let mut view_model = view_model.borrow_mut();
                let Some(option) = view_model.mode_options.get(index as usize).cloned() else {
                    return;
                };

                view_model.set_selected_mode(option.clone());
                behaviors.apply_interaction_mode.apply_mode(option.mode);

                if let Some(view) = view_weak.upgrade() {
                    apply_view_model(&view, &view_model);
                }
            });
        }

        {
            let view_model = Rc::clone(&view_model);
            let view_weak = view_weak.clone();
            view.on_close_dialog(move || {
                let mut view_model = view_model.borrow_mut();
                view_model.is_dialog_open = false;
                view_model.dialog_content = None;
                if let Some(view) = view_weak.upgrade() {
                    apply_view_model(&view, &view_model);
                }
            });
        }

        let timer = Timer::default();
        {
            let view_model = Rc::clone(&view_model);
            let behaviors = Rc::clone(&behaviors);
            let view_weak = view_weak.clone();
            timer.start(TimerMode::Repeated, Duration::from_millis(50), move || {
                let mut view_model = view_model.borrow_mut();
                let mut needs_update = false;

                if behaviors.show_dialog.try_handle(&mut view_model) {
                    needs_update = true;
                }

                if behaviors.hide_dialog.try_handle(&mut view_model) {
                    needs_update = true;
                }

                if behaviors.open_svg_file.try_handle() {
                    needs_update = true;
                }

                if needs_update
                    && let Some(view) = view_weak.upgrade()
                {
                    apply_view_model(&view, &view_model);
                }
            });
        }

        Self {
            view,
            view_model,
            behaviors,
            _timer: timer,
        }
    }

    pub fn view(&self) -> &ShellHost {
        &self.view
    }

    pub fn view_model(&self) -> Rc<RefCell<MainViewModel>> {
        Rc::clone(&self.view_model)
    }

    pub fn behaviors(&self) -> Rc<MainBehaviors<P>> {
        Rc::clone(&self.behaviors)
    }
}

fn apply_view_model(view: &ShellHost, view_model: &MainViewModel) {
    view.set_mode_items(build_mode_items(&view_model.mode_options));
    view.set_selected_mode_index(selected_mode_index(view_model));
    view.set_is_mode_selection_enabled(view_model.is_mode_selection_enabled);
    view.set_nav_width(view_model.nav_width);
    view.set_is_nav_open(view_model.is_nav_open);
    view.set_is_choreography_settings_open(view_model.is_choreography_settings_open);
    view.set_is_audio_player_open(view_model.is_audio_player_open);
    view.set_is_dialog_open(view_model.is_dialog_open);

    view.set_dialog_content(ComponentFactory::default());
}

fn build_mode_items(options: &[InteractionModeOption]) -> ModelRc<MenuItem> {
    let items = options
        .iter()
        .map(|option| MenuItem {
            icon: slint::Image::default(),
            text: SharedString::from(option.label.as_str()),
            trailing_text: SharedString::new(),
            enabled: true,
        })
        .collect::<Vec<_>>();

    ModelRc::new(VecModel::from(items))
}

fn selected_mode_index(view_model: &MainViewModel) -> i32 {
    view_model
        .mode_options
        .iter()
        .position(|option| option.mode == view_model.selected_mode_option.mode)
        .map(|index| index as i32)
        .unwrap_or(-1)
}

