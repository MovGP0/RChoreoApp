use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use crossbeam_channel::{unbounded, Receiver, Sender};
use slint::{ComponentHandle, ModelRc, Timer, TimerMode, VecModel};

use choreo_state_machine::ApplicationStateMachine;

use crate::audio_player::{
    AudioPlayerPositionChangedEvent,
    AudioPlayerViewModel,
    HapticFeedback,
    OpenAudioFileCommand,
};
use crate::floor::{
    build_floor_canvas_view_model,
    CanvasViewHandle,
    FloorAdapter,
    FloorCanvasViewModel,
    FloorDependencies,
    Point,
    PointerButton,
    PointerEventArgs,
    PointerMovedCommand,
    PointerPressedCommand,
    PointerReleasedCommand,
    PointerWheelChangedCommand,
};
use crate::global::GlobalStateModel;
use crate::scenes::{build_scenes_view_model, OpenChoreoActions, ScenesDependencies, ScenesPaneViewModel};
use crate::settings::{build_settings_view_model, MaterialSchemeHelper, SettingsDependencies, SettingsViewModel};
use crate::shell::ShellMaterialSchemeApplier;
use crate::time::format_seconds;
use crate::{SceneListItem, ShellHost};
use crate::preferences::SharedPreferences;

use super::{
    build_main_behaviors,
    build_main_view_model,
    CloseDialogCommand,
    MainBehaviorDependencies,
    MainBehaviors,
    MainDependencies,
    MainViewModelActions,
    MainViewModel,
    mode_index,
    mode_option_from_index,
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
    pub haptic_feedback: Option<Box<dyn HapticFeedback>>,
    pub open_audio_sender: Sender<OpenAudioFileCommand>,
    pub close_audio_sender: Sender<crate::audio_player::CloseAudioFileCommand>,
    pub audio_position_receiver: Receiver<AudioPlayerPositionChangedEvent>,
    pub open_svg_sender: Sender<OpenSvgFileCommand>,
    pub open_svg_receiver: Receiver<OpenSvgFileCommand>,
    pub show_dialog_sender: Sender<ShowDialogCommand>,
    pub show_dialog_receiver: Receiver<ShowDialogCommand>,
    pub close_dialog_sender: Sender<CloseDialogCommand>,
    pub close_dialog_receiver: Receiver<CloseDialogCommand>,
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
    behaviors: Rc<MainBehaviors>,
    #[allow(dead_code)]
    settings_view_model: Rc<RefCell<SettingsViewModel>>,
    #[allow(dead_code)]
    floor_view_model: Rc<RefCell<FloorCanvasViewModel>>,
    #[allow(dead_code)]
    floor_adapter: Rc<RefCell<FloorAdapter>>,
    #[allow(dead_code)]
    floor_audio_timer: Timer,
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
        let view_model = Rc::new(RefCell::new(build_main_view_model(MainDependencies {
            global_state: global_state.clone(),
            state_machine: state_machine.clone(),
            audio_player: deps.audio_player,
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
        let settings_view_model = Rc::new(RefCell::new(build_settings_view_model(
            SettingsDependencies {
                preferences: SharedPreferences::new(Rc::clone(&deps.preferences)),
                scheme_updater: MaterialSchemeHelper::new(ShellMaterialSchemeApplier::new(&view)),
            },
        )));
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

        let (draw_floor_sender, draw_floor_receiver) = unbounded();
        let floor_view_model = build_floor_canvas_view_model(FloorDependencies {
            global_state: deps.global_state.clone(),
            state_machine: deps.state_machine.clone(),
            preferences: Rc::clone(&deps.preferences),
            draw_floor_sender,
            draw_floor_receiver,
            redraw_receiver: deps.redraw_floor_receiver,
            render_gate: None,
        });

        let floor_adapter = Rc::new(RefCell::new(FloorAdapter::new(
            deps.global_state.clone(),
            deps.state_machine.clone(),
            Rc::clone(&deps.preferences),
            deps.audio_position_receiver,
        )));

        {
            let floor_view_model_for_redraw = Rc::clone(&floor_view_model);
            let floor_adapter_for_redraw = Rc::clone(&floor_adapter);
            let view_weak = view_weak.clone();
            floor_view_model.borrow_mut().set_on_redraw(Some(Rc::new(move || {
                let floor_view_model = Rc::clone(&floor_view_model_for_redraw);
                let floor_adapter = Rc::clone(&floor_adapter_for_redraw);
                let view_weak = view_weak.clone();
                slint::Timer::single_shot(Duration::from_millis(0), move || {
                    if let Some(view) = view_weak.upgrade() {
                        apply_floor_view(&view, &floor_adapter, &floor_view_model);
                    }
                });
            })));
        }

        let floor_audio_timer = Timer::default();
        {
            let floor_adapter = Rc::clone(&floor_adapter);
            let floor_view_model = Rc::clone(&floor_view_model);
            let view_weak = view_weak.clone();
            floor_audio_timer.start(
                TimerMode::Repeated,
                Duration::from_millis(100),
                move || {
                    let mut adapter = floor_adapter.borrow_mut();
                    if !adapter.poll_audio_position() {
                        return;
                    }

                    if let Some(view) = view_weak.upgrade() {
                        let mut floor_view_model = floor_view_model.borrow_mut();
                        adapter.apply(&view, &mut floor_view_model);
                    }
                },
            );
        }

        {
            let view_model_for_change = Rc::clone(&view_model);
            let floor_view_model_for_change = Rc::clone(&floor_view_model);
            let floor_adapter_for_change = Rc::clone(&floor_adapter);
            let behaviors_for_dialog = Rc::clone(&behaviors);
            let view_weak = view_weak.clone();
            let on_change = Rc::new(move || {
                let view_model = Rc::clone(&view_model_for_change);
                let floor_view_model = Rc::clone(&floor_view_model_for_change);
                let floor_adapter = Rc::clone(&floor_adapter_for_change);
                let behaviors = Rc::clone(&behaviors_for_dialog);
                let view_weak = view_weak.clone();
                slint::Timer::single_shot(std::time::Duration::from_millis(0), move || {
                    if let Some(view) = view_weak.upgrade()
                    {
                        let view_model_snapshot = view_model.borrow();
                        apply_view_model(&view, &view_model_snapshot);
                        drop(view_model_snapshot);
                        apply_floor_view(&view, &floor_adapter, &floor_view_model);
                        drain_dialog_commands(&view, &view_model, &behaviors);
                    }
                });
            });
            view_model.borrow_mut().set_on_change(Some(on_change));
        }

        {
            let scenes_view_model_for_change = Rc::clone(&scenes_view_model);
            let floor_view_model_for_change = Rc::clone(&floor_view_model);
            let floor_adapter_for_change = Rc::clone(&floor_adapter);
            let view_model_for_dialog = Rc::clone(&view_model);
            let behaviors_for_dialog = Rc::clone(&behaviors);
            let view_weak = view_weak.clone();
            let on_change = Rc::new(move || {
                let scenes_view_model = Rc::clone(&scenes_view_model_for_change);
                let floor_view_model = Rc::clone(&floor_view_model_for_change);
                let floor_adapter = Rc::clone(&floor_adapter_for_change);
                let view_model = Rc::clone(&view_model_for_dialog);
                let behaviors = Rc::clone(&behaviors_for_dialog);
                let view_weak = view_weak.clone();
                slint::Timer::single_shot(std::time::Duration::from_millis(0), move || {
                    if let Some(view) = view_weak.upgrade()
                    {
                        let scenes_view_model_snapshot = scenes_view_model.borrow();
                        apply_scenes_view_model(&view, &scenes_view_model_snapshot);
                        drop(scenes_view_model_snapshot);
                        apply_floor_view(&view, &floor_adapter, &floor_view_model);
                        drain_dialog_commands(&view, &view_model, &behaviors);
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
            let floor_view_model_for_image = Rc::clone(&floor_view_model);
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
                        let _ = behaviors_for_image.open_svg_file.try_handle();
                        floor_view_model_for_image.borrow_mut().draw_floor();
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
        apply_floor_view(&view, &floor_adapter, &floor_view_model);

        {
            let floor_view_model = Rc::clone(&floor_view_model);
            let view_weak = view_weak.clone();
            let floor_adapter = Rc::clone(&floor_adapter);
            slint::Timer::single_shot(Duration::from_millis(0), move || {
                if let Some(view) = view_weak.upgrade() {
                    apply_floor_view(&view, &floor_adapter, &floor_view_model);
                    floor_view_model.borrow_mut().draw_floor();
                }
            });
        }

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
                let mut view_model = floor_view_model.borrow_mut();
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
                let mut view_model = floor_view_model.borrow_mut();
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
                let mut view_model = floor_view_model.borrow_mut();
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
                let mut view_model = floor_view_model.borrow_mut();
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
            behaviors,
            settings_view_model,
            floor_view_model,
            floor_adapter,
            floor_audio_timer,
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

fn apply_floor_view(
    view: &ShellHost,
    floor_adapter: &Rc<RefCell<FloorAdapter>>,
    floor_view_model: &Rc<RefCell<FloorCanvasViewModel>>,
) {
    let mut floor_view_model = floor_view_model.borrow_mut();
    floor_adapter.borrow_mut().apply(view, &mut floor_view_model);
}

fn drain_dialog_commands(
    view: &ShellHost,
    view_model: &Rc<RefCell<MainViewModel>>,
    behaviors: &Rc<MainBehaviors>,
)
{
    let mut view_model_ref = view_model.borrow_mut();
    let mut updated = false;

    while behaviors.show_dialog.try_handle(&mut view_model_ref)
    {
        updated = true;
    }

    while behaviors.hide_dialog.try_handle(&mut view_model_ref)
    {
        updated = true;
    }

    drop(view_model_ref);

    if updated
    {
        let view_model_snapshot = view_model.borrow();
        apply_view_model(view, &view_model_snapshot);
    }
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

