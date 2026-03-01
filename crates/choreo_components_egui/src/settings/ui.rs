use egui::Color32;
use egui::Ui;

use super::actions::SettingsAction;
use super::state::AudioPlayerBackend;
use super::state::SettingsState;
use super::state::ThemeMode;

#[must_use]
pub fn draw(ui: &mut Ui, state: &SettingsState) -> Vec<SettingsAction> {
    let mut actions: Vec<SettingsAction> = Vec::new();

    ui.spacing_mut().item_spacing = egui::vec2(12.0, 12.0);
    ui.heading("Settings");

    ui.group(|ui| {
        ui.set_min_width(300.0);
        ui.label("Theme");

        let mut use_system_theme = state.use_system_theme;
        if ui
            .checkbox(&mut use_system_theme, "Use system theme")
            .changed()
        {
            actions.push(SettingsAction::UpdateUseSystemTheme {
                enabled: use_system_theme,
            });
        }

        let mut is_dark_mode = matches!(state.theme_mode, ThemeMode::Dark);
        ui.add_enabled_ui(!use_system_theme, |ui| {
            if ui.checkbox(&mut is_dark_mode, "Dark mode").changed() {
                actions.push(SettingsAction::UpdateIsDarkMode {
                    enabled: is_dark_mode,
                });
            }
        });
    });

    ui.group(|ui| {
        ui.set_min_width(300.0);
        ui.label("Audio backend");

        let mut selected_backend = state.audio_player_backend;
        egui::ComboBox::from_label("Backend")
            .selected_text(audio_backend_label(selected_backend))
            .show_ui(ui, |ui| {
                for backend in [AudioPlayerBackend::Rodio, AudioPlayerBackend::Awedio] {
                    ui.selectable_value(&mut selected_backend, backend, audio_backend_label(backend));
                }
            });

        if selected_backend != state.audio_player_backend {
            actions.push(SettingsAction::UpdateAudioPlayerBackend {
                backend: selected_backend,
            });
        }
    });

    ui.group(|ui| {
        ui.set_min_width(300.0);
        ui.label("Colors");

        draw_color_controls(
            ui,
            ColorControlSpec {
                label: "Primary color",
                enabled: state.use_primary_color,
                can_enable: true,
                color_hex: &state.primary_color_hex,
                toggle_action: update_use_primary_color_action,
                color_action: update_primary_color_hex_action,
            },
            &mut actions,
        );

        draw_color_controls(
            ui,
            ColorControlSpec {
                label: "Secondary color",
                enabled: state.use_secondary_color,
                can_enable: state.use_primary_color,
                color_hex: &state.secondary_color_hex,
                toggle_action: update_use_secondary_color_action,
                color_action: update_secondary_color_hex_action,
            },
            &mut actions,
        );

        draw_color_controls(
            ui,
            ColorControlSpec {
                label: "Tertiary color",
                enabled: state.use_tertiary_color,
                can_enable: state.use_secondary_color,
                color_hex: &state.tertiary_color_hex,
                toggle_action: update_use_tertiary_color_action,
                color_action: update_tertiary_color_hex_action,
            },
            &mut actions,
        );
    });

    actions
}

struct ColorControlSpec<'a> {
    label: &'a str,
    enabled: bool,
    can_enable: bool,
    color_hex: &'a str,
    toggle_action: fn(bool) -> SettingsAction,
    color_action: fn(String) -> SettingsAction,
}

fn update_use_primary_color_action(enabled: bool) -> SettingsAction {
    SettingsAction::UpdateUsePrimaryColor { enabled }
}

fn update_primary_color_hex_action(value: String) -> SettingsAction {
    SettingsAction::UpdatePrimaryColorHex { value }
}

fn update_use_secondary_color_action(enabled: bool) -> SettingsAction {
    SettingsAction::UpdateUseSecondaryColor { enabled }
}

fn update_secondary_color_hex_action(value: String) -> SettingsAction {
    SettingsAction::UpdateSecondaryColorHex { value }
}

fn update_use_tertiary_color_action(enabled: bool) -> SettingsAction {
    SettingsAction::UpdateUseTertiaryColor { enabled }
}

fn update_tertiary_color_hex_action(value: String) -> SettingsAction {
    SettingsAction::UpdateTertiaryColorHex { value }
}

fn draw_color_controls(ui: &mut Ui, spec: ColorControlSpec<'_>, actions: &mut Vec<SettingsAction>) {
    let mut use_color = spec.enabled;
    ui.add_enabled_ui(spec.can_enable, |ui| {
        if ui.checkbox(&mut use_color, spec.label).changed() {
            actions.push((spec.toggle_action)(use_color));
        }
    });

    ui.horizontal(|ui| {
        if let Some(color) = parse_argb_hex(spec.color_hex) {
            ui.colored_label(color, "■");
        } else {
            ui.label("■");
        }

        let mut edit_value = spec.color_hex.to_string();
        let response = ui.add_enabled(
            spec.enabled,
            egui::TextEdit::singleline(&mut edit_value)
                .hint_text("#AARRGGBB")
                .desired_width(156.0),
        );
        if response.changed() {
            actions.push((spec.color_action)(edit_value));
        }
    });
}

#[must_use]
pub fn audio_backend_label(backend: AudioPlayerBackend) -> &'static str {
    match backend {
        AudioPlayerBackend::Rodio => "Rodio",
        AudioPlayerBackend::Awedio => "Awedio",
    }
}

#[must_use]
pub fn parse_argb_hex(value: &str) -> Option<Color32> {
    let trimmed = value.trim();
    if trimmed.len() != 9 || !trimmed.starts_with('#') {
        return None;
    }

    let alpha = u8::from_str_radix(&trimmed[1..3], 16).ok()?;
    let red = u8::from_str_radix(&trimmed[3..5], 16).ok()?;
    let green = u8::from_str_radix(&trimmed[5..7], 16).ok()?;
    let blue = u8::from_str_radix(&trimmed[7..9], 16).ok()?;
    Some(Color32::from_rgba_unmultiplied(red, green, blue, alpha))
}
