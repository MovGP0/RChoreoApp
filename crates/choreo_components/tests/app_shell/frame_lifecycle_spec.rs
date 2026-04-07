use super::Report;
use super::actions::AppShellAction;
use super::effects::AppShellEffect;
use super::reducer::reduce;
use super::state::AppShellState;

macro_rules! check_eq {
    ($errors:expr, $left:expr, $right:expr) => {
        if $left != $right {
            $errors.push(format!(
                "{} != {} (left = {:?}, right = {:?})",
                stringify!($left),
                stringify!($right),
                $left,
                $right
            ));
        }
    };
}

macro_rules! check {
    ($errors:expr, $condition:expr) => {
        if !$condition {
            $errors.push(format!("condition failed: {}", stringify!($condition)));
        }
    };
}

fn assert_no_errors(errors: Vec<String>) {
    assert!(
        errors.is_empty(),
        "Assertion failures:\n{}",
        errors.join("\n")
    );
}

#[test]
fn frame_lifecycle_spec() {
    let suite = rspec::describe("app shell frame lifecycle", (), |spec| {
        spec.it(
            "requests typography and main initialization on the first frame",
            |_| {
                let mut state = AppShellState::default();

                let effects = reduce(&mut state, AppShellAction::FrameStarted);
                let mut errors = Vec::new();

                check_eq!(
                    errors,
                    effects,
                    vec![
                        AppShellEffect::ApplyTypography,
                        AppShellEffect::InitializeMainPage,
                    ]
                );
                check!(errors, state.is_typography_initialized);
                check!(errors, state.is_main_page_initialized);
                check!(errors, state.show_splash_screen);

                assert_no_errors(errors);
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
                let mut errors = Vec::new();

                check!(errors, effects.is_empty());

                assert_no_errors(errors);
            },
        );

        spec.it(
            "dismisses the splash screen after it has been presented",
            |_| {
                let mut state = AppShellState::default();

                let effects = reduce(&mut state, AppShellAction::SplashPresented);
                let mut errors = Vec::new();

                check!(errors, !state.show_splash_screen);
                check_eq!(errors, effects, vec![AppShellEffect::RequestRepaint]);

                assert_no_errors(errors);
            },
        );
    });

    let report = super::run_suite(&suite);
    assert!(report.is_success());
}
