use egui::Ui;
use egui::vec2;
use egui_material3::MaterialIconButton;

use crate::i18n::t;
use crate::ui_style::typography;
use crate::ui_style::typography::TypographyRole;
use crate::ui_icons;
use crate::ui_icons::UiIconKey;

use super::actions::NavBarAction;
use super::hamburger_toggle_button;
use super::state::InteractionMode;
use super::state::NavBarState;
use super::state::all_modes;

#[must_use]
pub fn nav_button(state: &NavBarState) -> (&'static str, NavBarAction) {
    if state.is_nav_open {
        ("close_navigation", NavBarAction::CloseNavigation)
    } else {
        ("open_navigation", NavBarAction::ToggleNavigation)
    }
}

#[must_use]
pub fn settings_button(state: &NavBarState) -> (&'static str, NavBarAction) {
    if state.is_choreography_settings_open {
        ("close_settings", NavBarAction::CloseChoreographySettings)
    } else {
        ("open_settings", NavBarAction::ToggleChoreographySettings)
    }
}

#[must_use]
pub fn mode_label(mode: InteractionMode) -> &'static str {
    match mode {
        InteractionMode::View => "ModeView",
        InteractionMode::Move => "ModeMove",
        InteractionMode::RotateAroundCenter => "ModeRotateAroundCenter",
        InteractionMode::RotateAroundDancer => "ModeRotateAroundDancer",
        InteractionMode::Scale => "ModeScale",
        InteractionMode::LineOfSight => "ModeLineOfSight",
    }
}

#[must_use]
pub const fn mode_label_role() -> TypographyRole {
    TypographyRole::LabelLarge
}

pub fn draw(ui: &mut Ui, state: &NavBarState) -> Vec<NavBarAction> {
    let locale = "en";
    let mut actions: Vec<NavBarAction> = Vec::new();
    ui.horizontal(|ui| {
        let nav_response = hamburger_toggle_button::draw(
            ui,
            state.is_nav_open,
            true,
            &t(locale, "MainToggleNavTooltip", "MainToggleNavTooltip"),
            Some(vec2(48.0, 48.0)),
        );
        if nav_response.clicked() {
            let (_, nav_action) = nav_button(state);
            actions.push(nav_action);
        }

        ui.add_space(ui.available_width().max(0.0));

        let mut selected_mode = state.selected_mode;
        ui.add_enabled_ui(state.is_mode_selection_enabled, |ui| {
            egui::ComboBox::from_label(typography::rich_text_for_role(
                t(locale, "ModeLabel", "ModeLabel"),
                mode_label_role(),
            ))
                .selected_text(typography::rich_text_for_role(
                    mode_text(selected_mode, locale),
                    mode_label_role(),
                ))
                .show_ui(ui, |ui| {
                    for mode in all_modes() {
                        ui.selectable_value(
                            &mut selected_mode,
                            *mode,
                            typography::rich_text_for_role(mode_text(*mode, locale), mode_label_role()),
                        );
                    }
                });
        });

        if selected_mode != state.selected_mode {
            actions.push(NavBarAction::SetSelectedMode {
                mode: selected_mode,
            });
        }

        let (_, settings_action) = settings_button(state);
        let settings_icon = ui_icons::icon(UiIconKey::NavSettings);
        let settings_response = ui.add(
            MaterialIconButton::standard(settings_icon.token).svg_data(settings_icon.svg),
        );
        if settings_response.clicked() {
            actions.push(settings_action);
        }
        let _ = settings_response.on_hover_text(t(
            locale,
            "MainOpenSettingsTooltip",
            "MainOpenSettingsTooltip",
        ));

        let home_icon = ui_icons::icon(UiIconKey::FloorResetViewport);
        let home_response = ui.add(
            MaterialIconButton::standard(home_icon.token).svg_data(home_icon.svg),
        );
        if home_response.clicked() {
            actions.push(NavBarAction::ResetFloorViewport);
        }
        let _ = home_response.on_hover_text(t(locale, "MainHomeTooltip", "MainHomeTooltip"));

        let image_icon = ui_icons::icon(UiIconKey::FloorOpenSvgOverlay);
        let image_response = ui.add(
            MaterialIconButton::standard(image_icon.token).svg_data(image_icon.svg),
        );
        if image_response.clicked() {
            actions.push(NavBarAction::OpenImage);
        }
        let _ =
            image_response.on_hover_text(t(locale, "MainOpenImageTooltip", "MainOpenImageTooltip"));

        let audio_icon = ui_icons::icon(UiIconKey::AudioOpenPanel);
        let audio_response =
            ui.add(MaterialIconButton::standard(audio_icon.token).svg_data(audio_icon.svg));
        if audio_response.clicked() {
            actions.push(NavBarAction::OpenAudio);
        }
        let _ =
            audio_response.on_hover_text(t(locale, "MainOpenAudioTooltip", "MainOpenAudioTooltip"));
    });

    actions
}

fn mode_text(mode: InteractionMode, locale: &str) -> String {
    t(locale, mode_label(mode), mode_label(mode))
}
