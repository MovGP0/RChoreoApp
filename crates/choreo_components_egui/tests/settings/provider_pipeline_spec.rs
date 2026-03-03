use crate::settings::Report;
use crate::settings::provider::SettingsProvider;
use crate::settings::state::AudioPlayerBackend;

#[test]
fn provider_pipeline_spec() {
    let suite = rspec::describe("settings provider pipeline", (), |spec| {
        spec.it("wires view model commands through reducer actions", |_| {
            let provider = SettingsProvider::new();
            let view_model = provider.settings_view_model();

            {
                let mut view_model = view_model.borrow_mut();
                view_model.update_use_primary_color(true);
                view_model.update_primary_color_hex("#FF010203".to_string());
                view_model.update_audio_player_backend(AudioPlayerBackend::Awedio);
                view_model.navigate_back();
            }

            let view_model = view_model.borrow();
            assert!(view_model.state.use_primary_color);
            assert_eq!(view_model.state.primary_color_hex, "#FF010203");
            assert_eq!(view_model.state.audio_player_backend, AudioPlayerBackend::Awedio);
        });
    });

    let report = crate::settings::run_suite(&suite);
    assert!(report.is_success());
}
