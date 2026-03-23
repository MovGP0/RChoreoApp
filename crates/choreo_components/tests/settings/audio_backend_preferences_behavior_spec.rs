use crate::settings::Report;
use crate::settings::actions::SettingsAction;
use crate::settings::reducer::reduce;
use crate::settings::state::AUDIO_PLAYER_BACKEND_KEY;
use crate::settings::state::AudioPlayerBackend;
use crate::settings::state::SettingsState;

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

            assert_eq!(state.audio_player_backend, AudioPlayerBackend::Awedio);
            assert_eq!(
                state
                    .preferences
                    .get(AUDIO_PLAYER_BACKEND_KEY)
                    .map(String::as_str),
                Some("awedio")
            );
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

                if cfg!(target_arch = "wasm32") {
                    assert_eq!(state.audio_player_backend, AudioPlayerBackend::Browser);
                    assert_eq!(
                        state
                            .preferences
                            .get(AUDIO_PLAYER_BACKEND_KEY)
                            .map(String::as_str),
                        Some("browser")
                    );
                } else {
                    assert_eq!(state.audio_player_backend, AudioPlayerBackend::Rodio);
                    assert_eq!(
                        state
                            .preferences
                            .get(AUDIO_PLAYER_BACKEND_KEY)
                            .map(String::as_str),
                        Some("rodio")
                    );
                }
            },
        );
    });

    let report = crate::settings::run_suite(&suite);
    assert!(report.is_success());
}
