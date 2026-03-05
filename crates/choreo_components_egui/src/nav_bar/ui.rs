use egui::Ui;
use egui::vec2;
use egui_material3::MaterialIconButton;
use egui_material3::MaterialSelect;

use crate::ui_icons;
use crate::ui_icons::UiIconKey;
use crate::ui_style::typography::TypographyRole;

use super::actions::NavBarAction;
use super::hamburger_toggle_button;
use super::state::InteractionMode;
use super::state::NavBarState;
use super::state::all_modes;
use super::translations::mode_text;
use super::translations::nav_bar_translations;

const MODE_SELECTOR_WIDTH_PX: f32 = 180.0;
const MODE_SELECTOR_HEIGHT_PX: f32 = 56.0;

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

#[must_use]
pub const fn mode_selector_width_token() -> f32 {
    MODE_SELECTOR_WIDTH_PX
}

#[must_use]
pub const fn mode_selector_height_token() -> f32 {
    MODE_SELECTOR_HEIGHT_PX
}

#[must_use]
pub fn settings_button_checked(state: &NavBarState) -> bool {
    state.is_choreography_settings_open
}

#[must_use]
pub fn image_button_checked(state: &NavBarState) -> bool {
    state.is_floor_svg_overlay_open
}

pub fn draw(ui: &mut Ui, state: &NavBarState) -> Vec<NavBarAction> {
    let locale = "en";
    let strings = nav_bar_translations(locale);
    let mut actions: Vec<NavBarAction> = Vec::new();
    ui.horizontal(|ui| {
        let nav_response = hamburger_toggle_button::draw(
            ui,
            state.is_nav_open,
            true,
            strings.toggle_navigation_tooltip.as_str(),
            Some(vec2(48.0, 48.0)),
        );
        if nav_response.clicked() {
            let (_, nav_action) = nav_button(state);
            actions.push(nav_action);
        }

        ui.add_space(ui.available_width().max(0.0));

        let previous_mode_index = all_modes()
            .iter()
            .position(|mode| *mode == state.selected_mode)
            .unwrap_or(0);
        let mut selected_mode_index = Some(previous_mode_index);
        let mut mode_changed = false;
        ui.add_enabled_ui(state.is_mode_selection_enabled, |ui| {
            ui.set_min_height(mode_selector_height_token());
            ui.set_width(mode_selector_width_token());
            let mut mode_select = MaterialSelect::new(&mut selected_mode_index)
                .width(mode_selector_width_token())
                .placeholder(strings.mode_label.clone());
            for (index, mode) in all_modes().iter().enumerate() {
                mode_select = mode_select.option(index, mode_text(&strings, *mode));
            }
            mode_changed = ui.add(mode_select).changed();
        });

        if mode_changed
            && let Some(selected_mode_index) = selected_mode_index
            && selected_mode_index != previous_mode_index
            && let Some(mode) = all_modes().get(selected_mode_index)
        {
            actions.push(NavBarAction::SetSelectedMode { mode: *mode });
        }

        let (_, settings_action) = settings_button(state);
        let settings_icon = ui_icons::icon(UiIconKey::NavSettings);
        let mut settings_checked = settings_button_checked(state);
        let settings_response = ui.add(
            MaterialIconButton::toggle(settings_icon.token, &mut settings_checked)
                .svg_data(settings_icon.svg),
        );
        if settings_response.clicked() {
            actions.push(settings_action);
        }
        let _ = settings_response.on_hover_text(strings.open_settings_tooltip.as_str());

        let home_icon = ui_icons::icon(UiIconKey::FloorResetViewport);
        let home_response =
            ui.add(MaterialIconButton::standard(home_icon.token).svg_data(home_icon.svg));
        if home_response.clicked() {
            actions.push(NavBarAction::ResetFloorViewport);
        }
        let _ = home_response.on_hover_text(strings.reset_floor_viewport_tooltip.as_str());

        let image_icon = ui_icons::icon(UiIconKey::FloorOpenSvgOverlay);
        let mut image_checked = image_button_checked(state);
        let image_response = ui.add(
            MaterialIconButton::toggle(image_icon.token, &mut image_checked)
                .svg_data(image_icon.svg),
        );
        if image_response.clicked() {
            actions.push(NavBarAction::OpenImage);
        }
        let _ = image_response.on_hover_text(strings.open_image_tooltip.as_str());

        let audio_icon = ui_icons::icon(UiIconKey::AudioOpenPanel);
        let audio_response =
            ui.add(MaterialIconButton::standard(audio_icon.token).svg_data(audio_icon.svg));
        if audio_response.clicked() {
            actions.push(NavBarAction::OpenAudio);
        }
        let _ = audio_response.on_hover_text(strings.open_audio_tooltip.as_str());
    });

    actions
}
