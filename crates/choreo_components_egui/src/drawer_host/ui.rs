use egui::Area;
use egui::Id;
use egui::Order;
use egui::Rect;
use egui::Sense;
use egui::Ui;
use egui::pos2;

use super::actions::DrawerHostAction;
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
pub fn inline_left_width(state: &DrawerHostState) -> f32 {
    if state.inline_left && state.is_left_open {
        state.left_drawer_width
    } else {
        0.0
    }
}

#[must_use]
pub fn overlay_visible(state: &DrawerHostState) -> bool {
    (state.is_left_open && !state.inline_left && state.left_close_on_click_away)
        || (state.is_right_open && state.right_close_on_click_away)
        || (state.is_top_open && state.top_close_on_click_away)
        || (state.is_bottom_open && state.bottom_close_on_click_away)
}

#[must_use]
pub fn compute_layout(container_rect: Rect, state: &DrawerHostState) -> DrawerHostLayout {
    let inline_width = inline_left_width(state);
    let content_min = pos2(container_rect.min.x + inline_width, container_rect.min.y);
    let content_rect = Rect::from_min_max(content_min, container_rect.max);

    let panel_min = pos2(container_rect.min.x, container_rect.min.y + state.top_inset);
    let panel_rect = Rect::from_min_max(panel_min, container_rect.max);

    let left_x = if state.inline_left || state.is_left_open {
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
    let layout = compute_layout(host_rect, state);

    Area::new(Id::new((id_source, "content")))
        .order(Order::Middle)
        .fixed_pos(layout.content_rect.min)
        .show(context, |ui| {
            ui.set_min_size(layout.content_rect.size());
            draw_content(ui);
        });

    if overlay_visible(state) {
        let overlay_clicked = Area::new(Id::new((id_source, "overlay")))
            .order(Order::Foreground)
            .fixed_pos(layout.overlay_rect.min)
            .show(context, |ui| {
                let overlay_rect =
                    Rect::from_min_size(egui::Pos2::ZERO, layout.overlay_rect.size());
                let response = ui.allocate_rect(overlay_rect, Sense::click());
                ui.painter()
                    .rect_filled(overlay_rect, 0.0, state.overlay_color);
                response.clicked()
            })
            .inner;
        if overlay_clicked {
            actions.push(DrawerHostAction::OverlayClicked);
        }
    }

    if state.inline_left || state.is_left_open {
        Area::new(Id::new((id_source, "left_panel")))
            .order(Order::Foreground)
            .fixed_pos(layout.left_panel_rect.min)
            .show(context, |ui| {
                ui.set_min_size(layout.left_panel_rect.size());
                ui.painter().rect_filled(
                    Rect::from_min_size(egui::Pos2::ZERO, layout.left_panel_rect.size()),
                    0.0,
                    state.drawer_background,
                );
                draw_left_panel(ui);
            });
    }
    if state.is_right_open {
        Area::new(Id::new((id_source, "right_panel")))
            .order(Order::Foreground)
            .fixed_pos(layout.right_panel_rect.min)
            .show(context, |ui| {
                ui.set_min_size(layout.right_panel_rect.size());
                ui.painter().rect_filled(
                    Rect::from_min_size(egui::Pos2::ZERO, layout.right_panel_rect.size()),
                    0.0,
                    state.drawer_background,
                );
                draw_right_panel(ui);
            });
    }
    if state.is_top_open {
        Area::new(Id::new((id_source, "top_panel")))
            .order(Order::Foreground)
            .fixed_pos(layout.top_panel_rect.min)
            .show(context, |ui| {
                ui.set_min_size(layout.top_panel_rect.size());
                ui.painter().rect_filled(
                    Rect::from_min_size(egui::Pos2::ZERO, layout.top_panel_rect.size()),
                    0.0,
                    state.drawer_background,
                );
                draw_top_panel(ui);
            });
    }
    if state.is_bottom_open {
        Area::new(Id::new((id_source, "bottom_panel")))
            .order(Order::Foreground)
            .fixed_pos(layout.bottom_panel_rect.min)
            .show(context, |ui| {
                ui.set_min_size(layout.bottom_panel_rect.size());
                ui.painter().rect_filled(
                    Rect::from_min_size(egui::Pos2::ZERO, layout.bottom_panel_rect.size()),
                    0.0,
                    state.drawer_background,
                );
                draw_bottom_panel(ui);
            });
    }

    actions
}
