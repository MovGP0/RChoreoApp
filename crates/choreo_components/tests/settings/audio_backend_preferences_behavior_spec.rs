use std::time::Duration;

use crate::settings;

use choreo_components::audio_player::AudioPlayerBackend;
use choreo_components::preferences::Preferences;
use choreo_models::SettingsPreferenceKeys;
use settings::Report;

#[test]
#[serial_test::serial]
fn audio_backend_preferences_behavior_spec() {
    let suite = rspec::describe("audio backend preferences behavior", (), |spec| {
        spec.it("stores selected backend in preferences", |_| {
            let context = settings::SettingsTestContext::new();

            context
                .view_model
                .borrow_mut()
                .update_audio_player_backend(AudioPlayerBackend::Awedio);

            let updated = context.wait_until(Duration::from_secs(1), || {
                context.view_model.borrow().audio_player_backend == AudioPlayerBackend::Awedio
            });
            assert!(updated);
            assert_eq!(
                context
                    .preferences
                    .get_string(SettingsPreferenceKeys::AUDIO_PLAYER_BACKEND, ""),
                AudioPlayerBackend::AWEDIO_KEY
            );
        });
    });

    let report = settings::run_suite(&suite);
    assert!(report.is_success());
}
