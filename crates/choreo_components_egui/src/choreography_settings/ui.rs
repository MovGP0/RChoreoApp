use egui::Ui;

use super::actions::ChoreographySettingsAction;
use super::actions::UpdateSelectedSceneAction;
use super::state::ChoreographySettingsState;

#[must_use]
pub fn choreo_date_text(year: i32, month: u8, day: u8) -> String {
    format!("{year:04}-{month:02}-{day:02}")
}

#[must_use]
pub fn transparency_percentage_text(transparency: f64) -> String {
    let percentage = (transparency.clamp(0.0, 1.0) * 100.0).round() as i32;
    format!("Transparency: {percentage}%")
}

pub fn draw(ui: &mut Ui, state: &ChoreographySettingsState) -> Vec<ChoreographySettingsAction> {
    let mut actions: Vec<ChoreographySettingsAction> = Vec::new();
    ui.heading("Choreography Settings");
    ui.separator();
    ui.heading("Choreography");

    let mut comment = state.comment.clone();
    if ui.text_edit_singleline(&mut comment).changed() {
        actions.push(ChoreographySettingsAction::UpdateComment(comment));
    }

    let mut name = state.name.clone();
    if ui.text_edit_singleline(&mut name).changed() {
        actions.push(ChoreographySettingsAction::UpdateName(name));
    }

    let mut subtitle = state.subtitle.clone();
    if ui.text_edit_singleline(&mut subtitle).changed() {
        actions.push(ChoreographySettingsAction::UpdateSubtitle(subtitle));
    }

    ui.label(format!(
        "Date: {}",
        choreo_date_text(state.date.year, state.date.month, state.date.day)
    ));
    let mut year = state.date.year;
    let mut month = i32::from(state.date.month);
    let mut day = i32::from(state.date.day);
    let mut date_changed = false;
    ui.horizontal(|ui| {
        date_changed |= ui
            .add(egui::DragValue::new(&mut year).range(1..=9999))
            .changed();
        date_changed |= ui
            .add(egui::DragValue::new(&mut month).range(1..=12))
            .changed();
        date_changed |= ui
            .add(egui::DragValue::new(&mut day).range(1..=31))
            .changed();
    });
    if date_changed {
        actions.push(ChoreographySettingsAction::UpdateDate {
            year,
            month: month as u8,
            day: day as u8,
        });
    }

    let mut variation = state.variation.clone();
    if ui.text_edit_singleline(&mut variation).changed() {
        actions.push(ChoreographySettingsAction::UpdateVariation(variation));
    }

    let mut author = state.author.clone();
    if ui.text_edit_singleline(&mut author).changed() {
        actions.push(ChoreographySettingsAction::UpdateAuthor(author));
    }

    let mut description = state.description.clone();
    if ui.text_edit_singleline(&mut description).changed() {
        actions.push(ChoreographySettingsAction::UpdateDescription(description));
    }

    let mut show_timestamps = state.show_timestamps;
    if ui
        .checkbox(&mut show_timestamps, "Show timestamps")
        .changed()
    {
        actions.push(ChoreographySettingsAction::UpdateShowTimestamps(
            show_timestamps,
        ));
    }

    let mut positions_at_side = state.positions_at_side;
    if ui
        .checkbox(&mut positions_at_side, "Positions at side")
        .changed()
    {
        actions.push(ChoreographySettingsAction::UpdatePositionsAtSide(
            positions_at_side,
        ));
    }

    let mut draw_path_from = state.draw_path_from;
    if ui.checkbox(&mut draw_path_from, "Draw path from").changed() {
        actions.push(ChoreographySettingsAction::UpdateDrawPathFrom(
            draw_path_from,
        ));
    }

    let mut draw_path_to = state.draw_path_to;
    if ui.checkbox(&mut draw_path_to, "Draw path to").changed() {
        actions.push(ChoreographySettingsAction::UpdateDrawPathTo(draw_path_to));
    }

    let mut transparency = state.transparency;
    ui.label(transparency_percentage_text(transparency));
    if ui
        .add(egui::Slider::new(&mut transparency, 0.0..=1.0).text("Transparency"))
        .changed()
    {
        actions.push(ChoreographySettingsAction::UpdateTransparency(transparency));
    }

    if state.has_selected_scene {
        ui.separator();
        ui.label("Selected Scene");
        let mut scene_name = state.scene_name.clone();
        if ui.text_edit_singleline(&mut scene_name).changed() {
            actions.push(ChoreographySettingsAction::UpdateSelectedScene(
                UpdateSelectedSceneAction::SceneName(scene_name),
            ));
        }

        let mut scene_fixed_positions = state.scene_fixed_positions;
        if ui
            .checkbox(&mut scene_fixed_positions, "Fixed positions")
            .changed()
        {
            actions.push(ChoreographySettingsAction::UpdateSelectedScene(
                UpdateSelectedSceneAction::SceneFixedPositions(scene_fixed_positions),
            ));
        }
    }
    actions
}
