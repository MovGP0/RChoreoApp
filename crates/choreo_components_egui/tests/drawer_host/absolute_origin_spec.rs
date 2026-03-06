use crate::drawer_host::state::DrawerHostState;
use crate::drawer_host::ui::compute_layout;
use crate::drawer_host::ui::draw_with_slots_in_rect;
use egui::Rect;
use egui::pos2;

#[test]
fn absolute_origin_spec() {
    let state = DrawerHostState {
        is_left_open: true,
        is_right_open: true,
        left_drawer_width: 324.0,
        right_drawer_width: 480.0,
        ..DrawerHostState::default()
    };
    let host_rect = Rect::from_min_max(pos2(120.0, 84.0), pos2(1320.0, 780.0));
    let layout = compute_layout(host_rect, &state);
    assert_eq!(layout.content_rect.min, host_rect.min);
    assert_eq!(layout.left_panel_rect.min, host_rect.min);
    assert_eq!(layout.right_panel_rect.min.x, host_rect.max.x - state.right_drawer_width);
    assert_eq!(layout.right_panel_rect.min.y, host_rect.min.y);

    let context = egui::Context::default();
    let output = context.run(egui::RawInput::default(), |ctx| {
        let _ = draw_with_slots_in_rect(
            ctx,
            host_rect,
            "drawer_host_absolute_origin",
            &state,
            |_| {},
            |_| {},
            |_| {},
            |_| {},
            |_| {},
        );
    });

    assert!(!output.shapes.is_empty());
}
