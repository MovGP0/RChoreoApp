use super::Report;
use super::actions::AppShellAction;
use super::effects::AppShellEffect;
use super::reducer::reduce;
use super::state::AppShellState;

#[test]
fn frame_lifecycle_spec() {
    let suite = rspec::describe("app shell frame lifecycle", (), |spec| {
        spec.it(
            "requests typography and main initialization on the first frame",
            |_| {
                let mut state = AppShellState::default();

                let effects = reduce(&mut state, AppShellAction::FrameStarted);

                assert_eq!(
                    effects,
                    vec![
                        AppShellEffect::ApplyTypography,
                        AppShellEffect::InitializeMainPage,
                    ]
                );
                assert!(state.is_typography_initialized);
                assert!(state.is_main_page_initialized);
                assert!(state.show_splash_screen);
            },
        );

        spec.it(
            "does not repeat initialization effects after the first frame",
            |_| {
                let mut state = AppShellState {
                    is_typography_initialized: true,
                    is_main_page_initialized: true,
                    ..AppShellState::default()
                };

                let effects = reduce(&mut state, AppShellAction::FrameStarted);

                assert!(effects.is_empty());
            },
        );

        spec.it(
            "dismisses the splash screen after it has been presented",
            |_| {
                let mut state = AppShellState::default();

                let effects = reduce(&mut state, AppShellAction::SplashPresented);

                assert!(!state.show_splash_screen);
                assert_eq!(effects, vec![AppShellEffect::RequestRepaint]);
            },
        );
    });

    let report = super::run_suite(&suite);
    assert!(report.is_success());
}
