use egui::Area;
use egui::Id;
use egui::Order;
use egui::Rect;
use egui::Sense;
use egui::Ui;
use egui::pos2;
use egui::vec2;

use super::actions::MainPageDrawerHostAction;
use super::state::MainPageDrawerHostState;

#[derive(Debug, Clone, Copy)]
pub struct MainPageDrawerHostLayout {
    pub content_rect: Rect,
    pub panel_rect: Rect,
    pub left_panel_rect: Rect,
    pub right_panel_rect: Rect,
}

#[must_use]
pub fn compute_layout(container_rect: Rect, state: &MainPageDrawerHostState) -> MainPageDrawerHostLayout {
    let inline_width = state.inline_left_width();
    let top_inset = state.top_inset.max(0.0);

    let content_min = pos2(container_rect.min.x + inline_width, container_rect.min.y + top_inset);
    let content_rect = Rect::from_min_max(content_min, container_rect.max);

    let panel_min = pos2(container_rect.min.x, container_rect.min.y + top_inset);
    let panel_rect = Rect::from_min_max(panel_min, container_rect.max);

    let left_panel_rect = Rect::from_min_size(
        panel_rect.min,
        vec2(state.left_drawer_width, panel_rect.height()),
    );

    let right_panel_min = pos2(
        panel_rect.max.x - state.right_drawer_width,
        panel_rect.min.y,
    );
    let right_panel_rect = Rect::from_min_size(
        right_panel_min,
        vec2(state.right_drawer_width, panel_rect.height()),
    );

    MainPageDrawerHostLayout {
        content_rect,
        panel_rect,
        left_panel_rect,
        right_panel_rect,
    }
}

#[must_use]
pub fn draw(ui: &mut Ui, state: &MainPageDrawerHostState) -> Vec<MainPageDrawerHostAction> {
    draw_with_slots(
        ui,
        "main_page_drawer_host",
        state,
        |_| {},
        |_| {},
        |_| {},
    )
}

#[must_use]
pub fn draw_with_slots(
    ui: &mut Ui,
    id_source: &str,
    state: &MainPageDrawerHostState,
    draw_content: impl FnOnce(&mut Ui),
    draw_left_panel: impl FnOnce(&mut Ui),
    draw_right_panel: impl FnOnce(&mut Ui),
) -> Vec<MainPageDrawerHostAction> {
    let mut actions: Vec<MainPageDrawerHostAction> = Vec::new();
    let host_rect = ui.max_rect();
    let layout = compute_layout(host_rect, state);

    Area::new(Id::new((id_source, "content")))
        .order(Order::Middle)
        .fixed_pos(layout.content_rect.min)
        .show(ui.ctx(), |ui| {
            ui.set_min_size(layout.content_rect.size());
            draw_content(ui);
        });

    if state.overlay_visible() {
        let overlay_clicked = Area::new(Id::new((id_source, "overlay")))
            .order(Order::Foreground)
            .fixed_pos(layout.panel_rect.min)
            .show(ui.ctx(), |ui| {
                let overlay_rect = Rect::from_min_size(egui::Pos2::ZERO, layout.panel_rect.size());
                let response = ui.allocate_rect(overlay_rect, Sense::click());
                ui.painter().rect_filled(overlay_rect, 0.0, state.overlay_color);
                response.clicked()
            })
            .inner;
        if overlay_clicked {
            actions.push(MainPageDrawerHostAction::OverlayClicked);
        }
    }

    if state.inline_left || state.is_left_open {
        Area::new(Id::new((id_source, "left_panel")))
            .order(Order::Foreground)
            .fixed_pos(layout.left_panel_rect.min)
            .show(ui.ctx(), |ui| {
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
            .show(ui.ctx(), |ui| {
                ui.set_min_size(layout.right_panel_rect.size());
                ui.painter().rect_filled(
                    Rect::from_min_size(egui::Pos2::ZERO, layout.right_panel_rect.size()),
                    0.0,
                    state.drawer_background,
                );
                draw_right_panel(ui);
            });
    }

    actions
}
