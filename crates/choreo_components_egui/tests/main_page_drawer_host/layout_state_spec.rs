use crate::main_page_drawer_host::Report;
use crate::main_page_drawer_host::state::MainPageDrawerHostState;

#[test]
fn layout_state_spec() {
    let suite = rspec::describe("main page drawer host layout state", (), |spec| {
        spec.it("computes inline left width when left drawer is inline and open", |_| {
            let state = MainPageDrawerHostState {
                inline_left: true,
                is_left_open: true,
                left_drawer_width: 320.0,
                ..MainPageDrawerHostState::default()
            };

            assert_eq!(state.inline_left_width(), 320.0);
        });

        spec.it("hides inline left width when drawer is closed", |_| {
            let state = MainPageDrawerHostState::default();
            assert_eq!(state.inline_left_width(), 0.0);
        });

        spec.it("shows overlay for left drawer when not inline and click-away is enabled", |_| {
            let state = MainPageDrawerHostState {
                is_left_open: true,
                inline_left: false,
                left_close_on_click_away: true,
                ..MainPageDrawerHostState::default()
            };

            assert!(state.overlay_visible());
        });

        spec.it("shows overlay for right drawer when click-away is enabled", |_| {
            let state = MainPageDrawerHostState {
                is_right_open: true,
                right_close_on_click_away: true,
                ..MainPageDrawerHostState::default()
            };

            assert!(state.overlay_visible());
        });

        spec.it("hides overlay when click-away is disabled for both drawers", |_| {
            let state = MainPageDrawerHostState {
                is_left_open: true,
                is_right_open: true,
                left_close_on_click_away: false,
                right_close_on_click_away: false,
                ..MainPageDrawerHostState::default()
            };

            assert!(!state.overlay_visible());
        });
    });

    let report = crate::main_page_drawer_host::run_suite(&suite);
    assert!(report.is_success());
}
