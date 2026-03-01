use egui::Ui;

use super::actions::ScenesAction;
use super::state::ScenesState;

pub fn draw(ui: &mut Ui, state: &ScenesState) -> Vec<ScenesAction> {
    let mut actions: Vec<ScenesAction> = Vec::new();
    ui.heading("Scenes");
    ui.label(format!(
        "Visible scenes: {} / {}",
        state.visible_scenes.len(),
        state.scenes.len()
    ));

    let mut search = state.search_text.clone();
    if ui.text_edit_singleline(&mut search).changed() {
        actions.push(ScenesAction::UpdateSearchText(search));
    }

    if ui.button("Insert Before").clicked() {
        actions.push(ScenesAction::InsertScene {
            insert_after: false,
        });
    }
    if ui.button("Insert After").clicked() {
        actions.push(ScenesAction::InsertScene { insert_after: true });
    }

    let mut show_timestamps = state.show_timestamps;
    if ui
        .checkbox(&mut show_timestamps, "Show scene timestamps")
        .changed()
    {
        actions.push(ScenesAction::UpdateShowTimestamps(show_timestamps));
    }

    for (index, scene) in state.visible_scenes.iter().enumerate() {
        let label = if scene.is_selected {
            format!("* {}", scene.name)
        } else {
            scene.name.clone()
        };
        if ui.button(label).clicked() {
            actions.push(ScenesAction::SelectScene { index });
        }
    }
    actions
}
