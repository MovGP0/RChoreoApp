use egui::Ui;

use super::actions::MainPageDrawerHostAction;
use super::state::MainPageDrawerHostState;

pub fn draw(ui: &mut Ui, state: &MainPageDrawerHostState) -> Vec<MainPageDrawerHostAction> {
    let mut actions: Vec<MainPageDrawerHostAction> = Vec::new();

    ui.spacing_mut().item_spacing = egui::vec2(12.0, 12.0);
    ui.heading("Main Page Drawer Host");

    ui.horizontal(|ui| {
        if ui
            .button(if state.is_left_open {
                "Close Left Drawer"
            } else {
                "Open Left Drawer"
            })
            .clicked()
        {
            actions.push(MainPageDrawerHostAction::SetLeftOpen {
                is_open: !state.is_left_open,
            });
        }

        if ui
            .button(if state.is_right_open {
                "Close Right Drawer"
            } else {
                "Open Right Drawer"
            })
            .clicked()
        {
            actions.push(MainPageDrawerHostAction::SetRightOpen {
                is_open: !state.is_right_open,
            });
        }
    });

    let mut inline_left = state.inline_left;
    if ui
        .checkbox(&mut inline_left, "Inline left drawer")
        .changed()
    {
        actions.push(MainPageDrawerHostAction::SetInlineLeft {
            inline: inline_left,
        });
    }

    if state.overlay_visible() && ui.button("Overlay Click").clicked() {
        actions.push(MainPageDrawerHostAction::OverlayClicked);
    }

    let content = state.content_rect();
    let left = state.left_panel();
    let right = state.right_panel();

    ui.label(format!(
        "inline_left_width: {:.0}",
        state.inline_left_width()
    ));
    ui.label(format!("overlay_visible: {}", state.overlay_visible()));
    ui.label(format!(
        "content rect: x={:.0}, y={:.0}, w={:.0}, h={:.0}",
        content.x, content.y, content.width, content.height
    ));
    ui.label(format!(
        "left panel: visible={}, x={:.0}, w={:.0}, h={:.0}",
        left.visible, left.x, left.width, left.height
    ));
    ui.label(format!(
        "right panel: visible={}, x={:.0}, w={:.0}, h={:.0}",
        right.visible, right.x, right.width, right.height
    ));

    actions
}
