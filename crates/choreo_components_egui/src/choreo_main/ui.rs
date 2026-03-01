use egui::Ui;

use super::actions::ChoreoMainAction;
use super::state::ChoreoMainState;
use super::state::MainContent;

pub fn draw(ui: &mut Ui, state: &ChoreoMainState) -> Vec<ChoreoMainAction> {
    let mut actions: Vec<ChoreoMainAction> = Vec::new();
    ui.heading("Choreo Main");
    ui.horizontal(|ui| {
        if ui.button("Main").clicked() {
            actions.push(ChoreoMainAction::NavigateToMain);
        }
        if ui.button("Settings").clicked() {
            actions.push(ChoreoMainAction::NavigateToSettings);
        }
        if ui.button("Dancers").clicked() {
            actions.push(ChoreoMainAction::NavigateToDancers);
        }
    });

    let content_label = match state.content {
        MainContent::Main => "main",
        MainContent::Settings => "settings",
        MainContent::Dancers => "dancers",
    };
    ui.label(format!("current content: {content_label}"));

    if state.is_dialog_open {
        ui.group(|ui| {
            ui.label("Dialog");
            ui.label(state.dialog_content.as_deref().unwrap_or_default());
            if ui.button("Close Dialog").clicked() {
                actions.push(ChoreoMainAction::HideDialog);
            }
        });
    }

    ui.separator();
    ui.label(format!(
        "selected scene: {}",
        state.floor_scene_name.as_deref().unwrap_or("none")
    ));
    ui.label(format!(
        "audio position: {:.2}s",
        state.audio_position_seconds
    ));
    if ui.button("Initialize").clicked() {
        actions.push(ChoreoMainAction::Initialize);
    }
    actions
}
