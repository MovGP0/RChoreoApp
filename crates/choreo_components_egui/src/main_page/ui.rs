use egui::Frame;
use egui::Layout;
use egui::Rect;
use egui::Ui;
use egui::pos2;
use egui::vec2;

use crate::audio_player;
use crate::audio_player::actions::AudioPlayerAction;
use crate::choreo_main::actions::ChoreoMainAction;
use crate::choreo_main::actions::OpenChoreoRequested;
use crate::choreo_main::state::ChoreoMainState;
use crate::choreo_main::state::InteractionMode;
use crate::choreography_settings;
use crate::choreography_settings::actions::ChoreographySettingsAction;
use crate::drawer_host::actions::DrawerHostAction;
use crate::drawer_host::state::DrawerHostOpenMode;
use crate::drawer_host::state::DrawerHostState;
use crate::drawer_host::ui::draw_with_slots_in_rect;
use crate::floor;
use crate::floor::actions::FloorAction;
use crate::hamburger_toggle_button;
use crate::material::components;
use crate::nav_bar::translations::mode_text;
use crate::nav_bar::translations::nav_bar_translations;
use crate::scenes;
use crate::scenes::actions::ScenesAction;
use crate::scenes::state::SceneItemState;
use crate::scenes::state::ScenesState;
use crate::scenes::state::format_seconds;
use crate::scenes::state::parse_timestamp_seconds;
use crate::ui_icons;
use crate::ui_icons::UiIconKey;
use crate::ui_style::typography::TypographyRole;
use choreo_master_mobile_json::Color;
use choreo_master_mobile_json::SceneId;

const TOP_BAR_HEIGHT_PX: f32 = 84.0;
const DRAWER_WIDTH_LEFT_PX: f32 = 324.0;
const DRAWER_WIDTH_RIGHT_PX: f32 = 480.0;
const AUDIO_PANEL_HEIGHT_PX: f32 = 84.0;
const GRID_12_PX: f32 = 12.0;
const DEFAULT_LOCALE: &str = "en";
const MODE_SELECTOR_WIDTH_PX: f32 = 180.0;
const MODE_SELECTOR_HEIGHT_PX: f32 = 56.0;

pub fn draw(ui: &mut Ui, state: &ChoreoMainState) -> Vec<ChoreoMainAction> {
    let mut actions: Vec<ChoreoMainAction> = Vec::new();
    ui.spacing_mut().item_spacing = vec2(GRID_12_PX, GRID_12_PX);
    let page_rect = shell_rect(ui);
    let audio_panel_height = audio_panel_height_px(state.is_audio_player_open);
    let top_bar_rect = top_bar_rect(page_rect);
    let drawer_host_rect = drawer_host_rect(page_rect, audio_panel_height);
    let host_bottom = drawer_host_rect.max.y;
    egui::Area::new(egui::Id::new("main_page_top_bar"))
        .order(egui::Order::Foreground)
        .fixed_pos(top_bar_rect.min)
        .show(ui.ctx(), |ui| {
            ui.painter().rect_filled(
                Rect::from_min_size(egui::Pos2::ZERO, top_bar_rect.size()),
                0.0,
                ui.visuals().panel_fill,
            );
            ui.set_width(top_bar_rect.width());
            ui.set_min_height(top_bar_rect.height());
            draw_top_bar(ui, state, &mut actions);
        });

    let drawer_state = drawer_host_state(drawer_host_rect.size(), state);
    let slot_actions = std::cell::RefCell::new(Vec::new());
    let drawer_host_actions = draw_with_slots_in_rect(
        ui.ctx(),
        drawer_host_rect,
        "main_page",
        &drawer_state,
        |ui| {
            slot_actions
                .borrow_mut()
                .extend(draw_floor_host_content(ui, &state.floor_state));
        },
        |ui| {
            let mut slot_actions = slot_actions.borrow_mut();
            draw_scenes_drawer(ui, state, &mut slot_actions);
        },
        |ui| {
            slot_actions
                .borrow_mut()
                .extend(draw_settings_drawer(ui, state));
        },
        |_| {},
        |_| {},
    );

    actions.extend(slot_actions.into_inner());
    for action in drawer_host_actions {
        actions.extend(map_drawer_host_action(action, state));
    }

    if state.is_audio_player_open {
        let audio_rect = Rect::from_min_max(pos2(page_rect.min.x, host_bottom), page_rect.max);
        egui::Area::new(egui::Id::new("main_page_audio_host"))
            .order(egui::Order::Foreground)
            .fixed_pos(audio_rect.min)
            .show(ui.ctx(), |ui| {
                ui.set_width(audio_rect.width());
                ui.set_min_height(audio_rect.height());
                ui.painter().rect_filled(
                    Rect::from_min_size(egui::Pos2::ZERO, audio_rect.size()),
                    0.0,
                    ui.visuals().panel_fill,
                );
                actions.extend(draw_audio_host(ui, state));
            });
    }

    actions
}

fn draw_top_bar(ui: &mut Ui, state: &ChoreoMainState, actions: &mut Vec<ChoreoMainAction>) {
    let strings = nav_bar_translations(DEFAULT_LOCALE);
    ui.spacing_mut().item_spacing = vec2(GRID_12_PX, GRID_12_PX);
    ui.with_layout(Layout::left_to_right(egui::Align::Center), |ui| {
        ui.set_min_height(TOP_BAR_HEIGHT_PX);
        ui.add_space(8.0);
        let nav_response = hamburger_toggle_button::draw(
            ui,
            state.is_nav_open,
            true,
            strings.toggle_navigation_tooltip.as_str(),
            Some(vec2(48.0, 48.0)),
        );
        if nav_response.clicked() {
            actions.push(top_bar_nav_action(state.is_nav_open));
        }
        ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
            ui.add_space(8.0);

            let open_audio_response = components::icon_button(
                ui,
                components::icon_image(components::TopBarIcon::Audio),
                false,
            );
            if open_audio_response.clicked() {
                actions.push(ChoreoMainAction::OpenAudioPanel);
            }
            let _ = open_audio_response.on_hover_text(strings.open_audio_tooltip.as_str());

            let open_image_response = components::icon_button(
                ui,
                components::icon_image(components::TopBarIcon::Image),
                state.floor_state.svg_path.is_some(),
            );
            if open_image_response.clicked() {
                actions.push(ChoreoMainAction::RequestOpenImage {
                    file_path: String::new(),
                });
            }
            let _ = open_image_response.on_hover_text(strings.open_image_tooltip.as_str());

            let home_response = components::icon_button(
                ui,
                components::icon_image(components::TopBarIcon::Home),
                false,
            );
            if home_response.clicked() {
                actions.push(ChoreoMainAction::ResetFloorViewport);
            }
            let _ = home_response.on_hover_text(strings.reset_floor_viewport_tooltip.as_str());

            let settings_response = components::icon_button(
                ui,
                components::icon_image(components::TopBarIcon::Settings),
                state.is_choreography_settings_open,
            );
            if settings_response.clicked() {
                actions.push(top_bar_settings_action(state.is_choreography_settings_open));
            }
            let _ = settings_response.on_hover_text(strings.open_settings_tooltip.as_str());

            let previous_mode_index =
                effective_mode_index(state).clamp(0, mode_count() - 1) as usize;
            let selected_mode_index = components::mode_dropdown(
                ui,
                egui::Id::new("main_page_mode_dropdown"),
                Some(previous_mode_index),
                &translated_mode_labels(&strings),
                state.is_mode_selection_enabled,
                MODE_SELECTOR_WIDTH_PX,
                MODE_SELECTOR_HEIGHT_PX,
            );
            if let Some(selected_mode_index) = selected_mode_index
                && selected_mode_index != previous_mode_index
            {
                actions.push(ChoreoMainAction::SelectMode {
                    index: selected_mode_index as i32,
                });
            }
        });
    });
}

fn draw_scenes_drawer(ui: &mut Ui, state: &ChoreoMainState, actions: &mut Vec<ChoreoMainAction>) {
    let pane_state = scene_pane_state(state);
    ui.allocate_ui_with_layout(
        egui::vec2(DRAWER_WIDTH_LEFT_PX, ui.available_height()),
        egui::Layout::top_down(egui::Align::LEFT),
        |ui| {
            ui.set_min_width(DRAWER_WIDTH_LEFT_PX);
            ui.set_min_height(ui.available_height());
            for action in scenes::ui::draw(ui, &pane_state) {
                if let Some(mapped_action) = map_scene_pane_action(action) {
                    actions.push(mapped_action);
                }
            }
        },
    );
}

fn draw_floor_host_content(ui: &mut Ui, state: &floor::state::FloorState) -> Vec<ChoreoMainAction> {
    let mut actions: Vec<ChoreoMainAction> = Vec::new();
    Frame::group(ui.style()).show(ui, |ui| {
        ui.set_min_size(vec2(ui.available_width(), ui.available_height().max(360.0)));
        let floor_actions = floor::ui::draw(ui, state);
        actions.extend(floor_actions.into_iter().map(map_floor_host_action));
    });
    actions
}

fn draw_settings_drawer(ui: &mut Ui, state: &ChoreoMainState) -> Vec<ChoreoMainAction> {
    let mut actions = Vec::new();
    ui.allocate_ui_with_layout(
        egui::vec2(DRAWER_WIDTH_RIGHT_PX, ui.available_height()),
        egui::Layout::top_down(egui::Align::LEFT),
        |ui| {
            Frame::group(ui.style()).show(ui, |ui| {
                ui.set_min_width(DRAWER_WIDTH_RIGHT_PX);
                ui.set_min_height(ui.available_height());
                let settings_actions =
                    choreography_settings::ui::draw(ui, &state.choreography_settings_state);
                actions.extend(
                    settings_actions
                        .into_iter()
                        .map(map_choreography_settings_action),
                );
            });
        },
    );
    actions
}

fn draw_audio_host(ui: &mut Ui, state: &ChoreoMainState) -> Vec<ChoreoMainAction> {
    let mut actions: Vec<ChoreoMainAction> = Vec::new();
    Frame::group(ui.style()).show(ui, |ui| {
        ui.set_min_height(AUDIO_PANEL_HEIGHT_PX);
        for action in audio_player::ui::draw(ui, &state.audio_player_state) {
            actions.extend(map_audio_host_action(action));
        }
    });
    actions
}

#[must_use]
pub fn drawer_host_state(_viewport_size: egui::Vec2, state: &ChoreoMainState) -> DrawerHostState {
    DrawerHostState {
        left_drawer_width: DRAWER_WIDTH_LEFT_PX,
        right_drawer_width: DRAWER_WIDTH_RIGHT_PX,
        responsive_breakpoint: 900.0,
        open_mode: DrawerHostOpenMode::Standard,
        top_inset: TOP_BAR_HEIGHT_PX,
        inline_left: false,
        is_left_open: state.is_nav_open,
        is_right_open: state.is_choreography_settings_open,
        ..DrawerHostState::default()
    }
}

#[must_use]
pub fn top_bar_rect(page_rect: Rect) -> Rect {
    Rect::from_min_size(page_rect.min, vec2(page_rect.width(), TOP_BAR_HEIGHT_PX))
}

#[must_use]
pub fn drawer_host_rect(page_rect: Rect, audio_panel_height: f32) -> Rect {
    let host_top = page_rect.min.y;
    let host_bottom = (page_rect.max.y - audio_panel_height).max(host_top);
    Rect::from_min_max(
        pos2(page_rect.min.x, host_top),
        pos2(page_rect.max.x, host_bottom),
    )
}

#[must_use]
pub fn shell_rect(ui: &Ui) -> Rect {
    ui.max_rect()
}

#[must_use]
pub fn map_drawer_host_action(
    action: DrawerHostAction,
    state: &ChoreoMainState,
) -> Vec<ChoreoMainAction> {
    match action {
        DrawerHostAction::OverlayClicked {
            close_left,
            close_right,
            close_top: _,
            close_bottom: _,
        } => {
            let mut actions = Vec::new();
            if close_left && state.is_nav_open {
                actions.push(ChoreoMainAction::CloseNav);
            }
            if close_right && state.is_choreography_settings_open {
                actions.push(ChoreoMainAction::CloseSettings);
            }
            actions
        }
    }
}

#[must_use]
pub fn audio_panel_height_px(is_audio_player_open: bool) -> f32 {
    if is_audio_player_open {
        AUDIO_PANEL_HEIGHT_PX
    } else {
        0.0
    }
}

#[must_use]
pub fn map_floor_host_action(action: FloorAction) -> ChoreoMainAction {
    ChoreoMainAction::FloorAction(action)
}

#[must_use]
pub fn map_audio_host_action(action: AudioPlayerAction) -> Vec<ChoreoMainAction> {
    let mut mapped = vec![ChoreoMainAction::AudioPlayerAction(action.clone())];
    match action {
        AudioPlayerAction::SeekToPosition { position }
        | AudioPlayerAction::PositionDragCompleted { position } => {
            mapped.push(ChoreoMainAction::UpdateAudioPosition { seconds: position });
        }
        AudioPlayerAction::LinkSceneToPosition => {
            mapped.push(ChoreoMainAction::LinkSelectedSceneToAudioPosition);
        }
        _ => {}
    }
    mapped
}

#[must_use]
pub fn map_choreography_settings_action(action: ChoreographySettingsAction) -> ChoreoMainAction {
    ChoreoMainAction::ChoreographySettingsAction(action)
}

#[must_use]
pub fn scene_pane_state(state: &ChoreoMainState) -> ScenesState {
    let scenes = if state
        .choreography_settings_state
        .choreography
        .scenes
        .is_empty()
    {
        state
            .scenes
            .iter()
            .enumerate()
            .map(|(index, scene)| SceneItemState {
                scene_id: SceneId(index as i32 + 1),
                name: scene.name.clone(),
                text: String::new(),
                fixed_positions: false,
                timestamp: scene.timestamp_seconds,
                is_selected: state.selected_scene_index == Some(index),
                positions: Vec::new(),
                variation_depth: 0,
                variations: Vec::new(),
                current_variation: Vec::new(),
                color: Color::transparent(),
            })
            .collect::<Vec<_>>()
    } else {
        state
            .choreography_settings_state
            .choreography
            .scenes
            .iter()
            .enumerate()
            .map(|(index, scene)| project_scene_list_item(index, scene, state))
            .collect::<Vec<_>>()
    };
    let visible_scenes = filter_scene_items(&scenes, &state.scene_search_text);
    let selected_scene = state
        .selected_scene_index
        .and_then(|index| scenes.get(index).cloned());
    let mut pane_state = ScenesState {
        choreography: choreo_models::ChoreographyModel::default(),
        scenes,
        visible_scenes,
        selected_scene: selected_scene.clone(),
        search_text: state.scene_search_text.clone(),
        show_timestamps: state.choreography_settings_state.show_timestamps
            || state
                .choreography_settings_state
                .choreography
                .settings
                .show_timestamps,
        is_place_mode: state.interaction_mode != InteractionMode::View,
        can_save_choreo: can_save_choreo(state),
        can_delete_scene: selected_scene.is_some(),
        can_navigate_to_settings: true,
        can_navigate_to_dancer_settings: true,
        has_selected_scene: selected_scene.is_some(),
        ..ScenesState::default()
    };
    if let Some(selected_scene) = selected_scene {
        pane_state.selected_scene_name = selected_scene.name.clone();
        pane_state.selected_scene_text = selected_scene.text.clone();
        pane_state.selected_scene_fixed_positions = selected_scene.fixed_positions;
        pane_state.selected_scene_timestamp_text = selected_scene
            .timestamp
            .map(format_seconds)
            .unwrap_or_default();
        pane_state.selected_scene_color = selected_scene.color;
    }
    pane_state
}

#[must_use]
pub fn map_scene_pane_action(action: ScenesAction) -> Option<ChoreoMainAction> {
    match action {
        ScenesAction::RequestOpenChoreography => {
            Some(ChoreoMainAction::RequestOpenChoreo(OpenChoreoRequested {
                file_path: None,
                file_name: None,
                contents: String::new(),
            }))
        }
        ScenesAction::RequestSaveChoreography => Some(ChoreoMainAction::RequestSaveChoreo),
        ScenesAction::NavigateToSettings => Some(ChoreoMainAction::NavigateToSettings),
        ScenesAction::NavigateToDancerSettings => Some(ChoreoMainAction::NavigateToDancers),
        ScenesAction::UpdateSearchText(value) => {
            Some(ChoreoMainAction::UpdateSceneSearchText(value))
        }
        ScenesAction::InsertScene { insert_after } => {
            Some(ChoreoMainAction::InsertScene { insert_after })
        }
        ScenesAction::OpenDeleteSceneDialog => Some(ChoreoMainAction::DeleteSelectedScene),
        ScenesAction::SelectScene { index } => Some(ChoreoMainAction::SelectScene { index }),
        ScenesAction::LoadScenes { .. }
        | ScenesAction::ReloadScenes
        | ScenesAction::SelectSceneFromAudioPosition { .. }
        | ScenesAction::ApplyPlacementModeForSelected
        | ScenesAction::SyncShowTimestampsFromChoreography
        | ScenesAction::UpdateShowTimestamps(_)
        | ScenesAction::CancelDeleteSceneDialog
        | ScenesAction::ConfirmDeleteSceneDialog
        | ScenesAction::OpenCopyScenePositionsDialog
        | ScenesAction::CancelCopyScenePositionsDialog
        | ScenesAction::ConfirmCopyScenePositionsDialog { .. }
        | ScenesAction::OpenChoreography { .. }
        | ScenesAction::SaveChoreography
        | ScenesAction::ClearEphemeralOutputs => None,
    }
}

fn can_save_choreo(state: &ChoreoMainState) -> bool {
    let has_file = state
        .last_opened_choreo_file
        .as_ref()
        .is_some_and(|path| !path.trim().is_empty() && std::path::Path::new(path).exists());
    let choreography = &state.choreography_settings_state.choreography;
    let has_choreo = !choreography.name.is_empty() || !choreography.scenes.is_empty();
    has_file && has_choreo
}

fn filter_scene_items(scenes: &[SceneItemState], search_text: &str) -> Vec<SceneItemState> {
    if search_text.trim().is_empty() {
        return scenes.to_vec();
    }

    let search_text = search_text.to_ascii_lowercase();
    scenes
        .iter()
        .filter(|scene| scene.name.to_ascii_lowercase().contains(&search_text))
        .cloned()
        .collect()
}

fn project_scene_list_item(
    index: usize,
    scene: &choreo_models::SceneModel,
    state: &ChoreoMainState,
) -> SceneItemState {
    SceneItemState {
        scene_id: scene.scene_id,
        name: scene.name.clone(),
        text: scene.text.clone().unwrap_or_default(),
        fixed_positions: scene.fixed_positions,
        timestamp: scene.timestamp.as_deref().and_then(parse_timestamp_seconds),
        is_selected: state.selected_scene_index == Some(index),
        positions: Vec::new(),
        variation_depth: 0,
        variations: Vec::new(),
        current_variation: Vec::new(),
        color: scene.color.clone(),
    }
}

#[must_use]
pub fn top_bar_nav_action(is_nav_open: bool) -> ChoreoMainAction {
    if is_nav_open {
        ChoreoMainAction::CloseNav
    } else {
        ChoreoMainAction::ToggleNav
    }
}

#[must_use]
pub fn top_bar_settings_action(is_settings_open: bool) -> ChoreoMainAction {
    if is_settings_open {
        ChoreoMainAction::CloseSettings
    } else {
        ChoreoMainAction::OpenSettings
    }
}

#[must_use]
pub fn nav_icon_name(is_nav_open: bool) -> &'static str {
    nav_icon_spec(is_nav_open).token
}

#[must_use]
pub fn top_bar_settings_icon_name() -> &'static str {
    top_bar_settings_icon_spec().token
}

#[must_use]
pub fn home_icon_name() -> &'static str {
    home_icon_spec().token
}

#[must_use]
pub fn open_image_icon_name() -> &'static str {
    open_image_icon_spec().token
}

#[must_use]
pub fn open_audio_icon_name() -> &'static str {
    open_audio_icon_spec().token
}

#[must_use]
pub fn nav_icon_svg(is_nav_open: bool) -> &'static str {
    if is_nav_open {
        include_str!("../../assets/icons/Close.svg")
    } else {
        include_str!("../../assets/icons/Menu.svg")
    }
}

#[must_use]
pub fn top_bar_settings_icon_svg() -> &'static str {
    include_str!("../../assets/icons/Pen.svg")
}

#[must_use]
pub fn home_icon_svg() -> &'static str {
    include_str!("../../assets/icons/Home.svg")
}

#[must_use]
pub fn open_image_icon_svg() -> &'static str {
    include_str!("../../assets/icons/Svg.svg")
}

#[must_use]
pub fn open_audio_icon_svg() -> &'static str {
    include_str!("../../assets/icons/PlayCircle.svg")
}

fn nav_icon_spec(is_nav_open: bool) -> ui_icons::UiIconSpec {
    if is_nav_open {
        ui_icons::icon(UiIconKey::NavClose)
    } else {
        ui_icons::icon(UiIconKey::NavOpen)
    }
}

fn top_bar_settings_icon_spec() -> ui_icons::UiIconSpec {
    ui_icons::icon(UiIconKey::NavSettings)
}

fn home_icon_spec() -> ui_icons::UiIconSpec {
    ui_icons::icon(UiIconKey::FloorResetViewport)
}

fn open_image_icon_spec() -> ui_icons::UiIconSpec {
    ui_icons::icon(UiIconKey::FloorOpenSvgOverlay)
}

fn open_audio_icon_spec() -> ui_icons::UiIconSpec {
    ui_icons::icon(UiIconKey::AudioOpenPanel)
}

#[must_use]
pub fn mode_label(mode_index: i32) -> &'static str {
    match mode_index {
        0 => "View",
        1 => "Move",
        2 => "Rotate around center",
        3 => "Rotate around dancer",
        4 => "Scale",
        5 => "Line of sight",
        _ => "Mode",
    }
}

#[must_use]
pub fn mode_count() -> i32 {
    6
}

#[must_use]
pub const fn mode_label_role() -> TypographyRole {
    TypographyRole::LabelLarge
}

#[must_use]
pub fn top_bar_action_count() -> usize {
    6
}

#[must_use]
pub fn top_bar_action_icon_tokens(is_nav_open: bool) -> [&'static str; 5] {
    [
        nav_icon_name(is_nav_open),
        top_bar_settings_icon_name(),
        home_icon_name(),
        open_image_icon_name(),
        open_audio_icon_name(),
    ]
}

#[must_use]
pub const fn top_bar_action_icon_uris() -> [&'static str; 4] {
    [
        components::icon_uri(components::TopBarIcon::Settings),
        components::icon_uri(components::TopBarIcon::Home),
        components::icon_uri(components::TopBarIcon::Image),
        components::icon_uri(components::TopBarIcon::Audio),
    ]
}

#[must_use]
pub fn translated_mode_labels(
    strings: &crate::nav_bar::translations::NavBarTranslations,
) -> [&str; 6] {
    [
        mode_text(strings, crate::nav_bar::state::InteractionMode::View),
        mode_text(strings, crate::nav_bar::state::InteractionMode::Move),
        mode_text(
            strings,
            crate::nav_bar::state::InteractionMode::RotateAroundCenter,
        ),
        mode_text(
            strings,
            crate::nav_bar::state::InteractionMode::RotateAroundDancer,
        ),
        mode_text(strings, crate::nav_bar::state::InteractionMode::Scale),
        mode_text(strings, crate::nav_bar::state::InteractionMode::LineOfSight),
    ]
}

fn effective_mode_index(state: &ChoreoMainState) -> i32 {
    if state.selected_mode_index >= 0 {
        return state.selected_mode_index;
    }

    match state.interaction_mode {
        InteractionMode::View => 0,
        InteractionMode::Move => 1,
        InteractionMode::RotateAroundCenter => 2,
        InteractionMode::RotateAroundDancer => 3,
        InteractionMode::Scale => 4,
        InteractionMode::LineOfSight => 5,
    }
}
