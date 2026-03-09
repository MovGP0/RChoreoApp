use egui::Layout;
use egui::Ui;
use egui::vec2;

use crate::material::components;
use crate::material::styling::material_typography::TypographyRole;

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
pub fn action_button_tokens(state: &NavBarState) -> [&'static str; 5] {
    let nav_token = if state.is_nav_open { "close" } else { "menu" };
    [nav_token, "edit", "home", "image", "play_circle"]
}

#[must_use]
pub const fn top_bar_action_count() -> usize {
    6
}

#[must_use]
pub fn mode_option_labels(strings: &super::translations::NavBarTranslations) -> [&str; 6] {
    [
        mode_text(strings, InteractionMode::View),
        mode_text(strings, InteractionMode::Move),
        mode_text(strings, InteractionMode::RotateAroundCenter),
        mode_text(strings, InteractionMode::RotateAroundDancer),
        mode_text(strings, InteractionMode::Scale),
        mode_text(strings, InteractionMode::LineOfSight),
    ]
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
    ui.spacing_mut().item_spacing = vec2(12.0, 12.0);
    ui.with_layout(Layout::left_to_right(egui::Align::Center), |ui| {
        ui.set_min_height(MODE_SELECTOR_HEIGHT_PX);
        ui.add_space(8.0);
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

        let previous_mode_index = all_modes()
            .iter()
            .position(|mode| *mode == state.selected_mode)
            .unwrap_or(0);
        ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
            ui.add_space(8.0);

            let audio_response = components::top_bar_icon_button(
                ui,
                components::icon_image(components::TopBarIcon::Audio),
                false,
            );
            if audio_response.clicked() {
                actions.push(NavBarAction::OpenAudio);
            }
            let _ = audio_response.on_hover_text(strings.open_audio_tooltip.as_str());

            let image_response = components::top_bar_icon_button(
                ui,
                components::icon_image(components::TopBarIcon::Image),
                image_button_checked(state),
            );
            if image_response.clicked() {
                actions.push(NavBarAction::OpenImage);
            }
            let _ = image_response.on_hover_text(strings.open_image_tooltip.as_str());

            let home_response = components::top_bar_icon_button(
                ui,
                components::icon_image(components::TopBarIcon::Home),
                false,
            );
            if home_response.clicked() {
                actions.push(NavBarAction::ResetFloorViewport);
            }
            let _ = home_response.on_hover_text(strings.reset_floor_viewport_tooltip.as_str());

            let (_, settings_action) = settings_button(state);
            let settings_response = components::top_bar_icon_button(
                ui,
                components::icon_image(components::TopBarIcon::Settings),
                settings_button_checked(state),
            );
            if settings_response.clicked() {
                actions.push(settings_action);
            }
            let _ = settings_response.on_hover_text(strings.open_settings_tooltip.as_str());

            let selected_mode_index = components::mode_dropdown(
                ui,
                egui::Id::new("nav_bar_mode_dropdown"),
                Some(previous_mode_index),
                &mode_option_labels(&strings),
                state.is_mode_selection_enabled,
                mode_selector_width_token(),
                mode_selector_height_token(),
            );
            if let Some(selected_mode_index) = selected_mode_index
                && selected_mode_index != previous_mode_index
                && let Some(mode) = all_modes().get(selected_mode_index)
            {
                actions.push(NavBarAction::SetSelectedMode { mode: *mode });
            }
        });
    });

    actions
}

#[must_use]
pub const fn action_button_icon_uris() -> [&'static str; 4] {
    [
        components::icon_uri(components::TopBarIcon::Settings),
        components::icon_uri(components::TopBarIcon::Home),
        components::icon_uri(components::TopBarIcon::Image),
        components::icon_uri(components::TopBarIcon::Audio),
    ]
}
