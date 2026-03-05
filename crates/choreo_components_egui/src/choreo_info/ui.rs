use egui::DragValue;
use egui::Ui;

use super::messages::ChoreoInfoAction;
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
    let percentage = (transparency.clamp(0.0, 1.0) * 100.0).round() as i32;
    format!("Transparency: {percentage}%")
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
    ui.label(choreo_date_text(
        state.choreo_date.year,
        state.choreo_date.month,
        state.choreo_date.day,
    ));
    let mut year = state.choreo_date.year;
    let mut month = i32::from(state.choreo_date.month);
    let mut day = i32::from(state.choreo_date.day);
    let mut date_changed = false;
    ui.horizontal(|ui| {
        date_changed |= ui.add(DragValue::new(&mut year).range(1..=9999)).changed();
        date_changed |= ui.add(DragValue::new(&mut month).range(1..=12)).changed();
        date_changed |= ui.add(DragValue::new(&mut day).range(1..=31)).changed();
    });
    if date_changed {
        actions.push(ChoreoInfoAction::UpdateDate {
            year,
            month: month as u8,
            day: day as u8,
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
