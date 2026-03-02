use crate::main_page_drawer_host::Report;
use crate::main_page_drawer_host::actions::MainPageDrawerHostAction;
use crate::main_page_drawer_host::reducer::MainPageDrawerHostEffect;
use crate::main_page_drawer_host::reducer::reduce;
use crate::main_page_drawer_host::state::MainPageDrawerHostState;

#[test]
fn overlay_click_behavior_spec() {
    let suite = rspec::describe("main page drawer host overlay click", (), |spec| {
        spec.it("initializes without producing effects", |_| {
            let mut state = MainPageDrawerHostState::default();
            let effects = reduce(&mut state, MainPageDrawerHostAction::Initialize);
            assert!(effects.is_empty());
        });

        spec.it("clamps top inset to non-negative values", |_| {
            let mut state = MainPageDrawerHostState::default();
            let effects = reduce(
                &mut state,
                MainPageDrawerHostAction::SetTopInset { top_inset: -12.0 },
            );
            assert!(effects.is_empty());
            assert_eq!(state.top_inset, 0.0);
        });

        spec.it("requests close and closes both drawers when click-away is enabled", |_| {
            let mut state = MainPageDrawerHostState {
                is_left_open: true,
                is_right_open: true,
                left_close_on_click_away: true,
                right_close_on_click_away: true,
                ..MainPageDrawerHostState::default()
            };

            let effects = reduce(&mut state, MainPageDrawerHostAction::OverlayClicked);

            assert!(!state.is_left_open);
            assert!(!state.is_right_open);
            assert_eq!(
                effects,
                vec![
                    MainPageDrawerHostEffect::LeftCloseRequested,
                    MainPageDrawerHostEffect::RightCloseRequested,
                    MainPageDrawerHostEffect::OverlayClicked,
                ]
            );
        });

        spec.it("keeps drawers open when click-away is disabled", |_| {
            let mut state = MainPageDrawerHostState {
                is_left_open: true,
                is_right_open: true,
                left_close_on_click_away: false,
                right_close_on_click_away: false,
                ..MainPageDrawerHostState::default()
            };

            let effects = reduce(&mut state, MainPageDrawerHostAction::OverlayClicked);

            assert!(state.is_left_open);
            assert!(state.is_right_open);
            assert_eq!(effects, vec![MainPageDrawerHostEffect::OverlayClicked]);
        });
    });

    let report = crate::main_page_drawer_host::run_suite(&suite);
    assert!(report.is_success());
}
