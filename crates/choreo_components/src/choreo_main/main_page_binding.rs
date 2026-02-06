use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

use choreo_master_mobile_json::Color as ChoreoColor;
use crossbeam_channel::{bounded, Receiver, Sender};
use slint::{Color, ComponentHandle, Image, ModelRc, VecModel};

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
use crate::choreography_settings::{
    ChoreographySettingsViewModel,
    LoadChoreographySettingsBehavior,
    ReloadChoreographySettingsCommand,
    UpdateAuthorBehavior,
    UpdateAuthorCommand,
    UpdateCommentBehavior,
    UpdateCommentCommand,
    UpdateDateBehavior,
    UpdateDateCommand,
    UpdateDescriptionBehavior,
    UpdateDescriptionCommand,
    UpdateDrawPathFromBehavior,
    UpdateDrawPathFromCommand,
    UpdateDrawPathToBehavior,
    UpdateDrawPathToCommand,
    UpdateFloorBackBehavior,
    UpdateFloorBackCommand,
    UpdateFloorColorBehavior,
    UpdateFloorColorCommand,
    UpdateFloorFrontBehavior,
    UpdateFloorFrontCommand,
    UpdateFloorLeftBehavior,
    UpdateFloorLeftCommand,
    UpdateFloorRightBehavior,
    UpdateFloorRightCommand,
    UpdateGridLinesBehavior,
    UpdateGridLinesCommand,
    UpdateGridResolutionBehavior,
    UpdateGridResolutionCommand,
    UpdateNameBehavior,
    UpdateNameCommand,
    UpdatePositionsAtSideBehavior,
    UpdatePositionsAtSideCommand,
    UpdateSelectedSceneBehavior,
    UpdateSelectedSceneCommand,
    UpdateShowLegendBehavior,
    UpdateShowLegendCommand,
    UpdateShowTimestampsBehavior,
    UpdateShowTimestampsCommand,
    UpdateSnapToGridBehavior,
    UpdateSnapToGridCommand,
    UpdateSubtitleBehavior,
    UpdateSubtitleCommand,
    UpdateTransparencyBehavior,
    UpdateTransparencyCommand,
    UpdateVariationBehavior,
    UpdateVariationCommand,
};
use crate::settings::{
    MaterialSchemeHelper,
    SettingsDependencies,
    SettingsProvider,
    SettingsViewModel
};
use crate::shell::ShellMaterialSchemeApplier;
use crate::time::format_seconds;
use crate::{Date, MenuItem, SceneListItem, ShellHost};
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
use crate::behavior::{Behavior, CompositeDisposable};

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
    pub redraw_floor_sender: Sender<crate::choreography_settings::RedrawFloorCommand>,
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
    choreography_settings_view_model: Rc<RefCell<ChoreographySettingsViewModel>>,
    #[allow(dead_code)]
    floor_view_model: Rc<RefCell<FloorCanvasViewModel>>,
    #[allow(dead_code)]
    floor_provider: Rc<RefCell<FloorProvider>>,
    #[allow(dead_code)]
    nav_bar_timer: slint::Timer,
    #[allow(dead_code)]
    choreography_settings_disposables: CompositeDisposable,
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
        let redraw_floor_sender = deps.redraw_floor_sender.clone();

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
            show_timestamps_sender: show_timestamps_sender.clone(),
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
        let shared_preferences = SharedPreferences::new(Rc::clone(&deps.preferences));

        let choreography_settings_view_model =
            Rc::new(RefCell::new(ChoreographySettingsViewModel::default()));
        choreography_settings_view_model
            .borrow_mut()
            .set_self_handle(Rc::downgrade(&choreography_settings_view_model));

        let (reload_settings_sender, reload_settings_receiver) = crossbeam_channel::unbounded();
        let (update_selected_scene_sender, update_selected_scene_receiver) =
            crossbeam_channel::unbounded();
        let (update_comment_sender, update_comment_receiver) = crossbeam_channel::unbounded();
        let (update_name_sender, update_name_receiver) = crossbeam_channel::unbounded();
        let (update_subtitle_sender, update_subtitle_receiver) = crossbeam_channel::unbounded();
        let (update_date_sender, update_date_receiver) = crossbeam_channel::unbounded();
        let (update_variation_sender, update_variation_receiver) = crossbeam_channel::unbounded();
        let (update_author_sender, update_author_receiver) = crossbeam_channel::unbounded();
        let (update_description_sender, update_description_receiver) = crossbeam_channel::unbounded();
        let (update_floor_front_sender, update_floor_front_receiver) = crossbeam_channel::unbounded();
        let (update_floor_back_sender, update_floor_back_receiver) = crossbeam_channel::unbounded();
        let (update_floor_left_sender, update_floor_left_receiver) = crossbeam_channel::unbounded();
        let (update_floor_right_sender, update_floor_right_receiver) = crossbeam_channel::unbounded();
        let (update_grid_resolution_sender, update_grid_resolution_receiver) =
            crossbeam_channel::unbounded();
        let (update_draw_path_from_sender, update_draw_path_from_receiver) =
            crossbeam_channel::unbounded();
        let (update_draw_path_to_sender, update_draw_path_to_receiver) = crossbeam_channel::unbounded();
        let (update_grid_lines_sender, update_grid_lines_receiver) = crossbeam_channel::unbounded();
        let (update_snap_to_grid_sender, update_snap_to_grid_receiver) =
            crossbeam_channel::unbounded();
        let (update_floor_color_sender, update_floor_color_receiver) = crossbeam_channel::unbounded();
        let (update_show_timestamps_sender, update_show_timestamps_receiver) =
            crossbeam_channel::unbounded();
        let (update_show_legend_sender, update_show_legend_receiver) = crossbeam_channel::unbounded();
        let (update_positions_at_side_sender, update_positions_at_side_receiver) =
            crossbeam_channel::unbounded();
        let (update_transparency_sender, update_transparency_receiver) = crossbeam_channel::unbounded();

        let load_choreography_settings_behavior = LoadChoreographySettingsBehavior::new_with_receiver(
            Rc::clone(&deps.global_state_store),
            reload_settings_receiver,
        );
        let update_selected_scene_behavior = UpdateSelectedSceneBehavior::new_with_receiver(
            Rc::clone(&deps.global_state_store),
            update_selected_scene_receiver,
        );
        let update_comment_behavior = UpdateCommentBehavior::new_with_receiver(
            Rc::clone(&deps.global_state_store),
            redraw_floor_sender.clone(),
            update_comment_receiver,
        );
        let update_name_behavior = UpdateNameBehavior::new_with_receiver(
            Rc::clone(&deps.global_state_store),
            redraw_floor_sender.clone(),
            update_name_receiver,
        );
        let update_subtitle_behavior = UpdateSubtitleBehavior::new_with_receiver(
            Rc::clone(&deps.global_state_store),
            redraw_floor_sender.clone(),
            update_subtitle_receiver,
        );
        let update_date_behavior = UpdateDateBehavior::new_with_receiver(
            Rc::clone(&deps.global_state_store),
            redraw_floor_sender.clone(),
            update_date_receiver,
        );
        let update_variation_behavior = UpdateVariationBehavior::new_with_receiver(
            Rc::clone(&deps.global_state_store),
            redraw_floor_sender.clone(),
            update_variation_receiver,
        );
        let update_author_behavior = UpdateAuthorBehavior::new_with_receiver(
            Rc::clone(&deps.global_state_store),
            redraw_floor_sender.clone(),
            update_author_receiver,
        );
        let update_description_behavior = UpdateDescriptionBehavior::new_with_receiver(
            Rc::clone(&deps.global_state_store),
            redraw_floor_sender.clone(),
            update_description_receiver,
        );
        let update_floor_front_behavior = UpdateFloorFrontBehavior::new_with_receiver(
            Rc::clone(&deps.global_state_store),
            redraw_floor_sender.clone(),
            update_floor_front_receiver,
        );
        let update_floor_back_behavior = UpdateFloorBackBehavior::new_with_receiver(
            Rc::clone(&deps.global_state_store),
            redraw_floor_sender.clone(),
            update_floor_back_receiver,
        );
        let update_floor_left_behavior = UpdateFloorLeftBehavior::new_with_receiver(
            Rc::clone(&deps.global_state_store),
            redraw_floor_sender.clone(),
            update_floor_left_receiver,
        );
        let update_floor_right_behavior = UpdateFloorRightBehavior::new_with_receiver(
            Rc::clone(&deps.global_state_store),
            redraw_floor_sender.clone(),
            update_floor_right_receiver,
        );
        let update_grid_resolution_behavior = UpdateGridResolutionBehavior::new_with_receiver(
            Rc::clone(&deps.global_state_store),
            redraw_floor_sender.clone(),
            update_grid_resolution_receiver,
        );
        let update_draw_path_from_behavior = UpdateDrawPathFromBehavior::new_with_receiver(
            shared_preferences.clone(),
            redraw_floor_sender.clone(),
            update_draw_path_from_receiver,
        );
        let update_draw_path_to_behavior = UpdateDrawPathToBehavior::new_with_receiver(
            shared_preferences.clone(),
            redraw_floor_sender.clone(),
            update_draw_path_to_receiver,
        );
        let update_grid_lines_behavior = UpdateGridLinesBehavior::new_with_receiver(
            Rc::clone(&deps.global_state_store),
            redraw_floor_sender.clone(),
            update_grid_lines_receiver,
        );
        let update_snap_to_grid_behavior = UpdateSnapToGridBehavior::new_with_receiver(
            Rc::clone(&deps.global_state_store),
            shared_preferences.clone(),
            redraw_floor_sender.clone(),
            update_snap_to_grid_receiver,
        );
        let update_floor_color_behavior = UpdateFloorColorBehavior::new_with_receiver(
            Rc::clone(&deps.global_state_store),
            redraw_floor_sender.clone(),
            update_floor_color_receiver,
        );
        let update_show_timestamps_behavior = UpdateShowTimestampsBehavior::new_with_receiver(
            Rc::clone(&deps.global_state_store),
            shared_preferences.clone(),
            redraw_floor_sender.clone(),
            show_timestamps_sender.clone(),
            update_show_timestamps_receiver,
        );
        let update_show_legend_behavior = UpdateShowLegendBehavior::new_with_receiver(
            shared_preferences.clone(),
            redraw_floor_sender.clone(),
            update_show_legend_receiver,
        );
        let update_positions_at_side_behavior = UpdatePositionsAtSideBehavior::new_with_receiver(
            Rc::clone(&deps.global_state_store),
            shared_preferences,
            redraw_floor_sender.clone(),
            update_positions_at_side_receiver,
        );
        let update_transparency_behavior = UpdateTransparencyBehavior::new_with_receiver(
            Rc::clone(&deps.global_state_store),
            redraw_floor_sender.clone(),
            update_transparency_receiver,
        );

        let mut choreography_settings_disposables = CompositeDisposable::new();
        {
            let mut choreography_settings_view_model = choreography_settings_view_model.borrow_mut();
            load_choreography_settings_behavior.activate(
                &mut choreography_settings_view_model,
                &mut choreography_settings_disposables,
            );
            update_selected_scene_behavior.activate(
                &mut choreography_settings_view_model,
                &mut choreography_settings_disposables,
            );
            update_comment_behavior.activate(
                &mut choreography_settings_view_model,
                &mut choreography_settings_disposables,
            );
            update_name_behavior.activate(
                &mut choreography_settings_view_model,
                &mut choreography_settings_disposables,
            );
            update_subtitle_behavior.activate(
                &mut choreography_settings_view_model,
                &mut choreography_settings_disposables,
            );
            update_date_behavior.activate(
                &mut choreography_settings_view_model,
                &mut choreography_settings_disposables,
            );
            update_variation_behavior.activate(
                &mut choreography_settings_view_model,
                &mut choreography_settings_disposables,
            );
            update_author_behavior.activate(
                &mut choreography_settings_view_model,
                &mut choreography_settings_disposables,
            );
            update_description_behavior.activate(
                &mut choreography_settings_view_model,
                &mut choreography_settings_disposables,
            );
            update_floor_front_behavior.activate(
                &mut choreography_settings_view_model,
                &mut choreography_settings_disposables,
            );
            update_floor_back_behavior.activate(
                &mut choreography_settings_view_model,
                &mut choreography_settings_disposables,
            );
            update_floor_left_behavior.activate(
                &mut choreography_settings_view_model,
                &mut choreography_settings_disposables,
            );
            update_floor_right_behavior.activate(
                &mut choreography_settings_view_model,
                &mut choreography_settings_disposables,
            );
            update_grid_resolution_behavior.activate(
                &mut choreography_settings_view_model,
                &mut choreography_settings_disposables,
            );
            update_draw_path_from_behavior.activate(
                &mut choreography_settings_view_model,
                &mut choreography_settings_disposables,
            );
            update_draw_path_to_behavior.activate(
                &mut choreography_settings_view_model,
                &mut choreography_settings_disposables,
            );
            update_grid_lines_behavior.activate(
                &mut choreography_settings_view_model,
                &mut choreography_settings_disposables,
            );
            update_snap_to_grid_behavior.activate(
                &mut choreography_settings_view_model,
                &mut choreography_settings_disposables,
            );
            update_floor_color_behavior.activate(
                &mut choreography_settings_view_model,
                &mut choreography_settings_disposables,
            );
            update_show_timestamps_behavior.activate(
                &mut choreography_settings_view_model,
                &mut choreography_settings_disposables,
            );
            update_show_legend_behavior.activate(
                &mut choreography_settings_view_model,
                &mut choreography_settings_disposables,
            );
            update_positions_at_side_behavior.activate(
                &mut choreography_settings_view_model,
                &mut choreography_settings_disposables,
            );
            update_transparency_behavior.activate(
                &mut choreography_settings_view_model,
                &mut choreography_settings_disposables,
            );
        }

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
            let choreography_settings_view_model_for_change =
                Rc::clone(&choreography_settings_view_model);
            let reload_settings_sender = reload_settings_sender.clone();
            let update_selected_scene_sender = update_selected_scene_sender.clone();
            let view_weak = view_weak.clone();
            let on_change = Rc::new(move || {
                let scenes_view_model = Rc::clone(&scenes_view_model_for_change);
                let floor_provider = Rc::clone(&floor_provider_for_change);
                let choreography_settings_view_model =
                    Rc::clone(&choreography_settings_view_model_for_change);
                let reload_settings_sender = reload_settings_sender.clone();
                let update_selected_scene_sender = update_selected_scene_sender.clone();
                let view_weak = view_weak.clone();
                slint::Timer::single_shot(std::time::Duration::from_millis(0), move || {
                    if let Some(view) = view_weak.upgrade()
                    {
                        let scenes_view_model_snapshot = scenes_view_model.borrow();
                        apply_scenes_view_model(&view, &scenes_view_model_snapshot);
                        drop(scenes_view_model_snapshot);
                        floor_provider.borrow().apply_to_view(&view);
                        let _ = reload_settings_sender.send(ReloadChoreographySettingsCommand);
                        let _ = update_selected_scene_sender
                            .send(UpdateSelectedSceneCommand::SyncFromSelected);
                        let choreography_settings_view_model =
                            choreography_settings_view_model.borrow();
                        apply_choreography_settings_view_model(
                            &view,
                            &choreography_settings_view_model,
                        );
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

        {
            let choreography_settings_view_model_for_change =
                Rc::clone(&choreography_settings_view_model);
            let view_weak = view_weak.clone();
            let on_change = Rc::new(move || {
                let choreography_settings_view_model =
                    Rc::clone(&choreography_settings_view_model_for_change);
                let view_weak = view_weak.clone();
                slint::Timer::single_shot(std::time::Duration::from_millis(0), move || {
                    if let Some(view) = view_weak.upgrade() {
                        let choreography_settings_view_model = choreography_settings_view_model.borrow();
                        apply_choreography_settings_view_model(&view, &choreography_settings_view_model);
                    }
                });
            });
            choreography_settings_view_model
                .borrow_mut()
                .set_on_change(Some(on_change));
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
        apply_choreography_settings_view_model(&view, &choreography_settings_view_model.borrow());
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
            let choreography_settings_view_model = Rc::clone(&choreography_settings_view_model);
            let update_selected_scene_sender = update_selected_scene_sender.clone();
            let view_weak = view_weak.clone();
            view.on_choreo_update_scene_name(move |value| {
                let mut choreography_settings_view_model = choreography_settings_view_model.borrow_mut();
                choreography_settings_view_model.scene_name = value.to_string();
                let _ = update_selected_scene_sender.send(UpdateSelectedSceneCommand::SceneName(
                    choreography_settings_view_model.scene_name.clone(),
                ));
                if let Some(view) = view_weak.upgrade() {
                    apply_choreography_settings_view_model(&view, &choreography_settings_view_model);
                }
            });
        }

        {
            let choreography_settings_view_model = Rc::clone(&choreography_settings_view_model);
            let update_selected_scene_sender = update_selected_scene_sender.clone();
            let view_weak = view_weak.clone();
            view.on_choreo_update_scene_text(move |value| {
                let mut choreography_settings_view_model = choreography_settings_view_model.borrow_mut();
                choreography_settings_view_model.scene_text = value.to_string();
                let _ = update_selected_scene_sender.send(UpdateSelectedSceneCommand::SceneText(
                    choreography_settings_view_model.scene_text.clone(),
                ));
                if let Some(view) = view_weak.upgrade() {
                    apply_choreography_settings_view_model(&view, &choreography_settings_view_model);
                }
            });
        }

        {
            let choreography_settings_view_model = Rc::clone(&choreography_settings_view_model);
            let update_selected_scene_sender = update_selected_scene_sender.clone();
            let view_weak = view_weak.clone();
            view.on_choreo_update_scene_fixed_positions(move |value| {
                let mut choreography_settings_view_model = choreography_settings_view_model.borrow_mut();
                choreography_settings_view_model.scene_fixed_positions = value;
                let _ = update_selected_scene_sender.send(
                    UpdateSelectedSceneCommand::SceneFixedPositions(value),
                );
                if let Some(view) = view_weak.upgrade() {
                    apply_choreography_settings_view_model(&view, &choreography_settings_view_model);
                }
            });
        }

        {
            let choreography_settings_view_model = Rc::clone(&choreography_settings_view_model);
            let update_selected_scene_sender = update_selected_scene_sender.clone();
            let view_weak = view_weak.clone();
            view.on_choreo_update_scene_has_timestamp(move |value| {
                let mut choreography_settings_view_model = choreography_settings_view_model.borrow_mut();
                choreography_settings_view_model.scene_has_timestamp = value;
                let _ = update_selected_scene_sender.send(UpdateSelectedSceneCommand::SceneTimestamp {
                    has_timestamp: choreography_settings_view_model.scene_has_timestamp,
                    seconds: choreography_settings_view_model.scene_timestamp_seconds,
                });
                if let Some(view) = view_weak.upgrade() {
                    apply_choreography_settings_view_model(&view, &choreography_settings_view_model);
                }
            });
        }

        {
            let choreography_settings_view_model = Rc::clone(&choreography_settings_view_model);
            let update_selected_scene_sender = update_selected_scene_sender.clone();
            let view_weak = view_weak.clone();
            view.on_choreo_update_scene_timestamp_parts(move |minutes, seconds, millis| {
                let mut choreography_settings_view_model = choreography_settings_view_model.borrow_mut();
                choreography_settings_view_model.set_scene_timestamp_parts(minutes, seconds, millis);
                let _ = update_selected_scene_sender.send(UpdateSelectedSceneCommand::SceneTimestamp {
                    has_timestamp: choreography_settings_view_model.scene_has_timestamp,
                    seconds: choreography_settings_view_model.scene_timestamp_seconds,
                });
                if let Some(view) = view_weak.upgrade() {
                    apply_choreography_settings_view_model(&view, &choreography_settings_view_model);
                }
            });
        }

        {
            let choreography_settings_view_model = Rc::clone(&choreography_settings_view_model);
            let update_selected_scene_sender = update_selected_scene_sender.clone();
            let view_weak = view_weak.clone();
            view.on_choreo_update_scene_color(move |value| {
                let mut choreography_settings_view_model = choreography_settings_view_model.borrow_mut();
                choreography_settings_view_model.scene_color = choreo_color_from_slint(value);
                let _ = update_selected_scene_sender.send(UpdateSelectedSceneCommand::SceneColor(
                    choreography_settings_view_model.scene_color.clone(),
                ));
                if let Some(view) = view_weak.upgrade() {
                    apply_choreography_settings_view_model(&view, &choreography_settings_view_model);
                }
            });
        }

        {
            let choreography_settings_view_model = Rc::clone(&choreography_settings_view_model);
            let update_comment_sender = update_comment_sender.clone();
            let view_weak = view_weak.clone();
            view.on_choreo_update_comment(move |value| {
                let mut choreography_settings_view_model = choreography_settings_view_model.borrow_mut();
                choreography_settings_view_model.comment = value.to_string();
                let _ = update_comment_sender.send(UpdateCommentCommand {
                    value: choreography_settings_view_model.comment.clone(),
                });
                if let Some(view) = view_weak.upgrade() {
                    apply_choreography_settings_view_model(&view, &choreography_settings_view_model);
                }
            });
        }

        {
            let choreography_settings_view_model = Rc::clone(&choreography_settings_view_model);
            let update_name_sender = update_name_sender.clone();
            let view_weak = view_weak.clone();
            view.on_choreo_update_name(move |value| {
                let mut choreography_settings_view_model = choreography_settings_view_model.borrow_mut();
                choreography_settings_view_model.name = value.to_string();
                let _ = update_name_sender.send(UpdateNameCommand {
                    value: choreography_settings_view_model.name.clone(),
                });
                if let Some(view) = view_weak.upgrade() {
                    apply_choreography_settings_view_model(&view, &choreography_settings_view_model);
                }
            });
        }

        {
            let choreography_settings_view_model = Rc::clone(&choreography_settings_view_model);
            let update_subtitle_sender = update_subtitle_sender.clone();
            let view_weak = view_weak.clone();
            view.on_choreo_update_subtitle(move |value| {
                let mut choreography_settings_view_model = choreography_settings_view_model.borrow_mut();
                choreography_settings_view_model.subtitle = value.to_string();
                let _ = update_subtitle_sender.send(UpdateSubtitleCommand {
                    value: choreography_settings_view_model.subtitle.clone(),
                });
                if let Some(view) = view_weak.upgrade() {
                    apply_choreography_settings_view_model(&view, &choreography_settings_view_model);
                }
            });
        }

        {
            let choreography_settings_view_model = Rc::clone(&choreography_settings_view_model);
            let update_date_sender = update_date_sender.clone();
            let view_weak = view_weak.clone();
            view.on_choreo_update_date_parts(move |year, month, day| {
                let Some(new_date) = crate::date::build_date(year, month, day) else {
                    return;
                };
                let mut choreography_settings_view_model = choreography_settings_view_model.borrow_mut();
                choreography_settings_view_model.date = new_date;
                let _ = update_date_sender.send(UpdateDateCommand {
                    value: choreography_settings_view_model.date,
                });
                if let Some(view) = view_weak.upgrade() {
                    apply_choreography_settings_view_model(&view, &choreography_settings_view_model);
                }
            });
        }

        {
            let choreography_settings_view_model = Rc::clone(&choreography_settings_view_model);
            let update_variation_sender = update_variation_sender.clone();
            let view_weak = view_weak.clone();
            view.on_choreo_update_variation(move |value| {
                let mut choreography_settings_view_model = choreography_settings_view_model.borrow_mut();
                choreography_settings_view_model.variation = value.to_string();
                let _ = update_variation_sender.send(UpdateVariationCommand {
                    value: choreography_settings_view_model.variation.clone(),
                });
                if let Some(view) = view_weak.upgrade() {
                    apply_choreography_settings_view_model(&view, &choreography_settings_view_model);
                }
            });
        }

        {
            let choreography_settings_view_model = Rc::clone(&choreography_settings_view_model);
            let update_author_sender = update_author_sender.clone();
            let view_weak = view_weak.clone();
            view.on_choreo_update_author(move |value| {
                let mut choreography_settings_view_model = choreography_settings_view_model.borrow_mut();
                choreography_settings_view_model.author = value.to_string();
                let _ = update_author_sender.send(UpdateAuthorCommand {
                    value: choreography_settings_view_model.author.clone(),
                });
                if let Some(view) = view_weak.upgrade() {
                    apply_choreography_settings_view_model(&view, &choreography_settings_view_model);
                }
            });
        }

        {
            let choreography_settings_view_model = Rc::clone(&choreography_settings_view_model);
            let update_description_sender = update_description_sender.clone();
            let view_weak = view_weak.clone();
            view.on_choreo_update_description(move |value| {
                let mut choreography_settings_view_model = choreography_settings_view_model.borrow_mut();
                choreography_settings_view_model.description = value.to_string();
                let _ = update_description_sender.send(UpdateDescriptionCommand {
                    value: choreography_settings_view_model.description.clone(),
                });
                if let Some(view) = view_weak.upgrade() {
                    apply_choreography_settings_view_model(&view, &choreography_settings_view_model);
                }
            });
        }

        {
            let choreography_settings_view_model = Rc::clone(&choreography_settings_view_model);
            let update_positions_at_side_sender = update_positions_at_side_sender.clone();
            let view_weak = view_weak.clone();
            view.on_choreo_update_positions_at_side(move |value| {
                let mut choreography_settings_view_model = choreography_settings_view_model.borrow_mut();
                choreography_settings_view_model.positions_at_side = value;
                let _ = update_positions_at_side_sender.send(UpdatePositionsAtSideCommand {
                    value,
                });
                if let Some(view) = view_weak.upgrade() {
                    apply_choreography_settings_view_model(&view, &choreography_settings_view_model);
                }
            });
        }

        {
            let choreography_settings_view_model = Rc::clone(&choreography_settings_view_model);
            let update_draw_path_from_sender = update_draw_path_from_sender.clone();
            let view_weak = view_weak.clone();
            view.on_choreo_update_draw_path_from(move |value| {
                let choreography_settings_view_model = choreography_settings_view_model.borrow();
                let _ = update_draw_path_from_sender.send(UpdateDrawPathFromCommand {
                    value,
                });
                if let Some(view) = view_weak.upgrade() {
                    apply_choreography_settings_view_model(&view, &choreography_settings_view_model);
                }
            });
        }

        {
            let choreography_settings_view_model = Rc::clone(&choreography_settings_view_model);
            let update_draw_path_to_sender = update_draw_path_to_sender.clone();
            let view_weak = view_weak.clone();
            view.on_choreo_update_draw_path_to(move |value| {
                let choreography_settings_view_model = choreography_settings_view_model.borrow();
                let _ = update_draw_path_to_sender.send(UpdateDrawPathToCommand {
                    value,
                });
                if let Some(view) = view_weak.upgrade() {
                    apply_choreography_settings_view_model(&view, &choreography_settings_view_model);
                }
            });
        }

        {
            let choreography_settings_view_model = Rc::clone(&choreography_settings_view_model);
            let update_grid_lines_sender = update_grid_lines_sender.clone();
            let view_weak = view_weak.clone();
            view.on_choreo_update_grid_lines(move |value| {
                let mut choreography_settings_view_model = choreography_settings_view_model.borrow_mut();
                choreography_settings_view_model.grid_lines = value;
                let _ = update_grid_lines_sender.send(UpdateGridLinesCommand {
                    value,
                });
                if let Some(view) = view_weak.upgrade() {
                    apply_choreography_settings_view_model(&view, &choreography_settings_view_model);
                }
            });
        }

        {
            let choreography_settings_view_model = Rc::clone(&choreography_settings_view_model);
            let update_snap_to_grid_sender = update_snap_to_grid_sender.clone();
            let view_weak = view_weak.clone();
            view.on_choreo_update_snap_to_grid(move |value| {
                let choreography_settings_view_model = choreography_settings_view_model.borrow();
                let _ = update_snap_to_grid_sender.send(UpdateSnapToGridCommand {
                    value,
                });
                if let Some(view) = view_weak.upgrade() {
                    apply_choreography_settings_view_model(&view, &choreography_settings_view_model);
                }
            });
        }

        {
            let choreography_settings_view_model = Rc::clone(&choreography_settings_view_model);
            let update_show_timestamps_sender = update_show_timestamps_sender.clone();
            let view_weak = view_weak.clone();
            view.on_choreo_update_show_timestamps(move |value| {
                let choreography_settings_view_model = choreography_settings_view_model.borrow();
                let _ = update_show_timestamps_sender.send(UpdateShowTimestampsCommand {
                    value,
                });
                if let Some(view) = view_weak.upgrade() {
                    apply_choreography_settings_view_model(&view, &choreography_settings_view_model);
                }
            });
        }

        {
            let choreography_settings_view_model = Rc::clone(&choreography_settings_view_model);
            let update_show_legend_sender = update_show_legend_sender.clone();
            let view_weak = view_weak.clone();
            view.on_choreo_update_show_legend(move |value| {
                let choreography_settings_view_model = choreography_settings_view_model.borrow();
                let _ = update_show_legend_sender.send(UpdateShowLegendCommand {
                    value,
                });
                if let Some(view) = view_weak.upgrade() {
                    apply_choreography_settings_view_model(&view, &choreography_settings_view_model);
                }
            });
        }

        {
            let choreography_settings_view_model = Rc::clone(&choreography_settings_view_model);
            let update_transparency_sender = update_transparency_sender.clone();
            let view_weak = view_weak.clone();
            view.on_choreo_update_transparency(move |value| {
                let mut choreography_settings_view_model = choreography_settings_view_model.borrow_mut();
                choreography_settings_view_model.transparency = value as f64;
                let _ = update_transparency_sender.send(UpdateTransparencyCommand {
                    value: value as f64,
                });
                if let Some(view) = view_weak.upgrade() {
                    apply_choreography_settings_view_model(&view, &choreography_settings_view_model);
                }
            });
        }

        {
            let choreography_settings_view_model = Rc::clone(&choreography_settings_view_model);
            let update_floor_color_sender = update_floor_color_sender.clone();
            let view_weak = view_weak.clone();
            view.on_choreo_update_floor_color(move |value| {
                let mut choreography_settings_view_model = choreography_settings_view_model.borrow_mut();
                let floor_color = choreo_color_from_slint(value);
                choreography_settings_view_model.floor_color = floor_color.clone();
                let _ = update_floor_color_sender.send(UpdateFloorColorCommand {
                    value: floor_color,
                });
                if let Some(view) = view_weak.upgrade() {
                    apply_choreography_settings_view_model(&view, &choreography_settings_view_model);
                }
            });
        }

        {
            let choreography_settings_view_model = Rc::clone(&choreography_settings_view_model);
            let update_floor_front_sender = update_floor_front_sender.clone();
            let view_weak = view_weak.clone();
            view.on_choreo_update_floor_front(move |value| {
                let mut choreography_settings_view_model = choreography_settings_view_model.borrow_mut();
                let clamped = value.clamp(1, 100);
                choreography_settings_view_model.floor_front = clamped;
                let _ = update_floor_front_sender.send(UpdateFloorFrontCommand {
                    value: clamped,
                });
                if let Some(view) = view_weak.upgrade() {
                    apply_choreography_settings_view_model(&view, &choreography_settings_view_model);
                }
            });
        }

        {
            let choreography_settings_view_model = Rc::clone(&choreography_settings_view_model);
            let update_floor_back_sender = update_floor_back_sender.clone();
            let view_weak = view_weak.clone();
            view.on_choreo_update_floor_back(move |value| {
                let mut choreography_settings_view_model = choreography_settings_view_model.borrow_mut();
                let clamped = value.clamp(1, 100);
                choreography_settings_view_model.floor_back = clamped;
                let _ = update_floor_back_sender.send(UpdateFloorBackCommand {
                    value: clamped,
                });
                if let Some(view) = view_weak.upgrade() {
                    apply_choreography_settings_view_model(&view, &choreography_settings_view_model);
                }
            });
        }

        {
            let choreography_settings_view_model = Rc::clone(&choreography_settings_view_model);
            let update_floor_left_sender = update_floor_left_sender.clone();
            let view_weak = view_weak.clone();
            view.on_choreo_update_floor_left(move |value| {
                let mut choreography_settings_view_model = choreography_settings_view_model.borrow_mut();
                let clamped = value.clamp(1, 100);
                choreography_settings_view_model.floor_left = clamped;
                let _ = update_floor_left_sender.send(UpdateFloorLeftCommand {
                    value: clamped,
                });
                if let Some(view) = view_weak.upgrade() {
                    apply_choreography_settings_view_model(&view, &choreography_settings_view_model);
                }
            });
        }

        {
            let choreography_settings_view_model = Rc::clone(&choreography_settings_view_model);
            let update_floor_right_sender = update_floor_right_sender.clone();
            let view_weak = view_weak.clone();
            view.on_choreo_update_floor_right(move |value| {
                let mut choreography_settings_view_model = choreography_settings_view_model.borrow_mut();
                let clamped = value.clamp(1, 100);
                choreography_settings_view_model.floor_right = clamped;
                let _ = update_floor_right_sender.send(UpdateFloorRightCommand {
                    value: clamped,
                });
                if let Some(view) = view_weak.upgrade() {
                    apply_choreography_settings_view_model(&view, &choreography_settings_view_model);
                }
            });
        }

        {
            let choreography_settings_view_model = Rc::clone(&choreography_settings_view_model);
            let update_grid_resolution_sender = update_grid_resolution_sender.clone();
            let view_weak = view_weak.clone();
            view.on_choreo_update_grid_resolution(move |value| {
                let mut choreography_settings_view_model = choreography_settings_view_model.borrow_mut();
                choreography_settings_view_model.set_grid_resolution(value);
                let _ = update_grid_resolution_sender.send(UpdateGridResolutionCommand {
                    value,
                });
                if let Some(view) = view_weak.upgrade() {
                    apply_choreography_settings_view_model(&view, &choreography_settings_view_model);
                }
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

        {
            let floor_view_model = Rc::clone(&floor_view_model);
            view.on_floor_redraw(move || {
                floor_view_model.borrow_mut().draw_floor();
            });
        }

        Self {
            view,
            view_model,
            scenes_view_model,
            settings_view_model,
            choreography_settings_view_model,
            floor_view_model,
            floor_provider,
            nav_bar_timer,
            choreography_settings_disposables,
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

fn apply_choreography_settings_view_model(
    view: &ShellHost,
    view_model: &ChoreographySettingsViewModel,
) {
    view.set_choreo_has_selected_scene(view_model.has_selected_scene);
    view.set_choreo_scene_name(view_model.scene_name.as_str().into());
    view.set_choreo_scene_text(view_model.scene_text.as_str().into());
    view.set_choreo_scene_fixed_positions(view_model.scene_fixed_positions);
    view.set_choreo_scene_has_timestamp(view_model.scene_has_timestamp);
    view.set_choreo_scene_timestamp_minutes(view_model.scene_timestamp_minutes);
    view.set_choreo_scene_timestamp_seconds(view_model.scene_timestamp_seconds_part);
    view.set_choreo_scene_timestamp_millis(view_model.scene_timestamp_millis);
    view.set_choreo_scene_color(slint_color_from_choreo(&view_model.scene_color));

    view.set_choreo_comment(view_model.comment.as_str().into());
    view.set_choreo_name(view_model.name.as_str().into());
    view.set_choreo_subtitle(view_model.subtitle.as_str().into());
    view.set_choreo_date(slint_date_from_time(view_model.date));
    view.set_choreo_variation(view_model.variation.as_str().into());
    view.set_choreo_author(view_model.author.as_str().into());
    view.set_choreo_description(view_model.description.as_str().into());

    view.set_choreo_positions_at_side(view_model.positions_at_side);
    view.set_choreo_draw_path_from(view_model.draw_path_from);
    view.set_choreo_draw_path_to(view_model.draw_path_to);
    view.set_choreo_grid_lines(view_model.grid_lines);
    view.set_choreo_snap_to_grid(view_model.snap_to_grid);
    view.set_choreo_show_timestamps(view_model.show_timestamps);
    view.set_choreo_show_legend(view_model.show_legend);
    view.set_choreo_transparency(view_model.transparency as f32);
    view.set_choreo_floor_color(slint_color_from_choreo(&view_model.floor_color));

    view.set_choreo_floor_size_options(ModelRc::new(VecModel::from(
        view_model.floor_size_options.clone(),
    )));
    view.set_choreo_floor_front(view_model.floor_front);
    view.set_choreo_floor_back(view_model.floor_back);
    view.set_choreo_floor_left(view_model.floor_left);
    view.set_choreo_floor_right(view_model.floor_right);

    let grid_resolution_index = view_model.grid_resolution().saturating_sub(1);
    view.set_choreo_grid_resolution_index(grid_resolution_index);
    view.set_choreo_grid_size_options(ModelRc::new(VecModel::from(
        build_grid_menu_items(&view_model.grid_size_options),
    )));
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

fn build_grid_menu_items(options: &[crate::choreography_settings::GridSizeOption]) -> Vec<MenuItem> {
    options
        .iter()
        .map(|option| MenuItem {
            icon: Image::default(),
            text: option.display.as_str().into(),
            trailing_text: "".into(),
            enabled: true,
        })
        .collect()
}

fn slint_date_from_time(value: time::Date) -> Date {
    Date {
        day: i32::from(value.day()),
        month: i32::from(value.month() as u8),
        year: value.year(),
    }
}

fn slint_color_from_choreo(color: &ChoreoColor) -> Color {
    Color::from_argb_u8(color.a, color.r, color.g, color.b)
}

fn choreo_color_from_slint(color: Color) -> ChoreoColor {
    ChoreoColor {
        r: color.red(),
        g: color.green(),
        b: color.blue(),
        a: color.alpha(),
    }
}

