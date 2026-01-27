use std::cell::RefCell;
use std::rc::Rc;
use crossbeam_channel::{Receiver, Sender};
use slint::{ComponentFactory, ComponentHandle, ModelRc, SharedString, VecModel};

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
    MainViewModelActions,
    MainViewModel,
    OpenSvgFileCommand,
    ShowDialogCommand,
};

#[derive(Clone, Default)]
pub struct MainPageActionHandlers {
    pub pick_audio_path: Option<Rc<dyn Fn() -> Option<String>>>,
    pub pick_image_path: Option<Rc<dyn Fn() -> Option<String>>>,
}

pub struct MainPageDependencies<P: Preferences + Clone> {
    pub global_state: Rc<RefCell<GlobalStateModel>>,
    pub state_machine: Rc<RefCell<ApplicationStateMachine>>,
    pub audio_player: AudioPlayerViewModel,
    pub locale: String,
    pub haptic_feedback: Option<Box<dyn HapticFeedback>>,
    pub open_audio_sender: Sender<OpenAudioFileCommand>,
    pub open_svg_sender: Sender<OpenSvgFileCommand>,
    pub open_svg_receiver: Receiver<OpenSvgFileCommand>,
    pub show_dialog_receiver: Receiver<ShowDialogCommand>,
    pub close_dialog_receiver: Receiver<CloseDialogCommand>,
    pub preferences: P,
    pub actions: MainPageActionHandlers,
}

pub struct MainPageBinding<P: Preferences + Clone> {
    view: ShellHost,
    view_model: Rc<RefCell<MainViewModel>>,
    behaviors: Rc<MainBehaviors<P>>,
}

impl<P: Preferences + Clone + 'static> MainPageBinding<P> {
    pub fn new(view: ShellHost, deps: MainPageDependencies<P>) -> Self {
        let locale = deps.locale.clone();
        let view_model = Rc::new(RefCell::new(build_main_view_model(MainDependencies {
            global_state: deps.global_state.clone(),
            state_machine: deps.state_machine.clone(),
            audio_player: deps.audio_player,
            locale,
            haptic_feedback: deps.haptic_feedback,
            actions: MainViewModelActions::default(),
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

        let actions = deps.actions;
        let view_weak = view.as_weak();

        {
            let view_model_for_change = Rc::clone(&view_model);
            let view_weak = view_weak.clone();
            let on_change = Rc::new(move || {
                let view_model = Rc::clone(&view_model_for_change);
                let view_weak = view_weak.clone();
                slint::Timer::single_shot(std::time::Duration::from_millis(0), move || {
                    if let Some(view) = view_weak.upgrade() {
                        let view_model = view_model.borrow();
                        apply_view_model(&view, &view_model);
                    }
                });
            });
            view_model.borrow_mut().set_on_change(Some(on_change));
        }

        {
            let view_model = Rc::clone(&view_model);
            let actions_for_audio = actions.clone();
            let actions_for_image = actions.clone();
            let behaviors_for_audio = Rc::clone(&behaviors);
            let behaviors_for_image = Rc::clone(&behaviors);
            let behaviors_for_mode = Rc::clone(&behaviors);
            let actions = MainViewModelActions {
                open_audio_requested: Some(Rc::new(move || {
                    if let Some(picker) = actions_for_audio.pick_audio_path.as_ref()
                        && let Some(path) = picker()
                    {
                        behaviors_for_audio.open_audio.open_audio(path);
                        return true;
                    }
                    false
                })),
                open_image_requested: Some(Rc::new(move || {
                    if let Some(picker) = actions_for_image.pick_image_path.as_ref()
                        && let Some(path) = picker()
                    {
                        behaviors_for_image.open_image.open_svg(path);
                    }
                })),
                interaction_mode_changed: Some(Rc::new(move |mode| {
                    behaviors_for_mode.apply_interaction_mode.apply_mode(mode);
                })),
            };

            view_model.borrow_mut().set_actions(actions);
        }

        apply_view_model(&view, &view_model.borrow());

        {
            let view_model = Rc::clone(&view_model);
            view.on_toggle_nav(move || {
                let mut view_model = view_model.borrow_mut();
                view_model.toggle_navigation();
            });
        }

        {
            let view_model = Rc::clone(&view_model);
            view.on_close_nav(move || {
                let mut view_model = view_model.borrow_mut();
                view_model.close_navigation();
            });
        }

        {
            let view_model = Rc::clone(&view_model);
            view.on_open_audio(move || {
                let mut view_model = view_model.borrow_mut();
                view_model.open_audio();
            });
        }

        {
            let view_model = Rc::clone(&view_model);
            view.on_open_image(move || {
                let view_model = view_model.borrow_mut();
                view_model.open_image();
            });
        }

        {
            let view_model = Rc::clone(&view_model);
            view.on_open_settings(move || {
                let mut view_model = view_model.borrow_mut();
                view_model.open_choreography_settings();
            });
        }

        {
            let view_model = Rc::clone(&view_model);
            view.on_select_mode(move |index| {
                let mut view_model = view_model.borrow_mut();
                let Some(option) = view_model.mode_options.get(index as usize).cloned() else {
                    return;
                };

                view_model.set_selected_mode(option.clone());
            });
        }

        {
            let view_model = Rc::clone(&view_model);
            view.on_close_dialog(move || {
                let mut view_model = view_model.borrow_mut();
                view_model.hide_dialog();
            });
        }

        Self {
            view,
            view_model,
            behaviors,
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

