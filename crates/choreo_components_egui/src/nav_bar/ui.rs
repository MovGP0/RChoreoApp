use egui::Ui;
use egui::vec2;
use egui_material3::MaterialButton;

use crate::i18n::t;

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
        InteractionMode::View => "View",
        InteractionMode::Move => "Move",
        InteractionMode::RotateAroundCenter => "Rotate Center",
        InteractionMode::RotateAroundDancer => "Rotate Dancer",
        InteractionMode::Scale => "Scale",
        InteractionMode::LineOfSight => "Line of Sight",
    }
}

pub fn draw(ui: &mut Ui, state: &NavBarState) -> Vec<NavBarAction> {
    let locale = "en";
    let mut actions: Vec<NavBarAction> = Vec::new();
    ui.horizontal(|ui| {
        ui.heading(t(locale, "ModeLabel", "Mode"));
        let nav_response = hamburger_toggle_button::draw(
            ui,
            state.is_nav_open,
            true,
            &t(locale, "MainToggleNavTooltip", "Toggle navigation"),
            Some(vec2(48.0, 48.0)),
        );
        if nav_response.clicked() {
            let (_, nav_action) = nav_button(state);
            actions.push(nav_action);
        }
        let (_, settings_action) = settings_button(state);
        if ui
            .add(MaterialButton::new("S"))
            .on_hover_text(t(locale, "MainOpenSettingsTooltip", "Open settings"))
            .clicked()
        {
            actions.push(settings_action);
        }
    });

    ui.horizontal_wrapped(|ui| {
        if ui
            .add(MaterialButton::new("A"))
            .on_hover_text(t(locale, "MainOpenAudioTooltip", "Open audio"))
            .clicked()
        {
            actions.push(NavBarAction::OpenAudio);
        }
        if ui
            .add(MaterialButton::new("I"))
            .on_hover_text(t(locale, "MainOpenImageTooltip", "Open floor SVG"))
            .clicked()
        {
            actions.push(NavBarAction::OpenImage);
        }
        if ui
            .add(MaterialButton::new("R"))
            .on_hover_text(t(locale, "MainHomeTooltip", "Reset viewport"))
            .clicked()
        {
            actions.push(NavBarAction::ResetFloorViewport);
        }
    });

    let mut selected_mode = state.selected_mode;
    ui.add_enabled_ui(state.is_mode_selection_enabled, |ui| {
        egui::ComboBox::from_label(t(locale, "ModeLabel", "Mode"))
            .selected_text(mode_text(selected_mode, locale))
            .show_ui(ui, |ui| {
                for mode in all_modes() {
                    ui.selectable_value(&mut selected_mode, *mode, mode_text(*mode, locale));
                }
            });
    });

    if selected_mode != state.selected_mode {
        actions.push(NavBarAction::SetSelectedMode {
            mode: selected_mode,
        });
    }

    actions
}

fn mode_text(mode: InteractionMode, locale: &str) -> String {
    match mode {
        InteractionMode::View => t(locale, "ModeView", mode_label(mode)),
        InteractionMode::Move => t(locale, "ModeMove", mode_label(mode)),
        InteractionMode::RotateAroundCenter => {
            t(locale, "ModeRotateAroundCenter", mode_label(mode))
        }
        InteractionMode::RotateAroundDancer => {
            t(locale, "ModeRotateAroundDancer", mode_label(mode))
        }
        InteractionMode::Scale => t(locale, "ModeScale", mode_label(mode)),
        InteractionMode::LineOfSight => t(locale, "ModeLineOfSight", mode_label(mode)),
    }
}
