use egui::Area;
use egui::Id;
use egui::Order;
use egui::Pos2;
use egui::Rect;
use egui::Sense;
use egui::Ui;
use egui::UiBuilder;
use egui::pos2;

use super::actions::DrawerHostAction;
use super::state::DrawerHostOpenMode;
use super::state::DrawerHostState;

#[derive(Debug, Clone, Copy)]
pub struct DrawerHostLayout {
    pub content_rect: Rect,
    pub overlay_rect: Rect,
    pub panel_rect: Rect,
    pub left_panel_rect: Rect,
    pub right_panel_rect: Rect,
    pub top_panel_rect: Rect,
    pub bottom_panel_rect: Rect,
}

#[must_use]
pub fn inline_left_width(state: &DrawerHostState, host_width: f32) -> f32 {
    if is_inline_left_layout(state, host_width) && state.is_left_open {
        state.left_drawer_width
    } else {
        0.0
    }
}

#[must_use]
pub fn is_inline_left_layout(state: &DrawerHostState, host_width: f32) -> bool {
    if state.inline_left {
        return true;
    }

    match state.open_mode {
        DrawerHostOpenMode::Default | DrawerHostOpenMode::Modal => false,
        DrawerHostOpenMode::Standard => host_width >= state.responsive_breakpoint,
    }
}

#[must_use]
pub fn overlay_visible(state: &DrawerHostState, host_width: f32) -> bool {
    overlay_close_action(state, host_width).is_some()
}

#[must_use]
fn overlay_close_action(state: &DrawerHostState, host_width: f32) -> Option<DrawerHostAction> {
    let close_left = state.is_left_open
        && !is_inline_left_layout(state, host_width)
        && state.left_close_on_click_away;
    let close_right = state.is_right_open && state.right_close_on_click_away;
    let close_top = state.is_top_open && state.top_close_on_click_away;
    let close_bottom = state.is_bottom_open && state.bottom_close_on_click_away;

    if close_left || close_right || close_top || close_bottom {
        Some(DrawerHostAction::OverlayClicked {
            close_left,
            close_right,
            close_top,
            close_bottom,
        })
    } else {
        None
    }
}

#[must_use]
pub fn compute_layout(container_rect: Rect, state: &DrawerHostState) -> DrawerHostLayout {
    let inline_width = inline_left_width(state, container_rect.width());
    let content_min = pos2(container_rect.min.x + inline_width, container_rect.min.y);
    let content_rect = Rect::from_min_max(content_min, container_rect.max);

    let panel_min = pos2(container_rect.min.x, container_rect.min.y + state.top_inset);
    let panel_rect = Rect::from_min_max(panel_min, container_rect.max);

    let left_x = if state.is_left_open {
        panel_rect.min.x
    } else {
        panel_rect.min.x - state.left_drawer_width
    };
    let left_panel_rect = Rect::from_min_size(
        pos2(left_x, panel_rect.min.y),
        egui::vec2(state.left_drawer_width, panel_rect.height()),
    );

    let right_x = if state.is_right_open {
        panel_rect.max.x - state.right_drawer_width
    } else {
        panel_rect.max.x
    };
    let right_min = pos2(right_x, panel_rect.min.y);
    let right_panel_rect = Rect::from_min_size(
        right_min,
        egui::vec2(state.right_drawer_width, panel_rect.height()),
    );

    let top_y = if state.is_top_open {
        panel_rect.min.y
    } else {
        panel_rect.min.y - state.top_drawer_height
    };
    let top_panel_rect = Rect::from_min_size(
        pos2(panel_rect.min.x, top_y),
        egui::vec2(panel_rect.width(), state.top_drawer_height),
    );

    let bottom_y = if state.is_bottom_open {
        panel_rect.max.y - state.bottom_drawer_height
    } else {
        panel_rect.max.y
    };
    let bottom_min = pos2(panel_rect.min.x, bottom_y);
    let bottom_panel_rect = Rect::from_min_size(
        bottom_min,
        egui::vec2(panel_rect.width(), state.bottom_drawer_height),
    );

    DrawerHostLayout {
        content_rect,
        overlay_rect: container_rect,
        panel_rect,
        left_panel_rect,
        right_panel_rect,
        top_panel_rect,
        bottom_panel_rect,
    }
}

#[must_use]
pub fn draw(ui: &mut Ui, state: &DrawerHostState) -> Vec<DrawerHostAction> {
    draw_with_slots(
        ui,
        "drawer_host",
        state,
        |_| {},
        |_| {},
        |_| {},
        |_| {},
        |_| {},
    )
}

#[must_use]
#[allow(clippy::too_many_arguments)]
pub fn draw_with_slots(
    ui: &mut Ui,
    id_source: &str,
    state: &DrawerHostState,
    draw_content: impl FnOnce(&mut Ui),
    draw_left_panel: impl FnOnce(&mut Ui),
    draw_right_panel: impl FnOnce(&mut Ui),
    draw_top_panel: impl FnOnce(&mut Ui),
    draw_bottom_panel: impl FnOnce(&mut Ui),
) -> Vec<DrawerHostAction> {
    draw_with_slots_in_rect(
        ui.ctx(),
        ui.max_rect(),
        id_source,
        state,
        draw_content,
        draw_left_panel,
        draw_right_panel,
        draw_top_panel,
        draw_bottom_panel,
    )
}

#[must_use]
#[allow(clippy::too_many_arguments)]
pub fn draw_with_slots_in_rect(
    context: &egui::Context,
    host_rect: Rect,
    id_source: &str,
    state: &DrawerHostState,
    draw_content: impl FnOnce(&mut Ui),
    draw_left_panel: impl FnOnce(&mut Ui),
    draw_right_panel: impl FnOnce(&mut Ui),
    draw_top_panel: impl FnOnce(&mut Ui),
    draw_bottom_panel: impl FnOnce(&mut Ui),
) -> Vec<DrawerHostAction> {
    let mut actions: Vec<DrawerHostAction> = Vec::new();
    let local_host_rect = Rect::from_min_size(Pos2::ZERO, host_rect.size());
    let local_layout = compute_layout(local_host_rect, state);
    let overlay_action = overlay_close_action(state, host_rect.width());

    Area::new(Id::new((id_source, "host")))
        .order(Order::Middle)
        .fixed_pos(host_rect.min)
        .show(context, |ui| {
            ui.set_min_size(host_rect.size());

            let _ = ui.scope_builder(
                UiBuilder::new()
                    .id_salt((id_source, "content"))
                    .max_rect(local_layout.content_rect),
                |ui| {
                    draw_content(ui);
                },
            );

            if let Some(overlay_action) = overlay_action {
                let overlay_response = ui.allocate_rect(local_layout.overlay_rect, Sense::click());
                ui.painter()
                    .rect_filled(local_layout.overlay_rect, 0.0, state.overlay_color);

                if overlay_response.clicked()
                    && pointer_not_in_open_panel(ui, host_rect.min, &local_layout, state)
                {
                    actions.push(overlay_action);
                }
            }

            if state.is_top_open {
                ui.painter().rect_filled(
                    local_layout.top_panel_rect,
                    0.0,
                    state.drawer_background,
                );
                let _ = ui.scope_builder(
                    UiBuilder::new()
                        .id_salt((id_source, "top_panel"))
                        .max_rect(local_layout.top_panel_rect),
                    |ui| {
                        draw_top_panel(ui);
                    },
                );
            }

            if state.is_bottom_open {
                ui.painter().rect_filled(
                    local_layout.bottom_panel_rect,
                    0.0,
                    state.drawer_background,
                );
                let _ = ui.scope_builder(
                    UiBuilder::new()
                        .id_salt((id_source, "bottom_panel"))
                        .max_rect(local_layout.bottom_panel_rect),
                    |ui| {
                        draw_bottom_panel(ui);
                    },
                );
            }

            if state.is_left_open {
                ui.painter().rect_filled(
                    local_layout.left_panel_rect,
                    0.0,
                    state.drawer_background,
                );
                let _ = ui.scope_builder(
                    UiBuilder::new()
                        .id_salt((id_source, "left_panel"))
                        .max_rect(local_layout.left_panel_rect),
                    |ui| {
                        draw_left_panel(ui);
                    },
                );
            }

            if state.is_right_open {
                ui.painter().rect_filled(
                    local_layout.right_panel_rect,
                    0.0,
                    state.drawer_background,
                );
                let _ = ui.scope_builder(
                    UiBuilder::new()
                        .id_salt((id_source, "right_panel"))
                        .max_rect(local_layout.right_panel_rect),
                    |ui| {
                        draw_right_panel(ui);
                    },
                );
            }
        });

    actions
}

fn pointer_not_in_open_panel(
    ui: &Ui,
    host_origin: Pos2,
    layout: &DrawerHostLayout,
    state: &DrawerHostState,
) -> bool {
    let pointer_pos = ui.ctx().pointer_latest_pos();
    let Some(pointer_pos) = pointer_pos else {
        return false;
    };
    let local_pointer_pos = pos2(
        pointer_pos.x - host_origin.x,
        pointer_pos.y - host_origin.y,
    );

    if state.is_left_open && layout.left_panel_rect.contains(local_pointer_pos) {
        return false;
    }
    if state.is_right_open && layout.right_panel_rect.contains(local_pointer_pos) {
        return false;
    }
    if state.is_top_open && layout.top_panel_rect.contains(local_pointer_pos) {
        return false;
    }
    if state.is_bottom_open && layout.bottom_panel_rect.contains(local_pointer_pos) {
        return false;
    }

    true
}
