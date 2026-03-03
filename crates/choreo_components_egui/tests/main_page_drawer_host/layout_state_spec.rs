use crate::main_page_drawer_host::Report;
use crate::main_page_drawer_host::state::MainPageDrawerHostState;
use crate::main_page_drawer_host::ui::compute_layout;
use egui::Rect;
use egui::pos2;

#[test]
fn layout_state_spec() {
    let suite = rspec::describe("main page drawer host layout state", (), |spec| {
        spec.it(
            "computes inline left width when left drawer is inline and open",
            |_| {
                let state = MainPageDrawerHostState {
                    inline_left: true,
                    is_left_open: true,
                    left_drawer_width: 320.0,
                    ..MainPageDrawerHostState::default()
                };

                assert_eq!(state.inline_left_width(), 320.0);
            },
        );

        spec.it("hides inline left width when drawer is closed", |_| {
            let state = MainPageDrawerHostState::default();
            assert_eq!(state.inline_left_width(), 0.0);
        });

        spec.it(
            "shows overlay for left drawer when not inline and click-away is enabled",
            |_| {
                let state = MainPageDrawerHostState {
                    is_left_open: true,
                    inline_left: false,
                    left_close_on_click_away: true,
                    ..MainPageDrawerHostState::default()
                };

                assert!(state.overlay_visible());
            },
        );

        spec.it(
            "shows overlay for right drawer when click-away is enabled",
            |_| {
                let state = MainPageDrawerHostState {
                    is_right_open: true,
                    right_close_on_click_away: true,
                    ..MainPageDrawerHostState::default()
                };

                assert!(state.overlay_visible());
            },
        );

        spec.it(
            "hides overlay when click-away is disabled for both drawers",
            |_| {
                let state = MainPageDrawerHostState {
                    is_left_open: true,
                    is_right_open: true,
                    left_close_on_click_away: false,
                    right_close_on_click_away: false,
                    ..MainPageDrawerHostState::default()
                };

                assert!(!state.overlay_visible());
            },
        );

        spec.it(
            "computes host geometry from viewport and top inset",
            |_| {
                let state = MainPageDrawerHostState {
                    left_drawer_width: 320.0,
                    right_drawer_width: 480.0,
                    top_inset: 84.0,
                    inline_left: false,
                    is_left_open: true,
                    is_right_open: true,
                    viewport_width: 1600.0,
                    viewport_height: 900.0,
                    ..MainPageDrawerHostState::default()
                };
                let host_rect = Rect::from_min_max(pos2(20.0, 30.0), pos2(1620.0, 930.0));
                let layout = compute_layout(host_rect, &state);

                assert_eq!(layout.content_rect.left(), 20.0);
                assert_eq!(layout.content_rect.top(), 114.0);
                assert_eq!(layout.content_rect.width(), 1600.0);
                assert_eq!(layout.content_rect.height(), 816.0);
                assert_eq!(layout.panel_rect.left(), 20.0);
                assert_eq!(layout.panel_rect.top(), 114.0);
                assert_eq!(layout.left_panel_rect.left(), 20.0);
                assert_eq!(layout.left_panel_rect.width(), 320.0);
                assert_eq!(layout.right_panel_rect.left(), 1140.0);
                assert_eq!(layout.right_panel_rect.width(), 480.0);
            },
        );
    });

    let report = crate::main_page_drawer_host::run_suite(&suite);
    assert!(report.is_success());
}
