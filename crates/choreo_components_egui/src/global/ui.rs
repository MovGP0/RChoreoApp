use egui::Ui;

use super::actions::GlobalAction;
use super::state::{GlobalState, InteractionMode};

pub fn draw(ui: &mut Ui, state: &GlobalState) -> Vec<GlobalAction> {
    let mut actions: Vec<GlobalAction> = Vec::new();
    ui.heading("Global Interaction State");
    ui.label(format!("mode: {:?}", state.interaction_mode));
    ui.label(format!("place mode: {}", state.is_place_mode));
    ui.label(format!(
        "selected scene: {}",
        state.selected_scene_id.as_deref().unwrap_or("<none>")
    ));
    if ui.button("Initialize").clicked() {
        actions.push(GlobalAction::Initialize);
    }
    if ui.button("View mode").clicked() {
        actions.push(GlobalAction::SetInteractionMode {
            mode: InteractionMode::View,
        });
    }
    if ui.button("Move mode").clicked() {
        actions.push(GlobalAction::SetInteractionMode {
            mode: InteractionMode::Move,
        });
    }
    if ui
        .button(if state.is_place_mode {
            "Disable place mode"
        } else {
            "Enable place mode"
        })
        .clicked()
    {
        actions.push(GlobalAction::SetPlaceMode {
            is_place_mode: !state.is_place_mode,
        });
    }
    if ui.button("Request floor redraw").clicked() {
        actions.push(GlobalAction::RequestFloorRedraw);
    }
    if ui.button("Request scene scroll").clicked() {
        actions.push(GlobalAction::RequestScrollToSelectedScene);
    }
    actions
}
