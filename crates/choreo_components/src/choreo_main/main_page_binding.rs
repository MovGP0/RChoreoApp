use std::cell::RefCell;
use std::rc::Rc;

use crossbeam_channel::{Receiver, Sender};
use slint::{ComponentHandle, ModelRc, VecModel};

use choreo_state_machine::ApplicationStateMachine;

use crate::audio_player::{
    AudioPlayerPositionChangedEvent,
    AudioPlayerViewModel,
    OpenAudioFileCommand,
};
use crate::haptics::HapticFeedback;
use crate::floor::{
    CanvasViewHandle,
    FloorCanvasViewModel,
    FloorProvider,
    FloorProviderDependencies,
    Point,
    PointerButton,
    PointerEventArgs,
    PointerMovedCommand,
    PointerPressedCommand,
    PointerReleasedCommand,
    PointerWheelChangedCommand,
};
use crate::global::GlobalStateModel;
use crate::scenes::{
    OpenChoreoActions,
    ScenesDependencies,
    ScenesPaneViewModel,
    ScenesProvider,
};
use crate::settings::{
    build_settings_view_model,
    MaterialSchemeHelper,
    SettingsDependencies,
    SettingsViewModel
};
use crate::shell::ShellMaterialSchemeApplier;
use crate::time::format_seconds;
use crate::{SceneListItem, ShellHost};
use crate::preferences::SharedPreferences;

use super::{
    MainViewModel,
    MainViewModelActions,
    MainViewModelProvider,
    MainViewModelProviderDependencies,
    OpenAudioRequested,
    OpenImageRequested,
    mode_index,
    mode_option_from_index,
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
    pub audio_player: Rc<RefCell<AudioPlayerViewModel>>,
    pub haptic_feedback: Option<Box<dyn HapticFeedback>>,
    pub open_audio_sender: Sender<OpenAudioFileCommand>,
    pub close_audio_sender: Sender<crate::audio_player::CloseAudioFileCommand>,
    pub audio_position_receiver: Receiver<AudioPlayerPositionChangedEvent>,
    pub scenes_show_dialog_sender: Sender<crate::scenes::ShowDialogCommand>,
    pub scenes_close_dialog_sender: Sender<crate::scenes::CloseDialogCommand>,
    pub redraw_floor_receiver: Receiver<crate::choreography_settings::RedrawFloorCommand>,
    pub preferences: Rc<dyn crate::preferences::Preferences>,
    pub actions: MainPageActionHandlers,
}

pub struct MainPageBinding {
    view: ShellHost,
    view_model: Rc<RefCell<MainViewModel>>,
    scenes_view_model: Rc<RefCell<ScenesPaneViewModel>>,
    #[allow(dead_code)]
    settings_view_model: Rc<RefCell<SettingsViewModel>>,
    #[allow(dead_code)]
    floor_view_model: Rc<RefCell<FloorCanvasViewModel>>,
    #[allow(dead_code)]
    floor_provider: Rc<RefCell<FloorProvider>>,
}

impl MainPageBinding {
    pub fn new(view: ShellHost, deps: MainPageDependencies) -> Self {
        let global_state = deps.global_state.clone();
        let state_machine = deps.state_machine.clone();
        let preferences = Rc::clone(&deps.preferences);
        let open_audio_sender = deps.open_audio_sender.clone();
        let close_audio_sender = deps.close_audio_sender.clone();
        let scenes_show_dialog_sender = deps.scenes_show_dialog_sender.clone();
        let scenes_close_dialog_sender = deps.scenes_close_dialog_sender.clone();

        let actions = deps.actions;
        let view_weak = view.as_weak();
        let settings_view_model = Rc::new(RefCell::new(build_settings_view_model(
            SettingsDependencies {
                preferences: SharedPreferences::new(Rc::clone(&deps.preferences)),
                scheme_updater: MaterialSchemeHelper::new(ShellMaterialSchemeApplier::new(&view)),
            },
        )));
        let (show_timestamps_sender, show_timestamps_receiver) = crossbeam_channel::unbounded();
        let audio_position_receiver_for_scenes = deps.audio_position_receiver.clone();
        let scenes_provider = ScenesProvider::new(ScenesDependencies {
            global_state: deps.global_state.clone(),
            state_machine: Some(deps.state_machine.clone()),
            preferences: Rc::clone(&deps.preferences),
            show_dialog_sender: scenes_show_dialog_sender,
            close_dialog_sender: scenes_close_dialog_sender,
            haptic_feedback: None,
            open_audio_sender,
            close_audio_sender,
            audio_position_receiver: audio_position_receiver_for_scenes,
            show_timestamps_sender,
            show_timestamps_receiver,
            actions: OpenChoreoActions {
                pick_choreo_path: actions.pick_choreo_path.clone(),
            },
        });
        let scenes_view_model = scenes_provider.scenes_view_model();

        let audio_player = Rc::clone(&deps.audio_player);
        let audio_player_handle = Rc::downgrade(&audio_player);
        audio_player
            .borrow_mut()
            .set_self_handle(audio_player_handle);
        AudioPlayerViewModel::activate(&audio_player);

        let floor_provider = Rc::new(RefCell::new(FloorProvider::new(FloorProviderDependencies {
            global_state: Rc::clone(&deps.global_state),
            state_machine: Rc::clone(&deps.state_machine),
            preferences: Rc::clone(&deps.preferences),
            audio_position_receiver: deps.audio_position_receiver,
            redraw_floor_receiver: deps.redraw_floor_receiver,
        })));
        floor_provider
            .borrow_mut()
            .initialize_view(view_weak.clone());
        let draw_floor_sender = floor_provider.borrow().draw_floor_sender();
        let floor_view_model = floor_provider.borrow().floor_view_model();

        let mut main_provider = MainViewModelProvider::new(MainViewModelProviderDependencies {
            global_state: global_state.clone(),
            state_machine: state_machine.clone(),
            audio_player,
            open_audio_sender: deps.open_audio_sender,
            preferences: Rc::clone(&preferences),
            draw_floor_sender: draw_floor_sender.clone(),
            haptic_feedback: deps.haptic_feedback,
        });
        let interaction_mode_sender = main_provider.interaction_mode_sender();
        let open_audio_request_sender = main_provider.open_audio_request_sender();
        let open_image_request_sender = main_provider.open_image_request_sender();
        let view_model = Rc::new(RefCell::new(main_provider.create_main_view_model()));
        {
            let view_model_for_change = Rc::clone(&view_model);
            let floor_provider_for_change = Rc::clone(&floor_provider);
            let view_weak = view_weak.clone();
            let on_change = Rc::new(move || {
                let view_model = Rc::clone(&view_model_for_change);
                let floor_provider = Rc::clone(&floor_provider_for_change);
                let view_weak = view_weak.clone();
                slint::Timer::single_shot(std::time::Duration::from_millis(0), move || {
                    if let Some(view) = view_weak.upgrade()
                    {
                        let view_model_snapshot = view_model.borrow();
                        apply_view_model(&view, &view_model_snapshot);
                        drop(view_model_snapshot);
                        floor_provider.borrow().apply_to_view(&view);
                    }
                });
            });
            view_model.borrow_mut().set_on_change(Some(on_change));
        }

        {
            let scenes_view_model_for_change = Rc::clone(&scenes_view_model);
            let floor_provider_for_change = Rc::clone(&floor_provider);
            let view_weak = view_weak.clone();
            let on_change = Rc::new(move || {
                let scenes_view_model = Rc::clone(&scenes_view_model_for_change);
                let floor_provider = Rc::clone(&floor_provider_for_change);
                let view_weak = view_weak.clone();
                slint::Timer::single_shot(std::time::Duration::from_millis(0), move || {
                    if let Some(view) = view_weak.upgrade()
                    {
                        let scenes_view_model_snapshot = scenes_view_model.borrow();
                        apply_scenes_view_model(&view, &scenes_view_model_snapshot);
                        drop(scenes_view_model_snapshot);
                        floor_provider.borrow().apply_to_view(&view);
                    }
                });
            });
            scenes_view_model.borrow_mut().set_on_change(Some(on_change));
        }

        {
            let view_model = Rc::clone(&view_model);
            let actions_for_audio = actions.clone();
            let actions_for_image = actions.clone();
            let floor_view_model_for_image = Rc::clone(&floor_view_model);
            let actions = MainViewModelActions {
                open_audio_requested: Some(Rc::new(move || {
                    if let Some(picker) = actions_for_audio.pick_audio_path.as_ref()
                        && let Some(path) = picker()
                    {
                        let _ = open_audio_request_sender.try_send(OpenAudioRequested {
                            file_path: path,
                        });
                        return true;
                    }
                    false
                })),
                open_image_requested: Some(Rc::new(move || {
                    if let Some(picker) = actions_for_image.pick_image_path.as_ref()
                        && let Some(path) = picker()
                    {
                        let _ = open_image_request_sender.try_send(OpenImageRequested {
                            file_path: path,
                        });
                        floor_view_model_for_image.borrow_mut().draw_floor();
                    }
                })),
                interaction_mode_changed: Some(Rc::new(move |mode| {
                    let _ = interaction_mode_sender.try_send(mode);
                })),
            };

            view_model.borrow_mut().set_actions(actions);
        }

        view_model
            .borrow_mut()
            .set_self_handle(Rc::downgrade(&view_model));
        MainViewModel::activate(&view_model);

        apply_view_model(&view, &view_model.borrow());
        apply_scenes_view_model(&view, &scenes_view_model.borrow());
        floor_provider.borrow().apply_to_view(&view);

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
                let Some(mode) = mode_option_from_index(index) else {
                    return;
                };

                view_model.set_selected_mode(mode);
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

        {
            let floor_view_model = Rc::clone(&floor_view_model);
            view.on_floor_pointer_pressed(move |x, y, is_primary, is_in_contact| {
                let view_model = floor_view_model.borrow();
                let button = if is_primary {
                    PointerButton::Primary
                } else {
                    PointerButton::Secondary
                };
                let command = PointerPressedCommand {
                    canvas_view: CanvasViewHandle,
                    event_args: PointerEventArgs {
                        position: Point::new(x as f64, y as f64),
                        button,
                        is_in_contact,
                    },
                };
                view_model.pointer_pressed(command);
            });
        }

        {
            let floor_view_model = Rc::clone(&floor_view_model);
            view.on_floor_pointer_moved(move |x, y, is_primary, is_in_contact| {
                let view_model = floor_view_model.borrow();
                let button = if is_primary {
                    PointerButton::Primary
                } else {
                    PointerButton::Secondary
                };
                let command = PointerMovedCommand {
                    canvas_view: CanvasViewHandle,
                    event_args: PointerEventArgs {
                        position: Point::new(x as f64, y as f64),
                        button,
                        is_in_contact,
                    },
                };
                view_model.pointer_moved(command);
            });
        }

        {
            let floor_view_model = Rc::clone(&floor_view_model);
            view.on_floor_pointer_released(move |x, y, is_primary, is_in_contact| {
                let view_model = floor_view_model.borrow();
                let button = if is_primary {
                    PointerButton::Primary
                } else {
                    PointerButton::Secondary
                };
                let command = PointerReleasedCommand {
                    canvas_view: CanvasViewHandle,
                    event_args: PointerEventArgs {
                        position: Point::new(x as f64, y as f64),
                        button,
                        is_in_contact,
                    },
                };
                view_model.pointer_released(command);
            });
        }

        {
            let floor_view_model = Rc::clone(&floor_view_model);
            view.on_floor_pointer_wheel_changed(move |delta, x, y| {
                let view_model = floor_view_model.borrow();
                let command = PointerWheelChangedCommand {
                    canvas_view: CanvasViewHandle,
                    delta: delta as f64,
                    position: Some(Point::new(x as f64, y as f64)),
                };
                view_model.pointer_wheel_changed(command);
            });
        }

        Self {
            view,
            view_model,
            scenes_view_model,
            settings_view_model,
            floor_view_model,
            floor_provider,
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

}

fn apply_view_model(view: &ShellHost, view_model: &MainViewModel) {
    view.set_selected_mode_index(selected_mode_index(view_model));
    view.set_is_mode_selection_enabled(view_model.is_mode_selection_enabled);
    view.set_nav_width(view_model.nav_width);
    view.set_is_nav_open(view_model.is_nav_open);
    view.set_is_choreography_settings_open(view_model.is_choreography_settings_open);
    view.set_is_audio_player_open(view_model.is_audio_player_open);
    view.set_is_dialog_open(view_model.is_dialog_open);

    view.set_dialog_content(
        view_model
            .dialog_content
            .as_deref()
            .unwrap_or_default()
            .into(),
    );
}

fn apply_scenes_view_model(view: &ShellHost, view_model: &ScenesPaneViewModel) {
    view.set_scenes(build_scene_items(&view_model.scenes));
    view.set_scenes_search_text(view_model.search_text.as_str().into());
    view.set_scenes_show_timestamps(view_model.show_timestamps);
    view.set_scenes_can_save_choreo(view_model.can_save_choreo);
    view.set_scenes_can_delete_scene(view_model.can_delete_scene);
    view.set_scenes_can_navigate_to_settings(view_model.can_navigate_to_settings);
    view.set_scenes_can_navigate_to_dancer_settings(view_model.can_navigate_to_dancer_settings);
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

fn selected_mode_index(view_model: &MainViewModel) -> i32 {
    mode_index(view_model.selected_mode)
}

