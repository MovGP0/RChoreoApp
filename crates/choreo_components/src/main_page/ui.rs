use egui::Color32;
use egui::Layout;
use egui::Stroke;
use egui::StrokeKind;
use egui::Ui;
use egui::UiBuilder;
use egui::vec2;

use crate::audio_player;
use crate::choreo_main::actions::ChoreoMainAction;
use crate::choreo_main::state::ChoreoMainState;
use crate::choreography_settings;
use crate::floor;
use crate::material::components::drawer_host::ui::draw_with_slots_in_rect;
use crate::material::styling::material_palette::material_palette_for_visuals;

pub use super::layout::audio_panel_height_px;
pub use super::layout::audio_panel_rect;
pub use super::layout::drawer_host_rect;
pub use super::layout::drawer_host_state;
pub use super::layout::shell_rect;
pub use super::layout::top_bar_rect;
pub use super::mappings::map_audio_host_action;
pub use super::mappings::map_choreography_settings_action;
pub use super::mappings::map_drawer_host_action;
pub use super::mappings::map_floor_host_action;
pub use super::mappings::map_scene_pane_action;
pub use super::mappings::top_bar_nav_action;
pub use super::mappings::top_bar_open_audio_action;
pub use super::mappings::top_bar_settings_action;
pub use super::scene_pane::scene_pane_state;
pub use super::top_bar::home_icon_name;
pub use super::top_bar::home_icon_svg;
pub use super::top_bar::mode_count;
pub use super::top_bar::mode_label;
pub use super::top_bar::mode_label_role;
pub use super::top_bar::nav_icon_name;
pub use super::top_bar::nav_icon_svg;
pub use super::top_bar::open_audio_icon_name;
pub use super::top_bar::open_audio_icon_svg;
pub use super::top_bar::open_image_icon_name;
pub use super::top_bar::open_image_icon_svg;
pub use super::top_bar::top_bar_action_count;
pub use super::top_bar::top_bar_action_icon_tokens;
pub use super::top_bar::top_bar_action_icon_uris;
pub use super::top_bar::top_bar_settings_icon_name;
pub use super::top_bar::top_bar_settings_icon_svg;
pub use super::top_bar::translated_mode_labels;

use super::layout::AUDIO_PANEL_HEIGHT_PX;
use super::layout::GRID_12_PX;
use super::scene_pane::draw_scenes_drawer;
use super::top_bar::draw_top_bar;

const TOP_BAR_DEBUG_BORDER_COLOR: Color32 = Color32::from_rgb(255, 0, 255);
const DRAWER_HOST_DEBUG_BORDER_COLOR: Color32 = Color32::from_rgb(255, 128, 0);
const DRAWER_HOST_DEBUG_BORDER_COLOR_SECONDARY: Color32 = Color32::from_rgb(255, 255, 255);
const FLOOR_DEBUG_BORDER_COLOR: Color32 = Color32::from_rgb(255, 0, 0);
const SETTINGS_DEBUG_BORDER_COLOR: Color32 = Color32::from_rgb(0, 160, 255);
const AUDIO_DEBUG_BORDER_COLOR: Color32 = Color32::from_rgb(255, 208, 0);
const PANEL_DEBUG_CONTENT_PADDING_PX: f32 = 4.0;

#[must_use]
pub const fn top_bar_layer_order() -> egui::Order {
    egui::Order::Debug
}

#[must_use]
pub const fn audio_panel_layer_order() -> egui::Order {
    egui::Order::Tooltip
}

pub fn draw(ui: &mut Ui, state: &ChoreoMainState) -> Vec<ChoreoMainAction> {
    let mut actions: Vec<ChoreoMainAction> = Vec::new();
    let palette = material_palette_for_visuals(ui.visuals());
    ui.spacing_mut().item_spacing = vec2(GRID_12_PX, GRID_12_PX);
    let page_rect = shell_rect(ui);
    let audio_panel_height = audio_panel_height_px(state.is_audio_player_open);
    let top_bar_rect = top_bar_rect(page_rect);
    let drawer_host_rect = drawer_host_rect(page_rect, audio_panel_height);
    egui::Area::new(egui::Id::new("main_page_top_bar"))
        .order(top_bar_layer_order())
        .fixed_pos(top_bar_rect.min)
        .show(ui.ctx(), |ui| {
            ui.set_clip_rect(top_bar_rect);
            ui.painter()
                .rect_filled(top_bar_rect, 0.0, palette.background);
            if cfg!(debug_assertions) {
                ui.painter().rect_stroke(
                    top_bar_rect,
                    0.0,
                    Stroke::new(1.0, TOP_BAR_DEBUG_BORDER_COLOR),
                    StrokeKind::Inside,
                );
            }
            ui.set_min_size(top_bar_rect.size());
            let _ = ui.scope_builder(UiBuilder::new().max_rect(top_bar_rect), |ui| {
                draw_top_bar(ui, state, &mut actions);
            });
        });

    if cfg!(debug_assertions) {
        ui.painter().rect_stroke(
            drawer_host_rect,
            0.0,
            Stroke::new(1.0, DRAWER_HOST_DEBUG_BORDER_COLOR),
            StrokeKind::Inside,
        );
        ui.painter().rect_stroke(
            drawer_host_rect.shrink(2.0),
            0.0,
            Stroke::new(1.0, DRAWER_HOST_DEBUG_BORDER_COLOR_SECONDARY),
            StrokeKind::Inside,
        );
    }

    let drawer_state = drawer_host_state(drawer_host_rect.size(), state);
    let slot_actions = std::cell::RefCell::new(Vec::new());
    let drawer_host_actions = draw_with_slots_in_rect(
        ui.ctx(),
        drawer_host_rect,
        "main_page",
        &drawer_state,
        |ui| {
            slot_actions.borrow_mut().extend(draw_floor_host_content(
                ui,
                &state.floor_state,
                state.is_choreography_settings_open,
            ));
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
        let audio_rect = audio_panel_rect(page_rect, audio_panel_height);
        egui::Area::new(egui::Id::new("main_page_audio_host"))
            .order(audio_panel_layer_order())
            .fixed_pos(audio_rect.min)
            .show(ui.ctx(), |ui| {
                ui.set_clip_rect(audio_rect);
                ui.set_width(audio_rect.width());
                ui.set_min_height(audio_rect.height());
                ui.painter()
                    .rect_filled(audio_rect, 0.0, palette.background);
                if cfg!(debug_assertions) {
                    ui.painter().rect_stroke(
                        audio_rect,
                        0.0,
                        Stroke::new(1.0, AUDIO_DEBUG_BORDER_COLOR),
                        StrokeKind::Inside,
                    );
                }
                let _ = ui.scope_builder(UiBuilder::new().max_rect(audio_rect), |ui| {
                    actions.extend(draw_audio_host(ui, state));
                });
            });
    }

    actions
}

#[must_use]
pub fn floor_content_rect(slot_rect: egui::Rect, is_right_drawer_open: bool) -> egui::Rect {
    if !is_right_drawer_open {
        return slot_rect;
    }

    let right =
        (slot_rect.max.x - choreography_settings::ui::drawer_width_token()).max(slot_rect.min.x);
    egui::Rect::from_min_max(slot_rect.min, egui::pos2(right, slot_rect.max.y))
}

fn draw_floor_host_content(
    ui: &mut Ui,
    state: &floor::state::FloorState,
    is_right_drawer_open: bool,
) -> Vec<ChoreoMainAction> {
    let mut actions: Vec<ChoreoMainAction> = Vec::new();
    let host_rect = floor_content_rect(ui.max_rect(), is_right_drawer_open);
    ui.set_clip_rect(host_rect);
    ui.set_min_size(host_rect.size());
    ui.painter()
        .rect_filled(
            host_rect,
            0.0,
            material_palette_for_visuals(ui.visuals()).surface_container_low,
        );
    if cfg!(debug_assertions) {
        ui.painter().rect_stroke(
            host_rect,
            0.0,
            Stroke::new(1.0, FLOOR_DEBUG_BORDER_COLOR),
            StrokeKind::Inside,
        );
    }
    let inner_rect = host_rect.shrink(PANEL_DEBUG_CONTENT_PADDING_PX);
    let _ = ui.scope_builder(UiBuilder::new().max_rect(inner_rect), |ui| {
        let floor_actions = floor::ui::draw(ui, state);
        actions.extend(floor_actions.into_iter().map(map_floor_host_action));
    });
    actions
}

fn draw_settings_drawer(ui: &mut Ui, state: &ChoreoMainState) -> Vec<ChoreoMainAction> {
    let mut actions = Vec::new();
    let panel_rect = ui.max_rect();
    let panel_width = panel_rect.width();
    ui.set_clip_rect(panel_rect);
    ui.set_min_size(panel_rect.size());
    ui.set_width(panel_width);
    ui.set_min_width(panel_width);
    ui.painter()
        .rect_filled(
            panel_rect,
            0.0,
            material_palette_for_visuals(ui.visuals()).surface_container,
        );
    if cfg!(debug_assertions) {
        ui.painter().rect_stroke(
            panel_rect,
            0.0,
            Stroke::new(1.0, SETTINGS_DEBUG_BORDER_COLOR),
            StrokeKind::Inside,
        );
        ui.painter().text(
            panel_rect.left_top()
                + vec2(
                    PANEL_DEBUG_CONTENT_PADDING_PX,
                    PANEL_DEBUG_CONTENT_PADDING_PX,
                ),
            egui::Align2::LEFT_TOP,
            format!("{:.0} x {:.0}", panel_rect.width(), panel_rect.height()),
            egui::TextStyle::Monospace.resolve(ui.style()),
            SETTINGS_DEBUG_BORDER_COLOR,
        );
    }
    let inner_rect = panel_rect.shrink(PANEL_DEBUG_CONTENT_PADDING_PX);
    let _ = ui.scope_builder(UiBuilder::new().max_rect(inner_rect), |ui| {
        let settings_actions =
            choreography_settings::ui::draw(ui, &state.choreography_settings_state);
        actions.extend(
            settings_actions
                .into_iter()
                .map(map_choreography_settings_action),
        );
    });
    actions
}

fn draw_audio_host(ui: &mut Ui, state: &ChoreoMainState) -> Vec<ChoreoMainAction> {
    let mut actions: Vec<ChoreoMainAction> = Vec::new();
    let host_width = ui.available_width();
    ui.allocate_ui_with_layout(
        vec2(host_width, AUDIO_PANEL_HEIGHT_PX),
        Layout::left_to_right(egui::Align::Center),
        |ui| {
            ui.set_width(host_width);
            ui.set_min_width(host_width);
            ui.set_min_height(AUDIO_PANEL_HEIGHT_PX);
            for action in audio_player::ui::draw(ui, &state.audio_player_state) {
                actions.extend(map_audio_host_action(action));
            }
        },
    );
    actions
}
