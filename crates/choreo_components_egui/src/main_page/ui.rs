use egui::Frame;
use egui::Rect;
use egui::Ui;
use egui::UiBuilder;
use egui::pos2;
use egui::vec2;
use egui_material3::MaterialIconButton;
use egui_material3::MaterialSelect;

use crate::audio_player;
use crate::audio_player::actions::AudioPlayerAction;
use crate::choreo_main::actions::ChoreoMainAction;
use crate::choreo_main::state::ChoreoMainState;
use crate::choreo_main::state::InteractionMode;
use crate::choreography_settings;
use crate::choreography_settings::actions::ChoreographySettingsAction;
use crate::floor;
use crate::floor::actions::FloorAction;
use crate::main_page_drawer_host::actions::MainPageDrawerHostAction;
use crate::main_page_drawer_host::state::MainPageDrawerHostState;
use crate::main_page_drawer_host::ui::draw_with_slots;
use crate::ui_icons;
use crate::ui_icons::UiIconKey;
use crate::ui_style::typography::TypographyRole;

const TOP_BAR_HEIGHT_PX: f32 = 84.0;
const DRAWER_WIDTH_LEFT_PX: f32 = 324.0;
const DRAWER_WIDTH_RIGHT_PX: f32 = 480.0;
const AUDIO_PANEL_HEIGHT_PX: f32 = 84.0;
const GRID_12_PX: f32 = 12.0;

pub fn draw(ui: &mut Ui, state: &ChoreoMainState) -> Vec<ChoreoMainAction> {
    let mut actions: Vec<ChoreoMainAction> = Vec::new();
    ui.spacing_mut().item_spacing = vec2(GRID_12_PX, GRID_12_PX);
    let page_rect = ui.max_rect();
    let audio_panel_height = audio_panel_height_px(state.is_audio_player_open);
    let host_bottom = (page_rect.max.y - audio_panel_height).max(page_rect.min.y);

    let top_bar_rect =
        Rect::from_min_size(page_rect.min, vec2(page_rect.width(), TOP_BAR_HEIGHT_PX));
    ui.scope_builder(UiBuilder::new().max_rect(top_bar_rect), |ui| {
        ui.painter()
            .rect_filled(ui.max_rect(), 0.0, ui.visuals().panel_fill);
        draw_top_bar(ui, state, &mut actions);
    });

    let drawer_host_rect = Rect::from_min_max(page_rect.min, pos2(page_rect.max.x, host_bottom));
    ui.scope_builder(UiBuilder::new().max_rect(drawer_host_rect), |ui| {
        let drawer_state = drawer_host_state(drawer_host_rect.size(), state);
        let slot_actions = std::cell::RefCell::new(Vec::new());
        let drawer_host_actions = draw_with_slots(
            ui,
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
        );

        actions.extend(slot_actions.into_inner());
        for action in drawer_host_actions {
            actions.extend(map_drawer_host_action(action, state));
        }
    });

    if state.is_audio_player_open {
        let audio_rect = Rect::from_min_max(pos2(page_rect.min.x, host_bottom), page_rect.max);
        ui.scope_builder(UiBuilder::new().max_rect(audio_rect), |ui| {
            actions.extend(draw_audio_host(ui, state));
        });
    }

    actions
}

fn draw_top_bar(ui: &mut Ui, state: &ChoreoMainState, actions: &mut Vec<ChoreoMainAction>) {
    ui.allocate_ui_with_layout(
        egui::vec2(ui.available_width(), TOP_BAR_HEIGHT_PX),
        egui::Layout::left_to_right(egui::Align::Center),
        |ui| {
            let nav_response = ui.add(
                MaterialIconButton::standard(nav_icon_name(state.is_nav_open))
                    .svg_data(nav_icon_svg(state.is_nav_open)),
            );
            if nav_response.clicked() {
                actions.push(top_bar_nav_action(state.is_nav_open));
            }
            let _ = nav_response.on_hover_text(if state.is_nav_open {
                "Close navigation"
            } else {
                "Open navigation"
            });

            ui.add_space(12.0);
            let previous_mode_index =
                effective_mode_index(state).clamp(0, mode_count() - 1) as usize;
            let mut selected_mode_index = Some(previous_mode_index);
            let mut mode_changed = false;
            ui.add_enabled_ui(state.is_mode_selection_enabled, |ui| {
                let mut mode_select = MaterialSelect::new(&mut selected_mode_index).label("Mode");
                for index in 0..mode_count() {
                    mode_select = mode_select.option(index as usize, mode_label(index));
                }
                mode_changed = ui.add(mode_select).changed();
            });
            if mode_changed
                && let Some(selected) = selected_mode_index
                && selected != previous_mode_index
            {
                actions.push(ChoreoMainAction::SelectMode {
                    index: selected as i32,
                });
            }

            ui.add_space(ui.available_width().max(0.0));

            let settings_response = ui.add(
                MaterialIconButton::standard(top_bar_settings_icon_name())
                    .svg_data(top_bar_settings_icon_svg()),
            );
            if settings_response.clicked() {
                actions.push(top_bar_settings_action(state.is_choreography_settings_open));
            }
            let _ = settings_response.on_hover_text(if state.is_choreography_settings_open {
                "Close choreography settings"
            } else {
                "Open choreography settings"
            });

            let home_response =
                ui.add(MaterialIconButton::standard(home_icon_name()).svg_data(home_icon_svg()));
            if home_response.clicked() {
                actions.push(ChoreoMainAction::ResetFloorViewport);
            }
            let _ = home_response.on_hover_text("Reset floor viewport");

            let open_image_response = ui.add(
                MaterialIconButton::standard(open_image_icon_name())
                    .svg_data(open_image_icon_svg()),
            );
            if open_image_response.clicked() {
                actions.push(ChoreoMainAction::RequestOpenImage {
                    file_path: String::new(),
                });
            }
            let _ = open_image_response.on_hover_text("Open image");

            let open_audio_response = ui.add(
                MaterialIconButton::standard(open_audio_icon_name())
                    .svg_data(open_audio_icon_svg()),
            );
            if open_audio_response.clicked() {
                actions.push(ChoreoMainAction::OpenAudioPanel);
            }
            let _ = open_audio_response.on_hover_text("Open audio");
        },
    );
}

fn draw_scenes_drawer(ui: &mut Ui, state: &ChoreoMainState, actions: &mut Vec<ChoreoMainAction>) {
    ui.allocate_ui_with_layout(
        egui::vec2(DRAWER_WIDTH_LEFT_PX, ui.available_height()),
        egui::Layout::top_down(egui::Align::LEFT),
        |ui| {
            Frame::group(ui.style()).show(ui, |ui| {
                ui.set_min_width(DRAWER_WIDTH_LEFT_PX);
                for (index, scene) in state.scenes.iter().enumerate() {
                    let is_selected = state.selected_scene_index == Some(index);
                    let label = if let Some(timestamp) = scene.timestamp_seconds {
                        format!("{} ({timestamp:.1}s)", scene.name)
                    } else {
                        scene.name.clone()
                    };
                    if ui.selectable_label(is_selected, label).clicked() {
                        actions.push(ChoreoMainAction::SelectScene { index });
                    }
                }
            });
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
pub fn drawer_host_state(
    viewport_size: egui::Vec2,
    state: &ChoreoMainState,
) -> MainPageDrawerHostState {
    MainPageDrawerHostState {
        left_drawer_width: DRAWER_WIDTH_LEFT_PX,
        right_drawer_width: DRAWER_WIDTH_RIGHT_PX,
        top_inset: TOP_BAR_HEIGHT_PX,
        inline_left: false,
        is_left_open: state.is_nav_open,
        is_right_open: state.is_choreography_settings_open,
        viewport_width: viewport_size.x.max(0.0),
        viewport_height: viewport_size.y.max(0.0),
        ..MainPageDrawerHostState::default()
    }
}

#[must_use]
pub fn map_drawer_host_action(
    action: MainPageDrawerHostAction,
    state: &ChoreoMainState,
) -> Vec<ChoreoMainAction> {
    match action {
        MainPageDrawerHostAction::OverlayClicked => {
            let mut actions = Vec::new();
            if state.is_nav_open {
                actions.push(ChoreoMainAction::CloseNav);
            }
            if state.is_choreography_settings_open {
                actions.push(ChoreoMainAction::CloseSettings);
            }
            actions
        }
        MainPageDrawerHostAction::Initialize
        | MainPageDrawerHostAction::SetInlineLeft { .. }
        | MainPageDrawerHostAction::SetLeftOpen { .. }
        | MainPageDrawerHostAction::SetRightOpen { .. }
        | MainPageDrawerHostAction::SetTopInset { .. } => Vec::new(),
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
    nav_icon_spec(is_nav_open).svg
}

#[must_use]
pub fn top_bar_settings_icon_svg() -> &'static str {
    top_bar_settings_icon_spec().svg
}

#[must_use]
pub fn home_icon_svg() -> &'static str {
    home_icon_spec().svg
}

#[must_use]
pub fn open_image_icon_svg() -> &'static str {
    open_image_icon_spec().svg
}

#[must_use]
pub fn open_audio_icon_svg() -> &'static str {
    open_audio_icon_spec().svg
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
        2 => "Rotate Center",
        3 => "Rotate Dancer",
        4 => "Scale",
        5 => "Line of Sight",
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
