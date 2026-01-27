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
use crate::scenes::{build_scenes_view_model, OpenChoreoActions, ScenesDependencies, ScenesPaneViewModel};
use crate::time::format_seconds;
use crate::{MenuItem, SceneListItem, ShellHost};

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
    pub pick_choreo_path: Option<Rc<dyn Fn() -> Option<String>>>,
}

pub struct MainPageDependencies {
    pub global_state: Rc<RefCell<GlobalStateModel>>,
    pub state_machine: Rc<RefCell<ApplicationStateMachine>>,
    pub audio_player: AudioPlayerViewModel,
    pub locale: String,
    pub haptic_feedback: Option<Box<dyn HapticFeedback>>,
    pub open_audio_sender: Sender<OpenAudioFileCommand>,
    pub close_audio_sender: Sender<crate::audio_player::CloseAudioFileCommand>,
    pub open_svg_sender: Sender<OpenSvgFileCommand>,
    pub open_svg_receiver: Receiver<OpenSvgFileCommand>,
    pub show_dialog_sender: Sender<ShowDialogCommand>,
    pub show_dialog_receiver: Receiver<ShowDialogCommand>,
    pub close_dialog_sender: Sender<CloseDialogCommand>,
    pub close_dialog_receiver: Receiver<CloseDialogCommand>,
    pub scenes_show_dialog_sender: Sender<crate::scenes::ShowDialogCommand>,
    pub scenes_close_dialog_sender: Sender<crate::scenes::CloseDialogCommand>,
    pub preferences: Rc<dyn crate::preferences::Preferences>,
    pub actions: MainPageActionHandlers,
}

pub struct MainPageBinding {
    view: ShellHost,
    view_model: Rc<RefCell<MainViewModel>>,
    scenes_view_model: Rc<RefCell<ScenesPaneViewModel>>,
    behaviors: Rc<MainBehaviors>,
}

impl MainPageBinding {
    pub fn new(view: ShellHost, deps: MainPageDependencies) -> Self {
        let locale = deps.locale.clone();
        let global_state = deps.global_state.clone();
        let state_machine = deps.state_machine.clone();
        let preferences = Rc::clone(&deps.preferences);
        let open_audio_sender = deps.open_audio_sender.clone();
        let close_audio_sender = deps.close_audio_sender.clone();
        let scenes_show_dialog_sender = deps.scenes_show_dialog_sender.clone();
        let scenes_close_dialog_sender = deps.scenes_close_dialog_sender.clone();
        let view_model = Rc::new(RefCell::new(build_main_view_model(MainDependencies {
            global_state: global_state.clone(),
            state_machine: state_machine.clone(),
            audio_player: deps.audio_player,
            locale,
            haptic_feedback: deps.haptic_feedback,
            actions: MainViewModelActions::default(),
        })));

        let behaviors = Rc::new(build_main_behaviors(MainBehaviorDependencies {
            global_state,
            state_machine,
            open_audio_sender: deps.open_audio_sender,
            open_svg_sender: deps.open_svg_sender,
            open_svg_receiver: deps.open_svg_receiver,
            show_dialog_receiver: deps.show_dialog_receiver,
            close_dialog_receiver: deps.close_dialog_receiver,
            preferences,
        }));

        let actions = deps.actions;
        let view_weak = view.as_weak();
        let scenes_view_model = Rc::new(RefCell::new(build_scenes_view_model(ScenesDependencies {
            global_state: deps.global_state.clone(),
            state_machine: Some(deps.state_machine.clone()),
            preferences: Rc::clone(&deps.preferences),
            show_dialog_sender: scenes_show_dialog_sender,
            close_dialog_sender: scenes_close_dialog_sender,
            haptic_feedback: None,
            open_audio_sender,
            close_audio_sender,
            actions: OpenChoreoActions {
                pick_choreo_path: actions.pick_choreo_path.clone(),
            },
        })));

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
            let scenes_view_model_for_change = Rc::clone(&scenes_view_model);
            let view_weak = view_weak.clone();
            let on_change = Rc::new(move || {
                let scenes_view_model = Rc::clone(&scenes_view_model_for_change);
                let view_weak = view_weak.clone();
                slint::Timer::single_shot(std::time::Duration::from_millis(0), move || {
                    if let Some(view) = view_weak.upgrade() {
                        let scenes_view_model = scenes_view_model.borrow();
                        apply_scenes_view_model(&view, &scenes_view_model);
                    }
                });
            });
            scenes_view_model.borrow_mut().set_on_change(Some(on_change));
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
        apply_scenes_view_model(&view, &scenes_view_model.borrow());

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
            view.on_close_settings(move || {
                let mut view_model = view_model.borrow_mut();
                view_model.close_choreography_settings();
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

        {
            let scenes_view_model = Rc::clone(&scenes_view_model);
            view.on_scenes_select_scene(move |index| {
                let mut view_model = scenes_view_model.borrow_mut();
                if index >= 0 {
                    view_model.select_scene(index as usize);
                }
            });
        }

        {
            let scenes_view_model = Rc::clone(&scenes_view_model);
            view.on_scenes_update_search_text(move |value| {
                let mut view_model = scenes_view_model.borrow_mut();
                view_model.update_search_text(value.to_string());
            });
        }

        {
            let scenes_view_model = Rc::clone(&scenes_view_model);
            view.on_scenes_add_scene_before(move || {
                let mut view_model = scenes_view_model.borrow_mut();
                view_model.add_scene_before();
            });
        }

        {
            let scenes_view_model = Rc::clone(&scenes_view_model);
            view.on_scenes_add_scene_after(move || {
                let mut view_model = scenes_view_model.borrow_mut();
                view_model.add_scene_after();
            });
        }

        {
            let scenes_view_model = Rc::clone(&scenes_view_model);
            view.on_scenes_delete_scene(move || {
                let mut view_model = scenes_view_model.borrow_mut();
                view_model.delete_scene();
            });
        }

        {
            let scenes_view_model = Rc::clone(&scenes_view_model);
            view.on_scenes_open_choreo(move || {
                let mut view_model = scenes_view_model.borrow_mut();
                view_model.open_choreo();
            });
        }

        {
            let scenes_view_model = Rc::clone(&scenes_view_model);
            view.on_scenes_save_choreo(move || {
                let mut view_model = scenes_view_model.borrow_mut();
                view_model.save_choreo();
            });
        }

        {
            let scenes_view_model = Rc::clone(&scenes_view_model);
            view.on_scenes_navigate_to_settings(move || {
                let mut view_model = scenes_view_model.borrow_mut();
                view_model.navigate_to_settings();
            });
        }

        {
            let scenes_view_model = Rc::clone(&scenes_view_model);
            view.on_scenes_navigate_to_dancer_settings(move || {
                let mut view_model = scenes_view_model.borrow_mut();
                view_model.navigate_to_dancer_settings();
            });
        }

        Self {
            view,
            view_model,
            scenes_view_model,
            behaviors,
        }
    }

    pub fn view(&self) -> &ShellHost {
        &self.view
    }

    pub fn view_model(&self) -> Rc<RefCell<MainViewModel>> {
        Rc::clone(&self.view_model)
    }

    pub fn scenes_view_model(&self) -> Rc<RefCell<ScenesPaneViewModel>> {
        Rc::clone(&self.scenes_view_model)
    }

    pub fn behaviors(&self) -> Rc<MainBehaviors> {
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

fn apply_scenes_view_model(view: &ShellHost, view_model: &ScenesPaneViewModel) {
    view.set_scenes(build_scene_items(&view_model.scenes));
    view.set_scenes_search_text(view_model.search_text.as_str().into());
    view.set_scenes_show_timestamps(view_model.show_timestamps);
    view.set_scenes_can_save_choreo(view_model.can_save_choreo);
    view.set_scenes_can_delete_scene(view_model.can_delete_scene);
    view.set_scenes_can_navigate_to_settings(view_model.can_navigate_to_settings);
    view.set_scenes_can_navigate_to_dancer_settings(view_model.can_navigate_to_dancer_settings);
    view.set_scenes_add_before_text(view_model.add_before_text.as_str().into());
    view.set_scenes_add_after_text(view_model.add_after_text.as_str().into());
    view.set_scenes_open_text(view_model.open_text.as_str().into());
    view.set_scenes_save_text(view_model.save_text.as_str().into());
}

fn build_scene_items(scenes: &[crate::scenes::SceneViewModel]) -> ModelRc<SceneListItem> {
    let items = scenes
        .iter()
        .map(|scene| SceneListItem {
            name: scene.name.as_str().into(),
            timestamp_text: scene
                .timestamp
                .map(format_seconds)
                .unwrap_or_default()
                .into(),
            color: slint::Color::from_argb_u8(scene.color.a, scene.color.r, scene.color.g, scene.color.b),
            is_selected: scene.is_selected,
        })
        .collect::<Vec<_>>();

    ModelRc::new(VecModel::from(items))
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

