use crate::settings::Report;
use crate::settings::actions::SettingsAction;
use crate::settings::reducer::reduce;
use crate::settings::state::AUDIO_PLAYER_BACKEND_KEY;
use crate::settings::state::AudioPlayerBackend;
use crate::settings::state::SettingsState;

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

fn assert_no_errors(errors: Vec<String>) {
    assert!(
        errors.is_empty(),
        "Assertion failures:\n{}",
        errors.join("\n")
    );
}

#[test]
fn audio_backend_preferences_behavior_spec() {
    let suite = rspec::describe("audio backend preferences reducer behavior", (), |spec| {
        spec.it("stores selected backend in preferences", |_| {
            let mut state = SettingsState::default();
            reduce(&mut state, SettingsAction::Initialize);
            reduce(
                &mut state,
                SettingsAction::UpdateAudioPlayerBackend {
                    backend: AudioPlayerBackend::Awedio,
                },
            );

            let mut errors = Vec::new();

            check_eq!(errors, state.audio_player_backend, AudioPlayerBackend::Awedio);
            check_eq!(
                errors,
                state
                    .preferences
                    .get(AUDIO_PLAYER_BACKEND_KEY)
                    .map(String::as_str),
                Some("awedio")
            );

            assert_no_errors(errors);
        });

        spec.it(
            "normalizes selected backend to the current target capabilities",
            |_| {
                let mut state = SettingsState::default();
                reduce(&mut state, SettingsAction::Initialize);
                reduce(
                    &mut state,
                    SettingsAction::UpdateAudioPlayerBackend {
                        backend: AudioPlayerBackend::Browser,
                    },
                );

                let mut errors = Vec::new();

                if cfg!(target_arch = "wasm32") {
                    check_eq!(errors, state.audio_player_backend, AudioPlayerBackend::Browser);
                    check_eq!(
                        errors,
                        state
                            .preferences
                            .get(AUDIO_PLAYER_BACKEND_KEY)
                            .map(String::as_str),
                        Some("browser")
                    );
                } else {
                    check_eq!(errors, state.audio_player_backend, AudioPlayerBackend::Rodio);
                    check_eq!(
                        errors,
                        state
                            .preferences
                            .get(AUDIO_PLAYER_BACKEND_KEY)
                            .map(String::as_str),
                        Some("rodio")
                    );
                }

                assert_no_errors(errors);
            },
        );
    });

    let report = crate::settings::run_suite(&suite);
    assert!(report.is_success());
}
