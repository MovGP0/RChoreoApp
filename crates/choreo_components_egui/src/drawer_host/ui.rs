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
    let mut actions: Vec<DrawerHostAction> = Vec::new();
    let host_rect = ui.max_rect();
    let layout = compute_layout(host_rect, state);
    let painter = ui.painter();
    let _ = layout.content_rect;

    if overlay_visible(state) {
        let response = ui.interact(
            layout.overlay_rect,
            ui.id().with("drawer_host_overlay"),
            Sense::click(),
        );
        painter.rect_filled(layout.overlay_rect, 0.0, state.overlay_color);
        if response.clicked() {
            actions.push(DrawerHostAction::OverlayClicked);
        }
    }

    if state.inline_left || state.is_left_open {
        painter.rect_filled(layout.left_panel_rect, 0.0, state.drawer_background);
    }
    if state.is_right_open {
        painter.rect_filled(layout.right_panel_rect, 0.0, state.drawer_background);
    }
    if state.is_top_open {
        painter.rect_filled(layout.top_panel_rect, 0.0, state.drawer_background);
    }
    if state.is_bottom_open {
        painter.rect_filled(layout.bottom_panel_rect, 0.0, state.drawer_background);
    }

    actions
}
