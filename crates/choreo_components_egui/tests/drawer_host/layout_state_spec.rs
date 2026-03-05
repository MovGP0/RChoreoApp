use crate::drawer_host::Report;
use crate::drawer_host::state::DrawerHostState;
use crate::drawer_host::ui::compute_layout;
use egui::Rect;
use egui::pos2;

#[test]
fn layout_state_spec() {
    let suite = rspec::describe("drawer host layout state", (), |spec| {
        spec.it(
            "keeps overlay bounds on full host while panel area uses top inset",
            |_| {
                let state = DrawerHostState {
                    top_inset: 84.0,
                    ..DrawerHostState::default()
                };
                let host_rect = Rect::from_min_max(pos2(20.0, 30.0), pos2(1620.0, 930.0));
                let layout = compute_layout(host_rect, &state);

                assert_eq!(layout.overlay_rect, host_rect);
                assert_eq!(layout.panel_rect.min.y, 114.0);
                assert_eq!(layout.panel_rect.height(), 816.0);
            },
        );

        spec.it(
            "positions closed drawers off-screen from the panel area",
            |_| {
                let state = DrawerHostState {
                    left_drawer_width: 320.0,
                    right_drawer_width: 480.0,
                    top_drawer_height: 240.0,
                    bottom_drawer_height: 300.0,
                    top_inset: 36.0,
                    ..DrawerHostState::default()
                };
                let host_rect = Rect::from_min_max(pos2(20.0, 30.0), pos2(1220.0, 930.0));
                let layout = compute_layout(host_rect, &state);

                assert_eq!(layout.left_panel_rect.left(), -300.0);
                assert_eq!(layout.right_panel_rect.left(), 1220.0);
                assert_eq!(layout.top_panel_rect.top(), -174.0);
                assert_eq!(layout.bottom_panel_rect.top(), 930.0);
            },
        );
    });

    let report = crate::drawer_host::run_suite(&suite);
    assert!(report.is_success());
}
