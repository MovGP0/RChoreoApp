use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

use crossbeam_channel::{bounded, Receiver, Sender};
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
use crate::global::{GlobalStateModel, GlobalStateActor};
use crate::scenes::{
    OpenChoreoActions,
    OpenChoreoRequested,
    ScenesDependencies,
    ScenesPaneViewModel,
    ScenesProvider,
};
use crate::settings::{
    MaterialSchemeHelper,
    SettingsDependencies,
    SettingsProvider,
    SettingsViewModel
};
use crate::shell::ShellMaterialSchemeApplier;
use crate::time::format_seconds;
use crate::{SceneListItem, ShellHost};
use crate::preferences::SharedPreferences;
use crate::nav_bar::{
    apply_nav_bar_view_model,
    bind_nav_bar,
    InteractionModeChangedCommand,
    NavBarSenders,
    OpenAudioRequestedCommand as NavBarOpenAudioRequestedCommand,
    OpenImageRequestedCommand as NavBarOpenImageRequestedCommand,
    NavBarViewModel,
};

use super::{
    MainViewModel,
    MainViewModelProvider,
    MainViewModelProviderDependencies,
    OpenAudioRequested,
    OpenImageRequested,
};

#[derive(Clone, Default)]
pub struct MainPageActionHandlers {
    pub pick_audio_path: Option<Rc<dyn Fn() -> Option<String>>>,
    pub pick_image_path: Option<Rc<dyn Fn() -> Option<String>>>,
    pub request_open_choreo: Option<Rc<dyn Fn(Sender<crate::scenes::OpenChoreoRequested>)>>,
    pub request_open_audio: Option<Rc<dyn Fn(Sender<OpenAudioRequested>)>>,
    pub request_open_image: Option<Rc<dyn Fn(Sender<OpenImageRequested>)>>,
}

pub struct MainPageDependencies {
    pub global_state: Rc<RefCell<GlobalStateModel>>,
    pub global_state_store: Rc<GlobalStateActor>,
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
    #[allow(dead_code)]
    nav_bar_timer: slint::Timer,
    open_choreo_sender: Sender<OpenChoreoRequested>,
    open_audio_request_sender: Sender<OpenAudioRequested>,
    open_image_request_sender: Sender<OpenImageRequested>,
}

impl MainPageBinding {
    pub fn new(view: ShellHost, deps: MainPageDependencies) -> Self {
        let global_state = deps.global_state.clone();
        let global_state_store = deps.global_state_store.clone();
        let state_machine = deps.state_machine.clone();
        let preferences = Rc::clone(&deps.preferences);
        let open_audio_sender = deps.open_audio_sender.clone();
        let close_audio_sender = deps.close_audio_sender.clone();
        let scenes_show_dialog_sender = deps.scenes_show_dialog_sender.clone();
        let scenes_close_dialog_sender = deps.scenes_close_dialog_sender.clone();

        let actions = deps.actions;
        let view_weak = view.as_weak();
        let settings_view_model = SettingsProvider::new(SettingsDependencies {
            preferences: SharedPreferences::new(Rc::clone(&deps.preferences)),
            scheme_updater: MaterialSchemeHelper::new(ShellMaterialSchemeApplier::new(&view)),
        })
        .settings_view_model();
        let (show_timestamps_sender, show_timestamps_receiver) = crossbeam_channel::unbounded();
        let audio_position_receiver_for_scenes = deps.audio_position_receiver.clone();
        let scenes_provider = ScenesProvider::new(ScenesDependencies {
            global_state: deps.global_state.clone(),
            global_state_store,
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
                request_open_choreo: actions.request_open_choreo.clone(),
            },
        });
        let open_choreo_sender = scenes_provider.open_choreo_sender();
        let scenes_view_model = scenes_provider.scenes_view_model();

        let audio_player = Rc::clone(&deps.audio_player);
        let audio_player_handle = Rc::downgrade(&audio_player);
        audio_player
            .borrow_mut()
            .set_self_handle(audio_player_handle);
        AudioPlayerViewModel::activate(&audio_player);

        let floor_provider = Rc::new(RefCell::new(FloorProvider::new(FloorProviderDependencies {
            global_state: Rc::clone(&deps.global_state),
            global_state_store: Rc::clone(&deps.global_state_store),
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
            global_state_store: deps.global_state_store.clone(),
            state_machine: state_machine.clone(),
            open_audio_sender: deps.open_audio_sender,
            preferences: Rc::clone(&preferences),
            draw_floor_sender: draw_floor_sender.clone(),
        });
        let interaction_mode_sender = main_provider.interaction_mode_sender();
        let open_audio_request_sender = main_provider.open_audio_request_sender();
        let open_image_request_sender = main_provider.open_image_request_sender();

        const NAV_BAR_EVENT_BUFFER: usize = 64;
        let (nav_bar_open_audio_sender, nav_bar_open_audio_receiver) =
            bounded::<NavBarOpenAudioRequestedCommand>(NAV_BAR_EVENT_BUFFER);
        let (nav_bar_open_image_sender, nav_bar_open_image_receiver) =
            bounded::<NavBarOpenImageRequestedCommand>(NAV_BAR_EVENT_BUFFER);
        let (nav_bar_mode_sender, nav_bar_mode_receiver) =
            bounded::<InteractionModeChangedCommand>(NAV_BAR_EVENT_BUFFER);

        let nav_bar = Rc::new(RefCell::new(NavBarViewModel::new(
            global_state.clone(),
            deps.haptic_feedback,
            Vec::new(),
            NavBarSenders {
                open_audio_requested: nav_bar_open_audio_sender,
                open_image_requested: nav_bar_open_image_sender,
                interaction_mode_changed: nav_bar_mode_sender,
            },
        )));

        let view_model = Rc::new(RefCell::new(main_provider.create_main_view_model(
            Rc::clone(&nav_bar),
        )));
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
            let view_model_for_change = Rc::clone(&view_model);
            let view_weak = view_weak.clone();
            let on_change = Rc::new(move || {
                let view_model = Rc::clone(&view_model_for_change);
                let view_weak = view_weak.clone();
                slint::Timer::single_shot(std::time::Duration::from_millis(0), move || {
                    if let Some(view) = view_weak.upgrade() {
                        let view_model_snapshot = view_model.borrow();
                        apply_view_model(&view, &view_model_snapshot);
                    }
                });
            });
            nav_bar.borrow_mut().set_on_change(Some(on_change));
        }

        let nav_bar_timer = {
            let nav_bar = Rc::clone(&nav_bar);
            let actions_for_audio = actions.clone();
            let actions_for_image = actions.clone();
            let floor_view_model_for_image = Rc::clone(&floor_view_model);
            let open_audio_request_sender = open_audio_request_sender.clone();
            let open_image_request_sender = open_image_request_sender.clone();
            let interaction_mode_sender = interaction_mode_sender.clone();
            let nav_bar_open_audio_receiver = nav_bar_open_audio_receiver.clone();
            let nav_bar_open_image_receiver = nav_bar_open_image_receiver.clone();
            let nav_bar_mode_receiver = nav_bar_mode_receiver.clone();
            let timer = slint::Timer::default();
            timer.start(
                slint::TimerMode::Repeated,
                std::time::Duration::from_millis(16),
                move || {
                    while nav_bar_open_audio_receiver.try_recv().is_ok() {
                        if let Some(requester) = actions_for_audio.request_open_audio.as_ref() {
                            requester(open_audio_request_sender.clone());
                            nav_bar.borrow_mut().set_audio_player_opened(true);
                            continue;
                        }

                        if let Some(picker) = actions_for_audio.pick_audio_path.as_ref()
                            && let Some(path) = picker()
                        {
                            let _ = open_audio_request_sender.try_send(OpenAudioRequested {
                                file_path: path,
                            });
                            nav_bar.borrow_mut().set_audio_player_opened(true);
                        }
                    }

                    while nav_bar_open_image_receiver.try_recv().is_ok() {
                        if let Some(requester) = actions_for_image.request_open_image.as_ref() {
                            requester(open_image_request_sender.clone());
                            floor_view_model_for_image.borrow_mut().draw_floor();
                            continue;
                        }

                        if let Some(picker) = actions_for_image.pick_image_path.as_ref()
                            && let Some(path) = picker()
                        {
                            let _ = open_image_request_sender.try_send(OpenImageRequested {
                                file_path: path,
                            });
                            floor_view_model_for_image.borrow_mut().draw_floor();
                        }
                    }

                    while let Ok(mode_command) = nav_bar_mode_receiver.try_recv() {
                        let _ = interaction_mode_sender.try_send(mode_command.mode);
                    }
                },
            );
            timer
        };

        view_model
            .borrow_mut()
            .set_self_handle(Rc::downgrade(&view_model));
        MainViewModel::activate(&view_model);

        apply_view_model(&view, &view_model.borrow());
        apply_scenes_view_model(&view, &scenes_view_model.borrow());
        floor_provider.borrow().apply_to_view(&view);

        bind_nav_bar(&view, Rc::clone(&nav_bar));

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
            let view_weak = view_weak.clone();
            view.on_scenes_navigate_to_settings(move || {
                if let Some(view) = view_weak.upgrade() {
                    let next_index = if view.get_content_index() == 1 { 0 } else { 1 };
                    view.set_content_index(next_index);
                }
                let mut view_model = scenes_view_model.borrow_mut();
                view_model.navigate_to_settings();
            });
        }

        {
            let scenes_view_model = Rc::clone(&scenes_view_model);
            let view_weak = view_weak.clone();
            view.on_scenes_navigate_to_dancer_settings(move || {
                if let Some(view) = view_weak.upgrade() {
                    let next_index = if view.get_content_index() == 2 { 0 } else { 2 };
                    view.set_content_index(next_index);
                }
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
            nav_bar_timer,
            open_choreo_sender,
            open_audio_request_sender,
            open_image_request_sender,
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

    pub fn open_choreo_sender(&self) -> Sender<OpenChoreoRequested> {
        self.open_choreo_sender.clone()
    }

    pub fn open_audio_request_sender(&self) -> Sender<OpenAudioRequested> {
        self.open_audio_request_sender.clone()
    }

    pub fn open_image_request_sender(&self) -> Sender<OpenImageRequested> {
        self.open_image_request_sender.clone()
    }

    pub fn route_external_file_path(&self, file_path: &str) {
        if file_path.trim().is_empty() {
            return;
        }

        let extension = Path::new(file_path)
            .extension()
            .and_then(|value| value.to_str())
            .unwrap_or_default()
            .to_ascii_lowercase();

        if extension == "choreo" {
            let Ok(contents) = std::fs::read_to_string(file_path) else {
                return;
            };
            let file_name = Path::new(file_path)
                .file_name()
                .map(|value| value.to_string_lossy().into_owned());
            let _ = self.open_choreo_sender.try_send(OpenChoreoRequested {
                file_path: Some(file_path.to_string()),
                file_name,
                contents,
            });
            return;
        }

        if extension == "svg" {
            let _ = self
                .open_image_request_sender
                .try_send(OpenImageRequested {
                    file_path: file_path.to_string(),
                });
            return;
        }

        if extension == "mp3" {
            let _ = self
                .open_audio_request_sender
                .try_send(OpenAudioRequested {
                    file_path: file_path.to_string(),
                });
        }
    }

}

fn apply_view_model(view: &ShellHost, view_model: &MainViewModel) {
    let nav_bar = view_model.nav_bar.borrow();
    apply_nav_bar_view_model(view, &nav_bar);
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

