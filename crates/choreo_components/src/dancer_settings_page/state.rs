use choreo_master_mobile_json::Color;
use egui::Color32;

use crate::dancers::state::DancerState;
use crate::dancers::state::DancersState;
use crate::material::components::color_picker::state::ColorPickerState;
use crate::material::components::color_picker::ui as color_picker_ui;

pub type DancerSettingsPageState = DancersState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SwapDialogViewModel {
    pub title_text: String,
    pub first_dancer_name: String,
    pub second_dancer_name: String,
    pub first_dancer_color: Color32,
    pub second_dancer_color: Color32,
    pub message_text: String,
    pub cancel_text: String,
    pub confirm_text: String,
}

#[must_use]
pub fn selected_dancer_color_picker_state(state: &DancerSettingsPageState) -> ColorPickerState {
    let selected_color = state
        .selected_dancer
        .as_ref()
        .map(|dancer| color_to_egui(&dancer.color))
        .unwrap_or(Color32::TRANSPARENT);

    color_picker_ui::state_for_color(selected_color)
}

#[must_use]
pub fn role_option_labels(state: &DancerSettingsPageState) -> Vec<String> {
    state.roles.iter().map(|role| role.name.clone()).collect()
}

#[must_use]
pub fn dancer_option_labels(state: &DancerSettingsPageState) -> Vec<String> {
    state
        .dancers
        .iter()
        .map(|dancer| dancer.name.clone())
        .collect()
}

#[must_use]
pub fn icon_option_labels(state: &DancerSettingsPageState) -> Vec<String> {
    state
        .icon_options
        .iter()
        .map(|option| option.display_name.clone())
        .collect()
}

#[must_use]
pub fn build_swap_dialog_view_model(
    state: &DancerSettingsPageState,
    locale: &str,
) -> Option<SwapDialogViewModel> {
    if !state.is_dialog_open || state.dialog_content.as_deref() != Some("swap_dancers") {
        return None;
    }

    let first_name = swap_dialog_dancer_name(state.swap_from_dancer.as_ref());
    let second_name = swap_dialog_dancer_name(state.swap_to_dancer.as_ref());
    let message_template = crate::i18n::t(locale, "DancerSwapDialogMessage");

    Some(SwapDialogViewModel {
        title_text: crate::i18n::t(locale, "DancerSwapDialogTitle"),
        first_dancer_name: first_name.clone(),
        second_dancer_name: second_name.clone(),
        first_dancer_color: state
            .swap_from_dancer
            .as_ref()
            .map(|dancer| color_to_egui(&dancer.color))
            .unwrap_or(Color32::TRANSPARENT),
        second_dancer_color: state
            .swap_to_dancer
            .as_ref()
            .map(|dancer| color_to_egui(&dancer.color))
            .unwrap_or(Color32::TRANSPARENT),
        message_text: message_template
            .replace("{0}", &first_name)
            .replace("{1}", &second_name),
        cancel_text: crate::i18n::t(locale, "DancerSwapDialogCancel"),
        confirm_text: crate::i18n::t(locale, "DancerSwapDialogConfirm"),
    })
}

#[must_use]
pub fn selected_dancer_index(state: &DancerSettingsPageState) -> Option<usize> {
    let selected_dancer_id = state
        .selected_dancer
        .as_ref()
        .map(|dancer| dancer.dancer_id)?;
    state
        .dancers
        .iter()
        .position(|dancer| dancer.dancer_id == selected_dancer_id)
}

#[must_use]
pub fn selected_role_index(state: &DancerSettingsPageState) -> Option<usize> {
    let selected_role_name = state
        .selected_role
        .as_ref()
        .map(|role| role.name.as_str())?;
    state
        .roles
        .iter()
        .position(|role| role.name == selected_role_name)
}

#[must_use]
pub fn selected_icon_index(state: &DancerSettingsPageState) -> Option<usize> {
    let selected_key = state
        .selected_icon_option
        .as_ref()
        .map(|option| option.key.as_str())?;
    state
        .icon_options
        .iter()
        .position(|option| option.key == selected_key)
}

#[must_use]
pub fn dancer_supporting_text(dancer: &DancerState) -> String {
    format!(
        "{} ({})  [{}]",
        dancer.role.name, dancer.role.z_index, dancer.shortcut
    )
}

#[must_use]
pub fn dancer_role_details_text(dancer: &DancerState) -> String {
    dancer_supporting_text(dancer)
}

fn swap_dialog_dancer_name(dancer: Option<&DancerState>) -> String {
    let Some(dancer) = dancer else {
        return "?".to_string();
    };

    if !dancer.name.trim().is_empty() {
        return dancer.name.clone();
    }
    if !dancer.shortcut.trim().is_empty() {
        return dancer.shortcut.clone();
    }

    format!("#{}", dancer.dancer_id)
}

fn color_to_egui(color: &Color) -> Color32 {
    Color32::from_rgba_unmultiplied(color.r, color.g, color.b, color.a)
}
