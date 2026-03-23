use egui::Layout;
use egui::Ui;
use egui::vec2;

use crate::choreo_main::actions::ChoreoMainAction;
use crate::choreo_main::state::ChoreoMainState;
use crate::choreo_main::state::InteractionMode;
use crate::hamburger_toggle_button;
use crate::material::components;
use crate::material::icons as ui_icons;
use crate::material::icons::UiIconKey;
use crate::material::styling::material_typography::TypographyRole;
use crate::nav_bar::translations::mode_text;
use crate::nav_bar::translations::nav_bar_translations;

use super::layout::GRID_12_PX;
use super::layout::TOP_BAR_HEIGHT_PX;
use super::mappings::top_bar_nav_action;
use super::mappings::top_bar_open_audio_action;
use super::mappings::top_bar_settings_action;

const DEFAULT_LOCALE: &str = "en";
const MODE_SELECTOR_WIDTH_PX: f32 = 180.0;
const MODE_SELECTOR_HEIGHT_PX: f32 = 56.0;

pub(super) fn draw_top_bar(
    ui: &mut Ui,
    state: &ChoreoMainState,
    actions: &mut Vec<ChoreoMainAction>,
) {
    let strings = nav_bar_translations(DEFAULT_LOCALE);
    ui.spacing_mut().item_spacing = vec2(GRID_12_PX, GRID_12_PX);
    ui.with_layout(Layout::left_to_right(egui::Align::Center), |ui| {
        ui.set_min_height(TOP_BAR_HEIGHT_PX);
        ui.add_space(8.0);
        let nav_response = hamburger_toggle_button::draw(
            ui,
            state.is_nav_open,
            true,
            strings.toggle_navigation_tooltip.as_str(),
            Some(vec2(48.0, 48.0)),
        );
        if nav_response.clicked() {
            actions.push(top_bar_nav_action(state.is_nav_open));
        }
        ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
            ui.add_space(8.0);

            let open_audio_response = components::top_bar_icon_button(
                ui,
                components::icon_image(components::TopBarIcon::Audio),
                false,
            );
            if open_audio_response.clicked() {
                actions.push(top_bar_open_audio_action());
            }
            let _ = open_audio_response.on_hover_text(strings.open_audio_tooltip.as_str());

            let open_image_response = components::top_bar_icon_button(
                ui,
                components::icon_image(components::TopBarIcon::Image),
                state.floor_state.svg_path.is_some(),
            );
            if open_image_response.clicked() {
                actions.push(ChoreoMainAction::RequestOpenImage {
                    file_path: String::new(),
                });
            }
            let _ = open_image_response.on_hover_text(strings.open_image_tooltip.as_str());

            let home_response = components::top_bar_icon_button(
                ui,
                components::icon_image(components::TopBarIcon::Home),
                false,
            );
            if home_response.clicked() {
                actions.push(ChoreoMainAction::ResetFloorViewport);
            }
            let _ = home_response.on_hover_text(strings.reset_floor_viewport_tooltip.as_str());

            let settings_response = components::top_bar_icon_button(
                ui,
                components::icon_image(components::TopBarIcon::Settings),
                state.is_choreography_settings_open,
            );
            if settings_response.clicked() {
                actions.push(top_bar_settings_action(state.is_choreography_settings_open));
            }
            let _ = settings_response.on_hover_text(strings.open_settings_tooltip.as_str());

            let previous_mode_index =
                effective_mode_index(state).clamp(0, mode_count() - 1) as usize;
            let selected_mode_index = components::mode_dropdown(
                ui,
                egui::Id::new("main_page_mode_dropdown"),
                Some(previous_mode_index),
                &translated_mode_labels(&strings),
                state.is_mode_selection_enabled,
                MODE_SELECTOR_WIDTH_PX,
                MODE_SELECTOR_HEIGHT_PX,
            );
            if let Some(selected_mode_index) = selected_mode_index
                && selected_mode_index != previous_mode_index
            {
                actions.push(ChoreoMainAction::SelectMode {
                    index: selected_mode_index as i32,
                });
            }
        });
    });
}

#[must_use]
pub fn nav_icon_name(is_nav_open: bool) -> &'static str {
    nav_icon_spec(is_nav_open).token
}

#[must_use]
pub fn top_bar_settings_icon_name() -> &'static str {
    top_bar_settings_icon_spec().token
}

#[must_use]
pub fn home_icon_name() -> &'static str {
    home_icon_spec().token
}

#[must_use]
pub fn open_image_icon_name() -> &'static str {
    open_image_icon_spec().token
}

#[must_use]
pub fn open_audio_icon_name() -> &'static str {
    open_audio_icon_spec().token
}

#[must_use]
pub fn nav_icon_svg(is_nav_open: bool) -> &'static str {
    if is_nav_open {
        include_str!("../../assets/icons/Close.svg")
    } else {
        include_str!("../../assets/icons/Menu.svg")
    }
}

#[must_use]
pub fn top_bar_settings_icon_svg() -> &'static str {
    include_str!("../../assets/icons/Pen.svg")
}

#[must_use]
pub fn home_icon_svg() -> &'static str {
    include_str!("../../assets/icons/Home.svg")
}

#[must_use]
pub fn open_image_icon_svg() -> &'static str {
    include_str!("../../assets/icons/Svg.svg")
}

#[must_use]
pub fn open_audio_icon_svg() -> &'static str {
    include_str!("../../assets/icons/PlayCircle.svg")
}

#[must_use]
pub fn mode_label(mode_index: i32) -> &'static str {
    match mode_index {
        0 => "View",
        1 => "Move",
        2 => "Rotate around center",
        3 => "Rotate around dancer",
        4 => "Scale",
        5 => "Line of sight",
        _ => "Mode",
    }
}

#[must_use]
pub fn mode_count() -> i32 {
    6
}

#[must_use]
pub const fn mode_label_role() -> TypographyRole {
    TypographyRole::LabelLarge
}

#[must_use]
pub fn top_bar_action_count() -> usize {
    6
}

#[must_use]
pub fn top_bar_action_icon_tokens(is_nav_open: bool) -> [&'static str; 5] {
    [
        nav_icon_name(is_nav_open),
        top_bar_settings_icon_name(),
        home_icon_name(),
        open_image_icon_name(),
        open_audio_icon_name(),
    ]
}

#[must_use]
pub const fn top_bar_action_icon_uris() -> [&'static str; 4] {
    [
        components::icon_uri(components::TopBarIcon::Settings),
        components::icon_uri(components::TopBarIcon::Home),
        components::icon_uri(components::TopBarIcon::Image),
        components::icon_uri(components::TopBarIcon::Audio),
    ]
}

#[must_use]
pub fn translated_mode_labels(
    strings: &crate::nav_bar::translations::NavBarTranslations,
) -> [&str; 6] {
    [
        mode_text(strings, crate::nav_bar::state::InteractionMode::View),
        mode_text(strings, crate::nav_bar::state::InteractionMode::Move),
        mode_text(
            strings,
            crate::nav_bar::state::InteractionMode::RotateAroundCenter,
        ),
        mode_text(
            strings,
            crate::nav_bar::state::InteractionMode::RotateAroundDancer,
        ),
        mode_text(strings, crate::nav_bar::state::InteractionMode::Scale),
        mode_text(strings, crate::nav_bar::state::InteractionMode::LineOfSight),
    ]
}

fn nav_icon_spec(is_nav_open: bool) -> ui_icons::UiIconSpec {
    if is_nav_open {
        ui_icons::icon(UiIconKey::NavClose)
    } else {
        ui_icons::icon(UiIconKey::NavOpen)
    }
}

fn top_bar_settings_icon_spec() -> ui_icons::UiIconSpec {
    ui_icons::icon(UiIconKey::NavSettings)
}

fn home_icon_spec() -> ui_icons::UiIconSpec {
    ui_icons::icon(UiIconKey::FloorResetViewport)
}

fn open_image_icon_spec() -> ui_icons::UiIconSpec {
    ui_icons::icon(UiIconKey::FloorOpenSvgOverlay)
}

fn open_audio_icon_spec() -> ui_icons::UiIconSpec {
    ui_icons::icon(UiIconKey::AudioOpenPanel)
}

fn effective_mode_index(state: &ChoreoMainState) -> i32 {
    if state.selected_mode_index >= 0 {
        return state.selected_mode_index;
    }

    match state.interaction_mode {
        InteractionMode::View => 0,
        InteractionMode::Move => 1,
        InteractionMode::RotateAroundCenter => 2,
        InteractionMode::RotateAroundDancer => 3,
        InteractionMode::Scale => 4,
        InteractionMode::LineOfSight => 5,
    }
}
