use egui::Ui;

use super::actions::ChoreographySettingsAction;
use super::actions::UpdateSelectedSceneAction;
use super::state::ChoreographySettingsState;

pub fn draw(ui: &mut Ui, state: &ChoreographySettingsState) -> Vec<ChoreographySettingsAction> {
    let mut actions: Vec<ChoreographySettingsAction> = Vec::new();
    ui.heading("Choreography Settings");
    ui.label(format!("Name: {}", state.name));

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
        actions.push(ChoreographySettingsAction::UpdateDrawPathFrom(draw_path_from));
    }

    let mut draw_path_to = state.draw_path_to;
    if ui.checkbox(&mut draw_path_to, "Draw path to").changed() {
        actions.push(ChoreographySettingsAction::UpdateDrawPathTo(draw_path_to));
    }

    let mut transparency = state.transparency;
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
