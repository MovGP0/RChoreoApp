use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;
use std::time::Instant;

use choreo_master_mobile_json::Color as ChoreoColor;
use crossbeam_channel::{Receiver, Sender, bounded};
use slint::{Color, ComponentHandle, Image, ModelRc, VecModel};

use crate::audio_player::{
    AudioPlayerPositionChangedEvent, AudioPlayerViewModel, AudioPlayerViewState,
    OpenAudioFileCommand, build_tick_values,
};
use crate::behavior::{Behavior, CompositeDisposable};
use crate::choreography_settings::{
    ChoreographySettingsViewModel, LoadChoreographySettingsBehavior,
    ReloadChoreographySettingsCommand, UpdateAuthorBehavior, UpdateAuthorCommand,
    UpdateCommentBehavior, UpdateCommentCommand, UpdateDateBehavior, UpdateDateCommand,
    UpdateDescriptionBehavior, UpdateDescriptionCommand, UpdateDrawPathFromBehavior,
    UpdateDrawPathFromCommand, UpdateDrawPathToBehavior, UpdateDrawPathToCommand,
    UpdateFloorBackBehavior, UpdateFloorBackCommand, UpdateFloorColorBehavior,
    UpdateFloorColorCommand, UpdateFloorFrontBehavior, UpdateFloorFrontCommand,
    UpdateFloorLeftBehavior, UpdateFloorLeftCommand, UpdateFloorRightBehavior,
    UpdateFloorRightCommand, UpdateGridLinesBehavior, UpdateGridLinesCommand,
    UpdateGridResolutionBehavior, UpdateGridResolutionCommand, UpdateNameBehavior,
    UpdateNameCommand, UpdatePositionsAtSideBehavior, UpdatePositionsAtSideCommand,
    UpdateSelectedSceneBehavior, UpdateSelectedSceneCommand, UpdateShowLegendBehavior,
    UpdateShowLegendCommand, UpdateShowTimestampsBehavior, UpdateShowTimestampsCommand,
    UpdateSnapToGridBehavior, UpdateSnapToGridCommand, UpdateSubtitleBehavior,
    UpdateSubtitleCommand, UpdateTransparencyBehavior, UpdateTransparencyCommand,
    UpdateVariationBehavior, UpdateVariationCommand,
};
use crate::floor::{
    CanvasViewHandle, FloorCanvasViewModel, FloorProvider, FloorProviderDependencies, Point,
    PointerButton, PointerEventArgs, PointerMovedCommand, PointerPressedCommand,
    PointerReleasedCommand, PointerWheelChangedCommand,
};
use crate::global::{GlobalStateActor, GlobalStateModel};
use crate::haptics::HapticFeedback;
use crate::nav_bar::{
    InteractionModeChangedCommand, NavBarSenders, NavBarViewModel,
    OpenAudioRequestedCommand as NavBarOpenAudioRequestedCommand,
    OpenImageRequestedCommand as NavBarOpenImageRequestedCommand, apply_nav_bar_view_model,
    bind_nav_bar,
};
use crate::observability::start_internal_span;
use crate::preferences::SharedPreferences;
use crate::scenes::{
    OpenChoreoActions, OpenChoreoRequested, ScenesDependencies, ScenesPaneViewModel, ScenesProvider,
};
use crate::settings::{
    MaterialSchemeHelper, SettingsDependencies, SettingsProvider, SettingsViewModel, ThemeMode,
};
use crate::shell::ShellMaterialSchemeApplier;
use crate::time::format_seconds;
use crate::{Date, MenuItem, SceneListItem, ShellHost};
use choreo_state_machine::ApplicationStateMachine;

use super::{
    MainViewModel, MainViewModelProvider, MainViewModelProviderDependencies, OpenAudioRequested,
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
    audio_player: Rc<RefCell<AudioPlayerViewModel>>,
    #[allow(dead_code)]
    nav_bar_timer: slint::Timer,
    #[allow(dead_code)]
    settings_timer: slint::Timer,
    #[allow(dead_code)]
    audio_player_timer: slint::Timer,
    #[allow(dead_code)]
    choreography_settings_disposables: CompositeDisposable,
    open_choreo_sender: Sender<OpenChoreoRequested>,
    open_audio_request_sender: Sender<OpenAudioRequested>,
    open_image_request_sender: Sender<OpenImageRequested>,
}

#[derive(Debug, Clone, Copy)]
struct AudioSyncTraceSample {
    actor_position_seconds: f64,
    view_model_position_seconds: f64,
    should_accept_position: bool,
    is_user_dragging: bool,
    pending_seek_position_seconds: Option<f64>,
    is_playing: bool,
    can_seek: bool,
    was_playing_before_drag: bool,
    is_adjusting_speed: bool,
}

impl AudioSyncTraceSample {
    fn has_meaningful_change(self, previous: Self) -> bool {
        const POSITION_EPSILON_SECONDS: f64 = 0.05;

        (self.actor_position_seconds - previous.actor_position_seconds).abs()
            > POSITION_EPSILON_SECONDS
            || (self.view_model_position_seconds - previous.view_model_position_seconds).abs()
                > POSITION_EPSILON_SECONDS
            || option_f64_changed(
                self.pending_seek_position_seconds,
                previous.pending_seek_position_seconds,
                POSITION_EPSILON_SECONDS,
            )
            || self.should_accept_position != previous.should_accept_position
            || self.is_user_dragging != previous.is_user_dragging
            || self.is_playing != previous.is_playing
            || self.can_seek != previous.can_seek
            || self.was_playing_before_drag != previous.was_playing_before_drag
            || self.is_adjusting_speed != previous.is_adjusting_speed
    }
}

fn option_f64_changed(current: Option<f64>, previous: Option<f64>, epsilon: f64) -> bool {
    match (current, previous) {
        (Some(current), Some(previous)) => (current - previous).abs() > epsilon,
        (None, None) => false,
        _ => true,
    }
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
            redraw_floor_sender: redraw_floor_sender.clone(),
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

        let floor_provider = Rc::new(RefCell::new(FloorProvider::new(
            FloorProviderDependencies {
                global_state: Rc::clone(&deps.global_state),
                global_state_store: Rc::clone(&deps.global_state_store),
                state_machine: Rc::clone(&deps.state_machine),
                preferences: Rc::clone(&deps.preferences),
                audio_position_receiver: deps.audio_position_receiver,
                redraw_floor_receiver: deps.redraw_floor_receiver,
            },
        )));
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
        let (update_description_sender, update_description_receiver) =
            crossbeam_channel::unbounded();
        let (update_floor_front_sender, update_floor_front_receiver) =
            crossbeam_channel::unbounded();
        let (update_floor_back_sender, update_floor_back_receiver) = crossbeam_channel::unbounded();
        let (update_floor_left_sender, update_floor_left_receiver) = crossbeam_channel::unbounded();
        let (update_floor_right_sender, update_floor_right_receiver) =
            crossbeam_channel::unbounded();
        let (update_grid_resolution_sender, update_grid_resolution_receiver) =
            crossbeam_channel::unbounded();
        let (update_draw_path_from_sender, update_draw_path_from_receiver) =
            crossbeam_channel::unbounded();
        let (update_draw_path_to_sender, update_draw_path_to_receiver) =
            crossbeam_channel::unbounded();
        let (update_grid_lines_sender, update_grid_lines_receiver) = crossbeam_channel::unbounded();
        let (update_snap_to_grid_sender, update_snap_to_grid_receiver) =
            crossbeam_channel::unbounded();
        let (update_floor_color_sender, update_floor_color_receiver) =
            crossbeam_channel::unbounded();
        let (update_show_timestamps_sender, update_show_timestamps_receiver) =
            crossbeam_channel::unbounded();
        let (update_show_legend_sender, update_show_legend_receiver) =
            crossbeam_channel::unbounded();
        let (update_positions_at_side_sender, update_positions_at_side_receiver) =
            crossbeam_channel::unbounded();
        let (update_transparency_sender, update_transparency_receiver) =
            crossbeam_channel::unbounded();

        let load_choreography_settings_behavior =
            LoadChoreographySettingsBehavior::new_with_receiver(
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
            let mut choreography_settings_view_model =
                choreography_settings_view_model.borrow_mut();
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

        let view_model = Rc::new(RefCell::new(
            main_provider.create_main_view_model(Rc::clone(&nav_bar)),
        ));
        {
            let view_model_for_change = Rc::clone(&view_model);
            let floor_provider_for_change = Rc::clone(&floor_provider);
            let view_weak = view_weak.clone();
            let on_change = Rc::new(move || {
                let view_model = Rc::clone(&view_model_for_change);
                let floor_provider = Rc::clone(&floor_provider_for_change);
                let view_weak = view_weak.clone();
                slint::Timer::single_shot(std::time::Duration::from_millis(0), move || {
                    if let Some(view) = view_weak.upgrade() {
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
                    if let Some(view) = view_weak.upgrade() {
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
            scenes_view_model
                .borrow_mut()
                .set_on_change(Some(on_change));
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
                        let choreography_settings_view_model =
                            choreography_settings_view_model.borrow();
                        apply_choreography_settings_view_model(
                            &view,
                            &choreography_settings_view_model,
                        );
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
            let global_state_store_for_image = deps.global_state_store.clone();
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
                                trace_context: None,
                            });
                            nav_bar.borrow_mut().set_audio_player_opened(true);
                        }
                    }

                    while nav_bar_open_image_receiver.try_recv().is_ok() {
                        let svg_loaded = global_state_store_for_image
                            .try_with_state(|global_state| {
                                global_state
                                    .svg_file_path
                                    .as_ref()
                                    .is_some_and(|path| !path.trim().is_empty())
                            })
                            .unwrap_or(false);

                        if svg_loaded {
                            let _ = open_image_request_sender.try_send(OpenImageRequested {
                                file_path: String::new(),
                            });
                            continue;
                        }

                        if let Some(requester) = actions_for_image.request_open_image.as_ref() {
                            requester(open_image_request_sender.clone());
                            continue;
                        }

                        if let Some(picker) = actions_for_image.pick_image_path.as_ref()
                            && let Some(path) = picker()
                        {
                            let _ = open_image_request_sender
                                .try_send(OpenImageRequested { file_path: path });
                        }
                    }

                    while let Ok(mode_command) = nav_bar_mode_receiver.try_recv() {
                        let _ = interaction_mode_sender.try_send(mode_command.mode);
                    }
                },
            );
            timer
        };

        let audio_player_view_state = Rc::new(RefCell::new(AudioPlayerViewState::new()));

        let audio_player_timer = {
            let audio_player = Rc::clone(&audio_player);
            let audio_player_view_state = Rc::clone(&audio_player_view_state);
            let global_state_store = Rc::clone(&deps.global_state_store);
            let view_weak = view_weak.clone();
            let last_metrics_emitted_at = Rc::new(RefCell::new(
                Instant::now() - std::time::Duration::from_secs(1),
            ));
            let last_accept_decision = Rc::new(RefCell::new(None::<bool>));
            let last_sync_sample = Rc::new(RefCell::new(None::<AudioSyncTraceSample>));
            let timer = slint::Timer::default();
            timer.start(
                slint::TimerMode::Repeated,
                std::time::Duration::from_millis(16),
                move || {
                    let mut audio_player = audio_player.borrow_mut();
                    let view_model_position_before = audio_player.position;
                    let player_position = audio_player
                        .player
                        .as_ref()
                        .map(|player| player.current_position())
                        .unwrap_or(audio_player.position);

                    let (state_before, should_accept, state_after) = {
                        let mut state = audio_player_view_state.borrow_mut();
                        let before = state.snapshot();
                        let should_accept = state.should_accept_player_position(player_position);
                        let after = state.snapshot();
                        (before, should_accept, after)
                    };

                    {
                        let mut last_accept_decision = last_accept_decision.borrow_mut();
                        if *last_accept_decision != Some(should_accept) {
                            let mut span = start_internal_span(
                                "audio_player.sync_loop.decision_changed",
                                None,
                            );
                            span.set_bool_attribute(
                                "choreo.audio.should_accept_position",
                                should_accept,
                            );
                            span.set_bool_attribute(
                                "choreo.audio.is_user_dragging",
                                state_after.is_user_dragging,
                            );
                            span.set_f64_attribute(
                                "choreo.audio.actor_position_seconds",
                                player_position,
                            );
                            span.set_f64_attribute(
                                "choreo.audio.view_model_position_seconds",
                                view_model_position_before,
                            );
                            if let Some(pending_seek) = state_after.pending_seek_position {
                                span.set_f64_attribute(
                                    "choreo.audio.pending_seek_position_seconds",
                                    pending_seek,
                                );
                            }
                            if let Some(pending_seek_age) = state_after.pending_seek_age_seconds {
                                span.set_f64_attribute(
                                    "choreo.audio.pending_seek_age_seconds",
                                    pending_seek_age,
                                );
                            }
                            *last_accept_decision = Some(should_accept);
                        }
                    }

                    if should_accept {
                        audio_player.sync_from_player();
                    }

                    let current_sample = AudioSyncTraceSample {
                        actor_position_seconds: player_position,
                        view_model_position_seconds: audio_player.position,
                        should_accept_position: should_accept,
                        is_user_dragging: state_after.is_user_dragging,
                        pending_seek_position_seconds: state_after.pending_seek_position,
                        is_playing: audio_player.is_playing,
                        can_seek: audio_player.can_seek,
                        was_playing_before_drag: state_after.was_playing,
                        is_adjusting_speed: state_after.is_adjusting_speed,
                    };
                    let changed = last_sync_sample
                        .borrow()
                        .is_none_or(|previous| current_sample.has_meaningful_change(previous));
                    let now = Instant::now();
                    let emit_metrics = changed
                        && now.duration_since(*last_metrics_emitted_at.borrow())
                            >= std::time::Duration::from_millis(500);
                    if emit_metrics {
                        *last_metrics_emitted_at.borrow_mut() = now;
                        *last_sync_sample.borrow_mut() = Some(current_sample);
                        let mut span = start_internal_span("audio_player.sync_loop.sample", None);
                        span.set_f64_attribute(
                            "choreo.audio.actor_position_seconds",
                            player_position,
                        );
                        span.set_f64_attribute(
                            "choreo.audio.view_model_position_before_seconds",
                            view_model_position_before,
                        );
                        span.set_f64_attribute(
                            "choreo.audio.view_model_position_after_seconds",
                            audio_player.position,
                        );
                        span.set_bool_attribute(
                            "choreo.audio.should_accept_position",
                            should_accept,
                        );
                        span.set_bool_attribute(
                            "choreo.audio.is_user_dragging",
                            state_after.is_user_dragging,
                        );
                        span.set_bool_attribute("choreo.audio.is_playing", audio_player.is_playing);
                        span.set_bool_attribute("choreo.audio.can_seek", audio_player.can_seek);
                        span.set_bool_attribute(
                            "choreo.audio.was_playing_before_drag",
                            state_after.was_playing,
                        );
                        span.set_bool_attribute(
                            "choreo.audio.is_adjusting_speed",
                            state_after.is_adjusting_speed,
                        );
                        if let Some(pending_seek) = state_before.pending_seek_position {
                            span.set_f64_attribute(
                                "choreo.audio.pending_seek_before_seconds",
                                pending_seek,
                            );
                        }
                        if let Some(pending_seek) = state_after.pending_seek_position {
                            span.set_f64_attribute(
                                "choreo.audio.pending_seek_after_seconds",
                                pending_seek,
                            );
                        }
                        if let Some(pending_seek_age) = state_after.pending_seek_age_seconds {
                            span.set_f64_attribute(
                                "choreo.audio.pending_seek_age_seconds",
                                pending_seek_age,
                            );
                        }
                    }

                    if let Some((scenes, selected_scene)) =
                        global_state_store.try_with_state(|global_state| {
                            (
                                global_state.scenes.clone(),
                                global_state.selected_scene.clone(),
                            )
                        })
                    {
                        audio_player.tick_values =
                            build_tick_values(audio_player.duration, &scenes);
                        audio_player.can_link_scene_to_position = selected_scene.is_some();
                    }
                    if let Some(view) = view_weak.upgrade() {
                        apply_audio_player_view_model(&view, &audio_player);
                    }
                },
            );
            timer
        };

        let settings_timer = {
            let settings_view_model = Rc::clone(&settings_view_model);
            let view_weak = view_weak.clone();
            let timer = slint::Timer::default();
            timer.start(
                slint::TimerMode::Repeated,
                std::time::Duration::from_millis(16),
                move || {
                    if let Some(view) = view_weak.upgrade() {
                        let settings_view_model = settings_view_model.borrow();
                        apply_settings_view_model(&view, &settings_view_model);
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
        apply_settings_view_model(&view, &settings_view_model.borrow());
        apply_choreography_settings_view_model(&view, &choreography_settings_view_model.borrow());
        apply_audio_player_view_model(&view, &audio_player.borrow());
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
            let settings_view_model = Rc::clone(&settings_view_model);
            view.on_settings_update_use_system_theme(move |value| {
                settings_view_model
                    .borrow_mut()
                    .update_use_system_theme(value);
            });
        }

        {
            let settings_view_model = Rc::clone(&settings_view_model);
            view.on_settings_update_is_dark_mode(move |value| {
                settings_view_model.borrow_mut().update_is_dark_mode(value);
            });
        }

        {
            let settings_view_model = Rc::clone(&settings_view_model);
            view.on_settings_update_use_primary_color(move |value| {
                settings_view_model
                    .borrow_mut()
                    .update_use_primary_color(value);
            });
        }

        {
            let settings_view_model = Rc::clone(&settings_view_model);
            view.on_settings_update_use_secondary_color(move |value| {
                settings_view_model
                    .borrow_mut()
                    .update_use_secondary_color(value);
            });
        }

        {
            let settings_view_model = Rc::clone(&settings_view_model);
            view.on_settings_update_use_tertiary_color(move |value| {
                settings_view_model
                    .borrow_mut()
                    .update_use_tertiary_color(value);
            });
        }

        {
            let settings_view_model = Rc::clone(&settings_view_model);
            view.on_settings_update_primary_color(move |value| {
                let color = choreo_color_from_slint(value);
                let hex = color.to_hex();
                let mut settings_view_model = settings_view_model.borrow_mut();
                settings_view_model.primary_color = color;
                settings_view_model.update_primary_color_hex(hex);
            });
        }

        {
            let settings_view_model = Rc::clone(&settings_view_model);
            view.on_settings_update_secondary_color(move |value| {
                let color = choreo_color_from_slint(value);
                let hex = color.to_hex();
                let mut settings_view_model = settings_view_model.borrow_mut();
                settings_view_model.secondary_color = color;
                settings_view_model.update_secondary_color_hex(hex);
            });
        }

        {
            let settings_view_model = Rc::clone(&settings_view_model);
            view.on_settings_update_tertiary_color(move |value| {
                let color = choreo_color_from_slint(value);
                let hex = color.to_hex();
                let mut settings_view_model = settings_view_model.borrow_mut();
                settings_view_model.tertiary_color = color;
                settings_view_model.update_tertiary_color_hex(hex);
            });
        }

        {
            let choreography_settings_view_model = Rc::clone(&choreography_settings_view_model);
            let update_selected_scene_sender = update_selected_scene_sender.clone();
            let view_weak = view_weak.clone();
            view.on_choreo_update_scene_name(move |value| {
                let mut choreography_settings_view_model =
                    choreography_settings_view_model.borrow_mut();
                choreography_settings_view_model.scene_name = value.to_string();
                let _ = update_selected_scene_sender.send(UpdateSelectedSceneCommand::SceneName(
                    choreography_settings_view_model.scene_name.clone(),
                ));
                if let Some(view) = view_weak.upgrade() {
                    apply_choreography_settings_view_model(
                        &view,
                        &choreography_settings_view_model,
                    );
                }
            });
        }

        {
            let choreography_settings_view_model = Rc::clone(&choreography_settings_view_model);
            let update_selected_scene_sender = update_selected_scene_sender.clone();
            let view_weak = view_weak.clone();
            view.on_choreo_update_scene_text(move |value| {
                let mut choreography_settings_view_model =
                    choreography_settings_view_model.borrow_mut();
                choreography_settings_view_model.scene_text = value.to_string();
                let _ = update_selected_scene_sender.send(UpdateSelectedSceneCommand::SceneText(
                    choreography_settings_view_model.scene_text.clone(),
                ));
                if let Some(view) = view_weak.upgrade() {
                    apply_choreography_settings_view_model(
                        &view,
                        &choreography_settings_view_model,
                    );
                }
            });
        }

        {
            let choreography_settings_view_model = Rc::clone(&choreography_settings_view_model);
            let update_selected_scene_sender = update_selected_scene_sender.clone();
            let view_weak = view_weak.clone();
            view.on_choreo_update_scene_fixed_positions(move |value| {
                let mut choreography_settings_view_model =
                    choreography_settings_view_model.borrow_mut();
                choreography_settings_view_model.scene_fixed_positions = value;
                let _ = update_selected_scene_sender
                    .send(UpdateSelectedSceneCommand::SceneFixedPositions(value));
                if let Some(view) = view_weak.upgrade() {
                    apply_choreography_settings_view_model(
                        &view,
                        &choreography_settings_view_model,
                    );
                }
            });
        }

        {
            let choreography_settings_view_model = Rc::clone(&choreography_settings_view_model);
            let update_selected_scene_sender = update_selected_scene_sender.clone();
            let view_weak = view_weak.clone();
            view.on_choreo_update_scene_has_timestamp(move |value| {
                let mut choreography_settings_view_model =
                    choreography_settings_view_model.borrow_mut();
                choreography_settings_view_model.scene_has_timestamp = value;
                let _ =
                    update_selected_scene_sender.send(UpdateSelectedSceneCommand::SceneTimestamp {
                        has_timestamp: choreography_settings_view_model.scene_has_timestamp,
                        seconds: choreography_settings_view_model.scene_timestamp_seconds,
                    });
                if let Some(view) = view_weak.upgrade() {
                    apply_choreography_settings_view_model(
                        &view,
                        &choreography_settings_view_model,
                    );
                }
            });
        }

        {
            let choreography_settings_view_model = Rc::clone(&choreography_settings_view_model);
            let update_selected_scene_sender = update_selected_scene_sender.clone();
            let view_weak = view_weak.clone();
            view.on_choreo_update_scene_timestamp_parts(move |minutes, seconds, millis| {
                let mut choreography_settings_view_model =
                    choreography_settings_view_model.borrow_mut();
                choreography_settings_view_model
                    .set_scene_timestamp_parts(minutes, seconds, millis);
                let _ =
                    update_selected_scene_sender.send(UpdateSelectedSceneCommand::SceneTimestamp {
                        has_timestamp: choreography_settings_view_model.scene_has_timestamp,
                        seconds: choreography_settings_view_model.scene_timestamp_seconds,
                    });
                if let Some(view) = view_weak.upgrade() {
                    apply_choreography_settings_view_model(
                        &view,
                        &choreography_settings_view_model,
                    );
                }
            });
        }

        {
            let choreography_settings_view_model = Rc::clone(&choreography_settings_view_model);
            let update_selected_scene_sender = update_selected_scene_sender.clone();
            let view_weak = view_weak.clone();
            view.on_choreo_update_scene_color(move |value| {
                let mut choreography_settings_view_model =
                    choreography_settings_view_model.borrow_mut();
                choreography_settings_view_model.scene_color = choreo_color_from_slint(value);
                let _ = update_selected_scene_sender.send(UpdateSelectedSceneCommand::SceneColor(
                    choreography_settings_view_model.scene_color.clone(),
                ));
                if let Some(view) = view_weak.upgrade() {
                    apply_choreography_settings_view_model(
                        &view,
                        &choreography_settings_view_model,
                    );
                }
            });
        }

        {
            let choreography_settings_view_model = Rc::clone(&choreography_settings_view_model);
            let update_comment_sender = update_comment_sender.clone();
            let view_weak = view_weak.clone();
            view.on_choreo_update_comment(move |value| {
                let mut choreography_settings_view_model =
                    choreography_settings_view_model.borrow_mut();
                choreography_settings_view_model.comment = value.to_string();
                let _ = update_comment_sender.send(UpdateCommentCommand {
                    value: choreography_settings_view_model.comment.clone(),
                });
                if let Some(view) = view_weak.upgrade() {
                    apply_choreography_settings_view_model(
                        &view,
                        &choreography_settings_view_model,
                    );
                }
            });
        }

        {
            let choreography_settings_view_model = Rc::clone(&choreography_settings_view_model);
            let update_name_sender = update_name_sender.clone();
            let view_weak = view_weak.clone();
            view.on_choreo_update_name(move |value| {
                let mut choreography_settings_view_model =
                    choreography_settings_view_model.borrow_mut();
                choreography_settings_view_model.name = value.to_string();
                let _ = update_name_sender.send(UpdateNameCommand {
                    value: choreography_settings_view_model.name.clone(),
                });
                if let Some(view) = view_weak.upgrade() {
                    apply_choreography_settings_view_model(
                        &view,
                        &choreography_settings_view_model,
                    );
                }
            });
        }

        {
            let choreography_settings_view_model = Rc::clone(&choreography_settings_view_model);
            let update_subtitle_sender = update_subtitle_sender.clone();
            let view_weak = view_weak.clone();
            view.on_choreo_update_subtitle(move |value| {
                let mut choreography_settings_view_model =
                    choreography_settings_view_model.borrow_mut();
                choreography_settings_view_model.subtitle = value.to_string();
                let _ = update_subtitle_sender.send(UpdateSubtitleCommand {
                    value: choreography_settings_view_model.subtitle.clone(),
                });
                if let Some(view) = view_weak.upgrade() {
                    apply_choreography_settings_view_model(
                        &view,
                        &choreography_settings_view_model,
                    );
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
                let mut choreography_settings_view_model =
                    choreography_settings_view_model.borrow_mut();
                choreography_settings_view_model.date = new_date;
                let _ = update_date_sender.send(UpdateDateCommand {
                    value: choreography_settings_view_model.date,
                });
                if let Some(view) = view_weak.upgrade() {
                    apply_choreography_settings_view_model(
                        &view,
                        &choreography_settings_view_model,
                    );
                }
            });
        }

        {
            let choreography_settings_view_model = Rc::clone(&choreography_settings_view_model);
            let update_variation_sender = update_variation_sender.clone();
            let view_weak = view_weak.clone();
            view.on_choreo_update_variation(move |value| {
                let mut choreography_settings_view_model =
                    choreography_settings_view_model.borrow_mut();
                choreography_settings_view_model.variation = value.to_string();
                let _ = update_variation_sender.send(UpdateVariationCommand {
                    value: choreography_settings_view_model.variation.clone(),
                });
                if let Some(view) = view_weak.upgrade() {
                    apply_choreography_settings_view_model(
                        &view,
                        &choreography_settings_view_model,
                    );
                }
            });
        }

        {
            let choreography_settings_view_model = Rc::clone(&choreography_settings_view_model);
            let update_author_sender = update_author_sender.clone();
            let view_weak = view_weak.clone();
            view.on_choreo_update_author(move |value| {
                let mut choreography_settings_view_model =
                    choreography_settings_view_model.borrow_mut();
                choreography_settings_view_model.author = value.to_string();
                let _ = update_author_sender.send(UpdateAuthorCommand {
                    value: choreography_settings_view_model.author.clone(),
                });
                if let Some(view) = view_weak.upgrade() {
                    apply_choreography_settings_view_model(
                        &view,
                        &choreography_settings_view_model,
                    );
                }
            });
        }

        {
            let choreography_settings_view_model = Rc::clone(&choreography_settings_view_model);
            let update_description_sender = update_description_sender.clone();
            let view_weak = view_weak.clone();
            view.on_choreo_update_description(move |value| {
                let mut choreography_settings_view_model =
                    choreography_settings_view_model.borrow_mut();
                choreography_settings_view_model.description = value.to_string();
                let _ = update_description_sender.send(UpdateDescriptionCommand {
                    value: choreography_settings_view_model.description.clone(),
                });
                if let Some(view) = view_weak.upgrade() {
                    apply_choreography_settings_view_model(
                        &view,
                        &choreography_settings_view_model,
                    );
                }
            });
        }

        {
            let choreography_settings_view_model = Rc::clone(&choreography_settings_view_model);
            let update_positions_at_side_sender = update_positions_at_side_sender.clone();
            let view_weak = view_weak.clone();
            view.on_choreo_update_positions_at_side(move |value| {
                let mut choreography_settings_view_model =
                    choreography_settings_view_model.borrow_mut();
                choreography_settings_view_model.positions_at_side = value;
                let _ =
                    update_positions_at_side_sender.send(UpdatePositionsAtSideCommand { value });
                if let Some(view) = view_weak.upgrade() {
                    apply_choreography_settings_view_model(
                        &view,
                        &choreography_settings_view_model,
                    );
                }
            });
        }

        {
            let choreography_settings_view_model = Rc::clone(&choreography_settings_view_model);
            let update_draw_path_from_sender = update_draw_path_from_sender.clone();
            let view_weak = view_weak.clone();
            view.on_choreo_update_draw_path_from(move |value| {
                let choreography_settings_view_model = choreography_settings_view_model.borrow();
                let _ = update_draw_path_from_sender.send(UpdateDrawPathFromCommand { value });
                if let Some(view) = view_weak.upgrade() {
                    apply_choreography_settings_view_model(
                        &view,
                        &choreography_settings_view_model,
                    );
                }
            });
        }

        {
            let choreography_settings_view_model = Rc::clone(&choreography_settings_view_model);
            let update_draw_path_to_sender = update_draw_path_to_sender.clone();
            let view_weak = view_weak.clone();
            view.on_choreo_update_draw_path_to(move |value| {
                let choreography_settings_view_model = choreography_settings_view_model.borrow();
                let _ = update_draw_path_to_sender.send(UpdateDrawPathToCommand { value });
                if let Some(view) = view_weak.upgrade() {
                    apply_choreography_settings_view_model(
                        &view,
                        &choreography_settings_view_model,
                    );
                }
            });
        }

        {
            let choreography_settings_view_model = Rc::clone(&choreography_settings_view_model);
            let update_grid_lines_sender = update_grid_lines_sender.clone();
            let view_weak = view_weak.clone();
            view.on_choreo_update_grid_lines(move |value| {
                let mut choreography_settings_view_model =
                    choreography_settings_view_model.borrow_mut();
                choreography_settings_view_model.grid_lines = value;
                let _ = update_grid_lines_sender.send(UpdateGridLinesCommand { value });
                if let Some(view) = view_weak.upgrade() {
                    apply_choreography_settings_view_model(
                        &view,
                        &choreography_settings_view_model,
                    );
                }
            });
        }

        {
            let choreography_settings_view_model = Rc::clone(&choreography_settings_view_model);
            let update_snap_to_grid_sender = update_snap_to_grid_sender.clone();
            let view_weak = view_weak.clone();
            view.on_choreo_update_snap_to_grid(move |value| {
                let choreography_settings_view_model = choreography_settings_view_model.borrow();
                let _ = update_snap_to_grid_sender.send(UpdateSnapToGridCommand { value });
                if let Some(view) = view_weak.upgrade() {
                    apply_choreography_settings_view_model(
                        &view,
                        &choreography_settings_view_model,
                    );
                }
            });
        }

        {
            let choreography_settings_view_model = Rc::clone(&choreography_settings_view_model);
            let update_show_timestamps_sender = update_show_timestamps_sender.clone();
            let view_weak = view_weak.clone();
            view.on_choreo_update_show_timestamps(move |value| {
                let choreography_settings_view_model = choreography_settings_view_model.borrow();
                let _ = update_show_timestamps_sender.send(UpdateShowTimestampsCommand { value });
                if let Some(view) = view_weak.upgrade() {
                    apply_choreography_settings_view_model(
                        &view,
                        &choreography_settings_view_model,
                    );
                }
            });
        }

        {
            let choreography_settings_view_model = Rc::clone(&choreography_settings_view_model);
            let update_show_legend_sender = update_show_legend_sender.clone();
            let view_weak = view_weak.clone();
            view.on_choreo_update_show_legend(move |value| {
                let choreography_settings_view_model = choreography_settings_view_model.borrow();
                let _ = update_show_legend_sender.send(UpdateShowLegendCommand { value });
                if let Some(view) = view_weak.upgrade() {
                    apply_choreography_settings_view_model(
                        &view,
                        &choreography_settings_view_model,
                    );
                }
            });
        }

        {
            let choreography_settings_view_model = Rc::clone(&choreography_settings_view_model);
            let update_transparency_sender = update_transparency_sender.clone();
            let view_weak = view_weak.clone();
            view.on_choreo_update_transparency(move |value| {
                let mut choreography_settings_view_model =
                    choreography_settings_view_model.borrow_mut();
                choreography_settings_view_model.transparency = value as f64;
                let _ = update_transparency_sender.send(UpdateTransparencyCommand {
                    value: value as f64,
                });
                if let Some(view) = view_weak.upgrade() {
                    apply_choreography_settings_view_model(
                        &view,
                        &choreography_settings_view_model,
                    );
                }
            });
        }

        {
            let choreography_settings_view_model = Rc::clone(&choreography_settings_view_model);
            let update_floor_color_sender = update_floor_color_sender.clone();
            let view_weak = view_weak.clone();
            view.on_choreo_update_floor_color(move |value| {
                let mut choreography_settings_view_model =
                    choreography_settings_view_model.borrow_mut();
                let floor_color = choreo_color_from_slint(value);
                choreography_settings_view_model.floor_color = floor_color.clone();
                let _ =
                    update_floor_color_sender.send(UpdateFloorColorCommand { value: floor_color });
                if let Some(view) = view_weak.upgrade() {
                    apply_choreography_settings_view_model(
                        &view,
                        &choreography_settings_view_model,
                    );
                }
            });
        }

        {
            let choreography_settings_view_model = Rc::clone(&choreography_settings_view_model);
            let update_floor_front_sender = update_floor_front_sender.clone();
            let view_weak = view_weak.clone();
            view.on_choreo_update_floor_front(move |value| {
                let mut choreography_settings_view_model =
                    choreography_settings_view_model.borrow_mut();
                let clamped = value.clamp(1, 100);
                choreography_settings_view_model.floor_front = clamped;
                let _ = update_floor_front_sender.send(UpdateFloorFrontCommand { value: clamped });
                if let Some(view) = view_weak.upgrade() {
                    apply_choreography_settings_view_model(
                        &view,
                        &choreography_settings_view_model,
                    );
                }
            });
        }

        {
            let choreography_settings_view_model = Rc::clone(&choreography_settings_view_model);
            let update_floor_back_sender = update_floor_back_sender.clone();
            let view_weak = view_weak.clone();
            view.on_choreo_update_floor_back(move |value| {
                let mut choreography_settings_view_model =
                    choreography_settings_view_model.borrow_mut();
                let clamped = value.clamp(1, 100);
                choreography_settings_view_model.floor_back = clamped;
                let _ = update_floor_back_sender.send(UpdateFloorBackCommand { value: clamped });
                if let Some(view) = view_weak.upgrade() {
                    apply_choreography_settings_view_model(
                        &view,
                        &choreography_settings_view_model,
                    );
                }
            });
        }

        {
            let choreography_settings_view_model = Rc::clone(&choreography_settings_view_model);
            let update_floor_left_sender = update_floor_left_sender.clone();
            let view_weak = view_weak.clone();
            view.on_choreo_update_floor_left(move |value| {
                let mut choreography_settings_view_model =
                    choreography_settings_view_model.borrow_mut();
                let clamped = value.clamp(1, 100);
                choreography_settings_view_model.floor_left = clamped;
                let _ = update_floor_left_sender.send(UpdateFloorLeftCommand { value: clamped });
                if let Some(view) = view_weak.upgrade() {
                    apply_choreography_settings_view_model(
                        &view,
                        &choreography_settings_view_model,
                    );
                }
            });
        }

        {
            let choreography_settings_view_model = Rc::clone(&choreography_settings_view_model);
            let update_floor_right_sender = update_floor_right_sender.clone();
            let view_weak = view_weak.clone();
            view.on_choreo_update_floor_right(move |value| {
                let mut choreography_settings_view_model =
                    choreography_settings_view_model.borrow_mut();
                let clamped = value.clamp(1, 100);
                choreography_settings_view_model.floor_right = clamped;
                let _ = update_floor_right_sender.send(UpdateFloorRightCommand { value: clamped });
                if let Some(view) = view_weak.upgrade() {
                    apply_choreography_settings_view_model(
                        &view,
                        &choreography_settings_view_model,
                    );
                }
            });
        }

        {
            let choreography_settings_view_model = Rc::clone(&choreography_settings_view_model);
            let update_grid_resolution_sender = update_grid_resolution_sender.clone();
            let view_weak = view_weak.clone();
            view.on_choreo_update_grid_resolution(move |value| {
                let mut choreography_settings_view_model =
                    choreography_settings_view_model.borrow_mut();
                choreography_settings_view_model.set_grid_resolution(value);
                let _ = update_grid_resolution_sender.send(UpdateGridResolutionCommand { value });
                if let Some(view) = view_weak.upgrade() {
                    apply_choreography_settings_view_model(
                        &view,
                        &choreography_settings_view_model,
                    );
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

        {
            let audio_player = Rc::clone(&audio_player);
            let view_weak = view_weak.clone();
            view.on_audio_speed_changed(move |value| {
                let mut span = start_internal_span("audio_player.ui.speed_changed", None);
                let mut audio_player = audio_player.borrow_mut();
                let previous_speed = audio_player.speed;
                audio_player.speed = value as f64;
                audio_player.update_speed_label();
                let speed = audio_player.speed;
                span.set_f64_attribute("choreo.audio.previous_speed", previous_speed);
                span.set_f64_attribute("choreo.audio.new_speed", speed);
                if let Some(player) = audio_player.player.as_mut() {
                    player.set_speed(speed);
                    span.set_bool_attribute("choreo.audio.player_available", true);
                } else {
                    span.set_bool_attribute("choreo.audio.player_available", false);
                }
                if let Some(view) = view_weak.upgrade() {
                    apply_audio_player_view_model(&view, &audio_player);
                }
            });
        }

        {
            let audio_player = Rc::clone(&audio_player);
            let audio_player_view_state = Rc::clone(&audio_player_view_state);
            let view_weak = view_weak.clone();
            view.on_audio_position_changed(move |value| {
                let mut audio_player = audio_player.borrow_mut();
                audio_player.position = value as f64;
                audio_player.update_duration_label();
                if !audio_player_view_state.borrow().is_user_dragging() {
                    let position = audio_player.position;
                    audio_player.seek(position);
                    audio_player.update_duration_label();
                }
                if let Some(view) = view_weak.upgrade() {
                    apply_audio_player_view_model(&view, &audio_player);
                }
            });
        }

        let slider_drag_start_position = Rc::new(RefCell::new(None::<f64>));
        let slider_drag_start_time = Rc::new(RefCell::new(None::<Instant>));

        {
            let audio_player = Rc::clone(&audio_player);
            let audio_player_view_state = Rc::clone(&audio_player_view_state);
            let view_weak = view_weak.clone();
            let slider_drag_start_position = Rc::clone(&slider_drag_start_position);
            let slider_drag_start_time = Rc::clone(&slider_drag_start_time);
            view.on_audio_position_drag_started(move || {
                let mut pointer_down_span =
                    start_internal_span("audio_player.ui.slider_pointer_down", None);
                let mut span = start_internal_span("audio_player.ui.position_drag_started", None);
                let mut audio_player = audio_player.borrow_mut();
                *slider_drag_start_position.borrow_mut() = Some(audio_player.position);
                *slider_drag_start_time.borrow_mut() = Some(Instant::now());
                span.set_f64_attribute("choreo.audio.drag_start_position", audio_player.position);
                span.set_bool_attribute("choreo.audio.was_playing", audio_player.is_playing);
                pointer_down_span
                    .set_f64_attribute("choreo.audio.pointer_down_position", audio_player.position);
                pointer_down_span
                    .set_bool_attribute("choreo.audio.was_playing", audio_player.is_playing);
                let snapshot = {
                    let mut state = audio_player_view_state.borrow_mut();
                    state.on_position_drag_started(&mut audio_player);
                    state.snapshot()
                };
                span.set_bool_attribute("choreo.audio.is_user_dragging", snapshot.is_user_dragging);
                if let Some(pending_seek) = snapshot.pending_seek_position {
                    span.set_f64_attribute(
                        "choreo.audio.pending_seek_position_seconds",
                        pending_seek,
                    );
                }
                if let Some(view) = view_weak.upgrade() {
                    apply_audio_player_view_model(&view, &audio_player);
                }
            });
        }

        {
            let audio_player = Rc::clone(&audio_player);
            let audio_player_view_state = Rc::clone(&audio_player_view_state);
            let view_weak = view_weak.clone();
            let slider_drag_start_position = Rc::clone(&slider_drag_start_position);
            let slider_drag_start_time = Rc::clone(&slider_drag_start_time);
            view.on_audio_position_drag_completed(move |value| {
                let mut pointer_up_span =
                    start_internal_span("audio_player.ui.slider_pointer_up", None);
                let mut span = start_internal_span("audio_player.ui.position_drag_completed", None);
                let mut audio_player = audio_player.borrow_mut();
                let previous_position = audio_player.position;
                let position = value as f64;
                let drag_start_position = slider_drag_start_position
                    .borrow()
                    .unwrap_or(previous_position);
                let drag_elapsed_seconds = slider_drag_start_time
                    .borrow()
                    .map(|started_at| started_at.elapsed().as_secs_f64())
                    .unwrap_or(0.0);
                audio_player.position = position;
                audio_player.update_duration_label();
                span.set_f64_attribute("choreo.audio.drag_start_position", previous_position);
                span.set_f64_attribute("choreo.audio.drag_target_position", position);
                span.set_bool_attribute("choreo.audio.can_seek", audio_player.can_seek);
                pointer_up_span.set_f64_attribute("choreo.audio.pointer_up_position", position);
                pointer_up_span
                    .set_f64_attribute("choreo.audio.drag_start_position", drag_start_position);
                pointer_up_span
                    .set_f64_attribute("choreo.audio.drag_elapsed_seconds", drag_elapsed_seconds);
                let snapshot = {
                    let mut state = audio_player_view_state.borrow_mut();
                    state.on_position_drag_completed(&mut audio_player, position);
                    state.snapshot()
                };
                span.set_bool_attribute("choreo.audio.is_user_dragging", snapshot.is_user_dragging);
                if let Some(pending_seek) = snapshot.pending_seek_position {
                    span.set_f64_attribute(
                        "choreo.audio.pending_seek_position_seconds",
                        pending_seek,
                    );
                }
                if let Some(pending_seek_age) = snapshot.pending_seek_age_seconds {
                    span.set_f64_attribute(
                        "choreo.audio.pending_seek_age_seconds",
                        pending_seek_age,
                    );
                }
                let is_click = (position - drag_start_position).abs() <= 0.01;
                if is_click {
                    let mut click_span = start_internal_span("audio_player.ui.slider_click", None);
                    click_span.set_f64_attribute("choreo.audio.click_target_position", position);
                    click_span.set_f64_attribute(
                        "choreo.audio.click_elapsed_seconds",
                        drag_elapsed_seconds,
                    );
                    click_span.set_bool_attribute("choreo.audio.can_seek", audio_player.can_seek);
                }
                *slider_drag_start_position.borrow_mut() = None;
                *slider_drag_start_time.borrow_mut() = None;
                if let Some(view) = view_weak.upgrade() {
                    apply_audio_player_view_model(&view, &audio_player);
                }
            });
        }

        {
            let audio_player = Rc::clone(&audio_player);
            let global_state_store = Rc::clone(&deps.global_state_store);
            let view_weak = view_weak.clone();
            view.on_audio_link_scene_to_position(move || {
                let mut span = start_internal_span("audio_player.ui.link_scene_to_position", None);
                let mut audio_player = audio_player.borrow_mut();
                span.set_f64_attribute("choreo.audio.position_seconds", audio_player.position);
                if let Some(haptic) = &audio_player.haptic_feedback
                    && haptic.is_supported()
                {
                    haptic.perform_click();
                }
                let linked_result = link_selected_scene_to_audio_position(
                    &global_state_store,
                    audio_player.position,
                    &mut audio_player,
                );
                if let Some((scene_id, linked_timestamp)) = linked_result {
                    span.set_bool_attribute("choreo.success", true);
                    span.set_string_attribute("choreo.scene.id", scene_id);
                    span.set_f64_attribute("choreo.scene.timestamp_seconds", linked_timestamp);
                } else {
                    span.set_bool_attribute("choreo.success", false);
                }
                if let Some(view) = view_weak.upgrade() {
                    apply_audio_player_view_model(&view, &audio_player);
                }
            });
        }

        {
            let audio_player = Rc::clone(&audio_player);
            let view_weak = view_weak.clone();
            view.on_audio_toggle_play_pause(move || {
                let mut span = start_internal_span("audio_player.ui.toggle_play_pause", None);
                let mut audio_player = audio_player.borrow_mut();
                span.set_bool_attribute("choreo.audio.was_playing", audio_player.is_playing);
                span.set_f64_attribute("choreo.audio.position_seconds", audio_player.position);
                audio_player.toggle_play_pause();
                span.set_bool_attribute("choreo.audio.is_playing", audio_player.is_playing);
                if let Some(view) = view_weak.upgrade() {
                    apply_audio_player_view_model(&view, &audio_player);
                }
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
            audio_player,
            nav_bar_timer,
            settings_timer,
            audio_player_timer,
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
            let _ = self.open_image_request_sender.try_send(OpenImageRequested {
                file_path: file_path.to_string(),
            });
            return;
        }

        if extension == "mp3" {
            let _ = self.open_audio_request_sender.try_send(OpenAudioRequested {
                file_path: file_path.to_string(),
                trace_context: None,
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

fn apply_settings_view_model(view: &ShellHost, view_model: &SettingsViewModel) {
    view.set_settings_use_system_theme(view_model.use_system_theme);
    view.set_settings_is_dark_mode(matches!(view_model.theme_mode, ThemeMode::Dark));
    view.set_settings_use_primary_color(view_model.use_primary_color);
    view.set_settings_use_secondary_color(view_model.use_secondary_color);
    view.set_settings_use_tertiary_color(view_model.use_tertiary_color);
    view.set_settings_primary_color(slint_color_from_choreo(&view_model.primary_color));
    view.set_settings_secondary_color(slint_color_from_choreo(&view_model.secondary_color));
    view.set_settings_tertiary_color(slint_color_from_choreo(&view_model.tertiary_color));
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
    view.set_choreo_grid_size_options(ModelRc::new(VecModel::from(build_grid_menu_items(
        &view_model.grid_size_options,
    ))));
}

fn apply_audio_player_view_model(view: &ShellHost, view_model: &AudioPlayerViewModel) {
    let max_tick = view_model
        .tick_values
        .iter()
        .copied()
        .fold(0.0_f64, f64::max);
    let slider_duration = view_model
        .duration
        .max(max_tick)
        .max(view_model.position)
        .max(1.0);

    view.set_audio_speed(view_model.speed as f32);
    view.set_audio_minimum_speed(view_model.minimum_speed as f32);
    view.set_audio_maximum_speed(view_model.maximum_speed as f32);
    view.set_audio_position(view_model.position as f32);
    view.set_audio_duration(slider_duration as f32);
    view.set_audio_tick_values(ModelRc::new(VecModel::from(
        view_model
            .tick_values
            .iter()
            .map(|value| *value as f32)
            .collect::<Vec<_>>(),
    )));
    view.set_audio_can_seek(view_model.can_seek);
    view.set_audio_can_link_scene_to_position(view_model.can_link_scene_to_position);
    view.set_audio_is_playing(view_model.is_playing);
    view.set_audio_speed_label(view_model.speed_label.as_str().into());
    view.set_audio_duration_label(view_model.duration_label.as_str().into());
}

fn link_selected_scene_to_audio_position(
    global_state_store: &GlobalStateActor,
    audio_position: f64,
    audio_player: &mut AudioPlayerViewModel,
) -> Option<(String, f64)> {
    let mut linked_scene_id = None;
    let mut linked_timestamp = None;
    let updated = global_state_store.try_update(|global_state| {
        let Some(selected_scene) = global_state.selected_scene.as_mut() else {
            return;
        };
        let selected_scene_id = selected_scene.scene_id;
        let rounded_timestamp = round_to_100_millis(audio_position);
        linked_scene_id = Some(format!("{selected_scene_id:?}"));
        linked_timestamp = Some(rounded_timestamp);
        selected_scene.timestamp = Some(rounded_timestamp);
        if let Some(scene) = global_state
            .scenes
            .iter_mut()
            .find(|scene| scene.scene_id == selected_scene_id)
        {
            scene.timestamp = Some(rounded_timestamp);
        }
        if let Some(model_scene) = global_state
            .choreography
            .scenes
            .iter_mut()
            .find(|scene| scene.scene_id == selected_scene_id)
        {
            model_scene.timestamp = Some(format_seconds(rounded_timestamp));
        }
    });
    if !updated {
        return None;
    }

    if let Some((scenes, selected_scene)) = global_state_store.try_with_state(|global_state| {
        (
            global_state.scenes.clone(),
            global_state.selected_scene.clone(),
        )
    }) {
        audio_player.tick_values = build_tick_values(audio_player.duration, &scenes);
        audio_player.can_link_scene_to_position = selected_scene.is_some();
    }

    linked_scene_id.zip(linked_timestamp)
}

fn round_to_100_millis(seconds: f64) -> f64 {
    let milliseconds = seconds * 1000.0;
    let rounded = (milliseconds / 100.0).round() * 100.0;
    rounded / 1000.0
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
            color: slint::Color::from_argb_u8(
                scene.color.a,
                scene.color.r,
                scene.color.g,
                scene.color.b,
            ),
            is_selected: scene.is_selected,
        })
        .collect::<Vec<_>>();

    ModelRc::new(VecModel::from(items))
}

fn build_grid_menu_items(
    options: &[crate::choreography_settings::GridSizeOption],
) -> Vec<MenuItem> {
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
