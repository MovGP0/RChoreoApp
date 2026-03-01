use crate::settings::state::AudioPlayerBackend;
use crate::settings::state::SettingsState;
use crate::settings::ui::audio_backend_label;
use crate::settings::ui::parse_argb_hex;

#[test]
fn settings_ui_draw_executes_without_panicking() {
    let state = SettingsState::default();
    let context = egui::Context::default();
    let _ = context.run(egui::RawInput::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let _ = crate::settings::ui::draw(ui, &state);
        });
    });
}

#[test]
fn audio_backend_labels_match_expected_values() {
    assert_eq!(audio_backend_label(AudioPlayerBackend::Rodio), "Rodio");
    assert_eq!(audio_backend_label(AudioPlayerBackend::Awedio), "Awedio");
}

#[test]
fn parse_argb_hex_handles_valid_and_invalid_values() {
    assert!(parse_argb_hex("#FF112233").is_some());
    assert!(parse_argb_hex("#112233").is_none());
    assert!(parse_argb_hex("#GG112233").is_none());
}
