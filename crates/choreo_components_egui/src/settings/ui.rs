use std::borrow::Cow;

use egui::Color32;
use egui::CornerRadius;
use egui::Frame;
use egui::Layout;
use egui::Margin;
use egui::ScrollArea;
use egui::Sense;
use egui::Stroke;
use egui::Ui;
use egui::vec2;

use crate::color_picker::state::ColorPickerDock;
use crate::color_picker::state::ColorPickerState;
use crate::color_picker::ui as color_picker_ui;
use crate::material::components;
use crate::material::icons as ui_icons;
use crate::material::icons::UiIconKey;
use crate::material::styling::material_palette::material_palette_for_visuals;
use crate::material::styling::material_style_metrics::material_style_metrics;
use crate::material::styling::material_typography as typography;
use crate::material::styling::material_typography::TypographyRole;

use super::actions::SettingsAction;
use super::state::AudioPlayerBackend;
use super::state::SettingsState;
use super::state::ThemeMode;
use super::translations::settings_translations;

const DEFAULT_LOCALE: &str = "en";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ThemeSelection {
    UseSystemTheme,
    DarkMode,
    LightMode,
}

struct ColorCardSpec<'a> {
    header: &'a str,
    switch_enabled: bool,
    enabled: bool,
    color_hex: &'a str,
    toggle_action: fn(bool) -> SettingsAction,
    color_action: fn(String) -> SettingsAction,
}

#[must_use]
pub fn draw(ui: &mut Ui, state: &SettingsState) -> Vec<SettingsAction> {
    let mut actions: Vec<SettingsAction> = Vec::new();
    let strings = settings_translations(DEFAULT_LOCALE);

    ui.spacing_mut().item_spacing = vec2(content_spacing_token(), content_spacing_token());

    let content_width = ui.available_width().min(content_max_width_token());

    ScrollArea::vertical()
        .id_salt("settings_page_scroll")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            draw_centered_content_column(ui, content_width, |ui| {
                draw_header(
                    ui,
                    strings.navigate_back.as_str(),
                    &strings.title,
                    &mut actions,
                );
                ui.add_space(card_spacing_token());

                draw_theme_card(ui, state, &strings, &mut actions);
                ui.add_space(card_spacing_token());

                draw_color_card(
                    ui,
                    ColorCardSpec {
                        header: strings.primary_color.as_str(),
                        switch_enabled: true,
                        enabled: state.use_primary_color,
                        color_hex: &state.primary_color_hex,
                        toggle_action: update_use_primary_color_action,
                        color_action: update_primary_color_hex_action,
                    },
                    &mut actions,
                );
                ui.add_space(card_spacing_token());

                draw_color_card(
                    ui,
                    ColorCardSpec {
                        header: strings.secondary_color.as_str(),
                        switch_enabled: state.use_primary_color,
                        enabled: state.use_secondary_color,
                        color_hex: &state.secondary_color_hex,
                        toggle_action: update_use_secondary_color_action,
                        color_action: update_secondary_color_hex_action,
                    },
                    &mut actions,
                );
                ui.add_space(card_spacing_token());

                draw_color_card(
                    ui,
                    ColorCardSpec {
                        header: strings.tertiary_color.as_str(),
                        switch_enabled: state.use_secondary_color,
                        enabled: state.use_tertiary_color,
                        color_hex: &state.tertiary_color_hex,
                        toggle_action: update_use_tertiary_color_action,
                        color_action: update_tertiary_color_hex_action,
                    },
                    &mut actions,
                );

                if shows_audio_backend_card() {
                    ui.add_space(card_spacing_token());
                    draw_audio_backend_card(ui, state, &strings, &mut actions);
                }
            });
        });

    actions
}

#[must_use]
pub const fn page_title_role() -> TypographyRole {
    TypographyRole::HeadlineSmall
}

#[must_use]
pub const fn content_spacing_token() -> f32 {
    material_style_metrics().spacings.spacing_12
}

#[must_use]
pub const fn content_max_width_token() -> f32 {
    720.0
}

#[must_use]
pub const fn header_row_height_token() -> f32 {
    material_style_metrics().sizes.size_40
}

#[must_use]
pub const fn section_label_role() -> TypographyRole {
    TypographyRole::TitleSmall
}

#[must_use]
pub const fn toggle_label_role() -> TypographyRole {
    TypographyRole::BodyMedium
}

#[must_use]
pub const fn card_spacing_token() -> f32 {
    material_style_metrics().spacings.spacing_12
}

#[must_use]
pub const fn card_padding_token() -> f32 {
    material_style_metrics().paddings.padding_12
}

#[must_use]
pub const fn card_corner_radius_token() -> f32 {
    material_style_metrics().corner_radii.border_radius_8
}

#[must_use]
pub const fn row_spacing_token() -> f32 {
    material_style_metrics().spacings.spacing_8
}

#[must_use]
pub const fn dropdown_height_token() -> f32 {
    material_style_metrics().sizes.size_56
}

#[must_use]
pub const fn toggle_row_height_token() -> f32 {
    material_style_metrics().sizes.size_48
}

#[must_use]
pub const fn color_swatch_width_token() -> f32 {
    108.0
}

#[must_use]
pub const fn color_swatch_height_token() -> f32 {
    36.0
}

#[must_use]
pub const fn color_picker_wheel_size_token() -> f32 {
    168.0
}

#[must_use]
pub fn navigate_back_icon_name() -> &'static str {
    ui_icons::icon(UiIconKey::SettingsNavigateBack).token
}

#[must_use]
pub fn navigate_back_icon_svg() -> &'static str {
    ui_icons::icon(UiIconKey::SettingsNavigateBack).svg
}

fn navigate_back_icon_image() -> egui::Image<'static> {
    egui::Image::from_bytes(
        format!("bytes://settings/{}.svg", navigate_back_icon_name()),
        navigate_back_icon_svg().as_bytes(),
    )
}

#[must_use]
pub fn available_audio_backends_for_current_target() -> Vec<AudioPlayerBackend> {
    AudioPlayerBackend::available_for_current_target().to_vec()
}

#[must_use]
pub fn shows_audio_backend_card() -> bool {
    AudioPlayerBackend::available_for_current_target().len() > 1
}

#[must_use]
pub fn visible_settings_card_headers(
    state: &SettingsState,
    strings: &super::translations::SettingsTranslations,
) -> Vec<String> {
    let mut headers = vec![
        strings.theme.clone(),
        strings.primary_color.clone(),
        strings.secondary_color.clone(),
        strings.tertiary_color.clone(),
    ];

    if shows_audio_backend_card() {
        headers.push(strings.audio_backend.clone());
    }

    let _ = state;
    headers
}

#[must_use]
pub fn theme_mode_dropdown_labels(
    state: &SettingsState,
    strings: &super::translations::SettingsTranslations,
) -> Vec<String> {
    let mut labels = Vec::new();
    if state.can_use_system_theme {
        labels.push(strings.use_system_theme.clone());
    }
    labels.push(strings.dark_mode.clone());
    labels.push(strings.light_mode.clone());
    labels
}

#[must_use]
pub fn selected_theme_mode_dropdown_index(state: &SettingsState) -> usize {
    match selected_theme_selection(state) {
        ThemeSelection::UseSystemTheme => 0,
        ThemeSelection::DarkMode if state.can_use_system_theme => 1,
        ThemeSelection::DarkMode => 0,
        ThemeSelection::LightMode if state.can_use_system_theme => 2,
        ThemeSelection::LightMode => 1,
    }
}

fn update_use_system_theme_action(enabled: bool) -> SettingsAction {
    SettingsAction::UpdateUseSystemTheme { enabled }
}

fn update_is_dark_mode_action(enabled: bool) -> SettingsAction {
    SettingsAction::UpdateIsDarkMode { enabled }
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

fn draw_header(
    ui: &mut Ui,
    navigate_back_text: &str,
    title: &str,
    actions: &mut Vec<SettingsAction>,
) {
    let width = ui.available_width();
    ui.allocate_ui_with_layout(
        vec2(width, header_row_height_token()),
        Layout::left_to_right(egui::Align::Center).with_cross_align(egui::Align::Center),
        |ui| {
            ui.spacing_mut().item_spacing.x = row_spacing_token();
            let palette = material_palette_for_visuals(ui.visuals());
            ui.allocate_ui_with_layout(
                vec2(header_row_height_token(), header_row_height_token()),
                Layout::left_to_right(egui::Align::Center).with_cross_align(egui::Align::Center),
                |ui| {
                    if draw_navigate_back_button(ui, navigate_back_text)
                        .on_hover_text(navigate_back_text)
                        .clicked()
                    {
                        actions.push(SettingsAction::NavigateBack);
                    }
                },
            );

            ui.label(
                typography::rich_text_for_role(title, page_title_role())
                    .color(palette.on_background),
            );
        },
    );
}

fn draw_navigate_back_button(ui: &mut Ui, navigate_back_text: &str) -> egui::Response {
    let palette = material_palette_for_visuals(ui.visuals());
    components::BaseButton {
        icon: Some(navigate_back_icon_image()),
        icon_color: Some(palette.on_background),
        tooltip: Cow::Borrowed(navigate_back_text),
        button_horizontal_padding: 0.0,
        button_vertical_padding: 0.0,
        min_layout_width: material_style_metrics().sizes.size_40,
        min_layout_height: material_style_metrics().sizes.size_40,
        icon_size: material_style_metrics().icon_sizes.icon_size_24,
        border_radius: Some(material_style_metrics().sizes.size_40 * 0.5),
        display_background: false,
        clip_ripple: true,
        ..components::BaseButton::new()
    }
    .show(ui, |_| {})
    .response
}

fn draw_theme_card(
    ui: &mut Ui,
    state: &SettingsState,
    strings: &super::translations::SettingsTranslations,
    actions: &mut Vec<SettingsAction>,
) {
    draw_settings_card(ui, |ui| {
        draw_card_header(ui, strings.theme.as_str());
        ui.add_space(row_spacing_token());
        draw_theme_mode_dropdown(ui, state, strings, actions);
    });
}

fn draw_audio_backend_card(
    ui: &mut Ui,
    state: &SettingsState,
    strings: &super::translations::SettingsTranslations,
    actions: &mut Vec<SettingsAction>,
) {
    draw_settings_card(ui, |ui| {
        draw_card_header(ui, strings.audio_backend.as_str());
        ui.add_space(row_spacing_token());

        let options = available_audio_backends_for_current_target();
        let selected_index = audio_backend_index(state.audio_player_backend);
        let labels = options
            .iter()
            .map(|backend| audio_backend_label(*backend, strings))
            .collect::<Vec<_>>();
        let response = components::mode_dropdown(
            ui,
            egui::Id::new("settings_audio_backend_dropdown"),
            Some(selected_index),
            labels.as_slice(),
            true,
            ui.available_width(),
            dropdown_height_token(),
        );

        if let Some(next_index) = response
            && next_index != selected_index
            && let Some(backend) = options.get(next_index)
        {
            actions.push(SettingsAction::UpdateAudioPlayerBackend { backend: *backend });
        }
    });
}

fn draw_color_card(ui: &mut Ui, spec: ColorCardSpec<'_>, actions: &mut Vec<SettingsAction>) {
    draw_settings_card(ui, |ui| {
        draw_card_header(ui, spec.header);
        ui.add_space(row_spacing_token());

        draw_toggle_switch_row(
            ui,
            spec.header,
            spec.enabled,
            spec.switch_enabled,
            spec.toggle_action,
            actions,
        );

        ui.add_space(row_spacing_token());

        draw_color_picker_row(ui, spec.color_hex, spec.enabled, spec.color_action, actions);
    });
}

fn draw_settings_card(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui)) {
    let palette = material_palette_for_visuals(ui.visuals());
    let metrics = material_style_metrics();

    Frame::new()
        .fill(palette.surface_container)
        .stroke(Stroke::new(
            metrics.strokes.outline,
            palette.outline_variant,
        ))
        .corner_radius(CornerRadius::same(card_corner_radius_token().round() as u8))
        .inner_margin(Margin::same(card_padding_token().round() as i8))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.spacing_mut().item_spacing = vec2(content_spacing_token(), row_spacing_token());
            add_contents(ui);
        });
}

fn draw_card_header(ui: &mut Ui, title: &str) {
    let palette = material_palette_for_visuals(ui.visuals());
    ui.label(
        typography::rich_text_for_role(title, section_label_role()).color(palette.on_surface),
    );
}

fn draw_toggle_switch_row<F>(
    ui: &mut Ui,
    tooltip: &str,
    current_value: bool,
    enabled: bool,
    ctor: F,
    actions: &mut Vec<SettingsAction>,
) where
    F: Fn(bool) -> SettingsAction,
{
    ui.add_enabled_ui(enabled, |ui| {
        ui.allocate_ui_with_layout(
            vec2(ui.available_width(), toggle_row_height_token()),
            Layout::right_to_left(egui::Align::Center),
            |ui| {
                let response = components::Switch {
                    checked: current_value,
                    enabled,
                    tooltip: Cow::Borrowed(tooltip),
                    ..components::Switch::new()
                }
                .show(ui);
                if response.changed {
                    actions.push(ctor(response.checked));
                }
            },
        );
    });
}

fn draw_theme_mode_dropdown(
    ui: &mut Ui,
    state: &SettingsState,
    strings: &super::translations::SettingsTranslations,
    actions: &mut Vec<SettingsAction>,
) {
    let labels = theme_mode_dropdown_labels(state, strings);
    let label_refs = labels.iter().map(String::as_str).collect::<Vec<_>>();
    let selected_index = selected_theme_mode_dropdown_index(state);
    let response = components::mode_dropdown(
        ui,
        egui::Id::new("settings_theme_mode_dropdown"),
        Some(selected_index),
        label_refs.as_slice(),
        true,
        ui.available_width(),
        dropdown_height_token(),
    );

    if let Some(next_index) = response
        && next_index != selected_index
    {
        apply_theme_mode_dropdown_selection(state, next_index, actions);
    }
}

fn draw_color_picker_row(
    ui: &mut Ui,
    color_hex: &str,
    enabled: bool,
    ctor: fn(String) -> SettingsAction,
    actions: &mut Vec<SettingsAction>,
) {
    ui.vertical(|ui| {
        let swatch_color = parse_argb_hex(color_hex).unwrap_or(ui.visuals().faint_bg_color);
        let (swatch_rect, _) = ui.allocate_exact_size(
            vec2(color_swatch_width_token(), color_swatch_height_token()),
            Sense::hover(),
        );
        // Slint uses a 6px swatch radius here; keep that exception to match the source UI.
        ui.painter()
            .rect_filled(swatch_rect, CornerRadius::same(6), swatch_color);

        ui.add_enabled_ui(enabled, |ui| {
            if let Some(color) =
                color_picker_ui::draw_bound(ui, color_picker_state_from_argb_hex(color_hex))
            {
                actions.push(ctor(format_argb_hex(color)));
            }
        });
    });
}

fn draw_centered_content_column(
    ui: &mut Ui,
    content_width: f32,
    add_contents: impl FnOnce(&mut Ui),
) {
    let available_width = ui.available_width();
    let side_spacing = ((available_width - content_width).max(0.0)) * 0.5;

    ui.horizontal(|ui| {
        if side_spacing > 0.0 {
            ui.add_space(side_spacing);
        }

        ui.vertical(|ui| {
            ui.set_width(content_width);
            add_contents(ui);
        });
    });
}

fn apply_theme_mode_dropdown_selection(
    state: &SettingsState,
    next_index: usize,
    actions: &mut Vec<SettingsAction>,
) {
    match theme_selection_for_index(state, next_index) {
        ThemeSelection::UseSystemTheme => {
            if !state.use_system_theme {
                actions.push(update_use_system_theme_action(true));
            }
        }
        ThemeSelection::DarkMode => {
            if state.use_system_theme {
                actions.push(update_use_system_theme_action(false));
            }
            if !matches!(state.theme_mode, ThemeMode::Dark) {
                actions.push(update_is_dark_mode_action(true));
            }
        }
        ThemeSelection::LightMode => {
            if state.use_system_theme {
                actions.push(update_use_system_theme_action(false));
            }
            if !matches!(state.theme_mode, ThemeMode::Light) {
                actions.push(update_is_dark_mode_action(false));
            }
        }
    }
}

#[must_use]
fn selected_theme_selection(state: &SettingsState) -> ThemeSelection {
    if state.can_use_system_theme && state.use_system_theme {
        ThemeSelection::UseSystemTheme
    } else if matches!(state.theme_mode, ThemeMode::Dark) {
        ThemeSelection::DarkMode
    } else {
        ThemeSelection::LightMode
    }
}

#[must_use]
fn theme_selection_for_index(state: &SettingsState, index: usize) -> ThemeSelection {
    if state.can_use_system_theme {
        match index {
            0 => ThemeSelection::UseSystemTheme,
            1 => ThemeSelection::DarkMode,
            _ => ThemeSelection::LightMode,
        }
    } else if index == 0 {
        ThemeSelection::DarkMode
    } else {
        ThemeSelection::LightMode
    }
}

#[must_use]
pub fn audio_backend_label(
    backend: AudioPlayerBackend,
    strings: &super::translations::SettingsTranslations,
) -> &str {
    match backend {
        AudioPlayerBackend::Rodio => strings.backend_rodio.as_str(),
        AudioPlayerBackend::Awedio => strings.backend_awedio.as_str(),
        AudioPlayerBackend::Browser => strings.backend_browser.as_str(),
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

#[must_use]
pub fn color_picker_state_from_argb_hex(value: &str) -> ColorPickerState {
    let mut state =
        color_picker_ui::state_for_color(parse_argb_hex(value).unwrap_or(Color32::BLACK));
    state.value_slider_position = ColorPickerDock::Bottom;
    state.wheel_minimum_width = color_picker_wheel_size_token();
    state.wheel_minimum_height = color_picker_wheel_size_token();
    state
}

#[must_use]
pub fn format_argb_hex(color: Color32) -> String {
    let [red, green, blue, alpha] = color.to_srgba_unmultiplied();
    format!(
        "#{alpha:02X}{red:02X}{green:02X}{blue:02X}",
        alpha = alpha,
        red = red,
        green = green,
        blue = blue,
    )
}

#[must_use]
fn audio_backend_index(backend: AudioPlayerBackend) -> usize {
    AudioPlayerBackend::available_for_current_target()
        .iter()
        .position(|candidate| *candidate == backend)
        .unwrap_or(0)
}
