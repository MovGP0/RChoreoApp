use egui::Frame;
use egui::Layout;
use egui::Rect;
use egui::Ui;
use egui::pos2;
use egui::vec2;

use crate::audio_player;
use crate::choreo_main::actions::ChoreoMainAction;
use crate::choreo_main::state::ChoreoMainState;
use crate::choreography_settings;
use crate::floor;
use crate::material::components::drawer_host::ui::draw_with_slots_in_rect;

pub use super::layout::audio_panel_height_px;
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
use super::layout::DRAWER_WIDTH_RIGHT_PX;
use super::layout::GRID_12_PX;
use super::scene_pane::draw_scenes_drawer;
use super::top_bar::draw_top_bar;

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
            .order(egui::Order::Middle)
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
