use egui::Ui;
use egui_material3::MaterialSlider;

use crate::material::components::DatePickerStrings;
use crate::material::components::DatePickerValue;
use crate::material::components::date_picker;

use super::messages::ChoreoInfoAction;
use super::state::ChoreoDate;
use super::state::ChoreoInfoState;

pub struct ChoreoInfoLabels {
    pub comment: String,
    pub name: String,
    pub subtitle: String,
    pub date: String,
    pub variation: String,
    pub author: String,
    pub description: String,
}

#[must_use]
pub fn choreo_date_text(year: i32, month: u8, day: u8) -> String {
    format!("{year:04}-{month:02}-{day:02}")
}

#[must_use]
pub fn transparency_percentage_text(transparency: f64) -> String {
    format!(
        "Transparency: {}",
        transparency_percentage_suffix(transparency)
    )
}

#[must_use]
pub fn transparency_percentage_suffix(transparency: f64) -> String {
    let percentage = (transparency.clamp(0.0, 1.0) * 100.0).round() as i32;
    format!("{percentage}%")
}

#[must_use]
pub const fn uses_calendar_date_picker() -> bool {
    true
}

#[must_use]
pub fn picker_date_value(date: ChoreoDate) -> DatePickerValue {
    crate::material::components::date_picker::normalize_date_value(DatePickerValue {
        year: date.year,
        month: date.month,
        day: date.day,
    })
}

pub fn draw(
    ui: &mut Ui,
    state: &ChoreoInfoState,
    labels: &ChoreoInfoLabels,
) -> Vec<ChoreoInfoAction> {
    let mut actions = Vec::new();

    ui.label(&labels.comment);
    let mut comment = state.choreo_comment.clone();
    if ui.text_edit_singleline(&mut comment).changed() {
        actions.push(ChoreoInfoAction::UpdateComment(comment));
    }

    ui.label(&labels.name);
    let mut name = state.choreo_name.clone();
    if ui.text_edit_singleline(&mut name).changed() {
        actions.push(ChoreoInfoAction::UpdateName(name));
    }

    ui.label(&labels.subtitle);
    let mut subtitle = state.choreo_subtitle.clone();
    if ui.text_edit_singleline(&mut subtitle).changed() {
        actions.push(ChoreoInfoAction::UpdateSubtitle(subtitle));
    }

    ui.label(&labels.date);
    if let Some(selected_date) = date_picker(
        ui,
        "choreo_info_date_picker",
        picker_date_value(state.choreo_date),
        DatePickerStrings {
            title: &labels.date,
            cancel_text: "Cancel",
            ok_text: "Ok",
        },
    ) {
        actions.push(ChoreoInfoAction::UpdateDate {
            year: selected_date.year,
            month: selected_date.month,
            day: selected_date.day,
        });
    }

    ui.label(&labels.variation);
    let mut variation = state.choreo_variation.clone();
    if ui.text_edit_singleline(&mut variation).changed() {
        actions.push(ChoreoInfoAction::UpdateVariation(variation));
    }

    ui.label(&labels.author);
    let mut author = state.choreo_author.clone();
    if ui.text_edit_singleline(&mut author).changed() {
        actions.push(ChoreoInfoAction::UpdateAuthor(author));
    }

    ui.label(&labels.description);
    let mut description = state.choreo_description.clone();
    if ui.text_edit_singleline(&mut description).changed() {
        actions.push(ChoreoInfoAction::UpdateDescription(description));
    }

    actions
}

pub fn draw_transparency(ui: &mut Ui, transparency: f64, label: &str) -> Option<ChoreoInfoAction> {
    let mut value = transparency as f32;
    ui.label(format!(
        "{label}: {}",
        transparency_percentage_suffix(f64::from(value))
    ));

    if ui
        .add(
            MaterialSlider::new(&mut value, 0.0..=1.0)
                .show_value(false)
                .width(240.0),
        )
        .changed()
    {
        return Some(ChoreoInfoAction::UpdateTransparency(f64::from(value)));
    }

    None
}
