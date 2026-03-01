use egui::Ui;

use super::actions::PreferencesAction;
use super::state::PreferencesState;

pub fn draw(ui: &mut Ui, state: &PreferencesState) -> Vec<PreferencesAction> {
    let mut actions: Vec<PreferencesAction> = Vec::new();
    ui.heading("Preferences");
    ui.label(format!("App: {}", state.app_name));
    ui.label(format!(
        "Values: {} strings, {} bools",
        state.strings.len(),
        state.bools.len()
    ));
    ui.label(format!("Pending writes: {}", state.pending_writes.len()));

    let mut show_timestamps = state.get_bool("show_timestamps", false);
    if ui
        .checkbox(&mut show_timestamps, "Show timestamps")
        .changed()
    {
        actions.push(PreferencesAction::SetBool {
            key: "show_timestamps".to_string(),
            value: show_timestamps,
        });
    }

    let mut last_opened = state.get_string("last_opened_choreo_file", "");
    if ui.text_edit_singleline(&mut last_opened).changed() {
        actions.push(PreferencesAction::SetString {
            key: "last_opened_choreo_file".to_string(),
            value: last_opened,
        });
    }

    if ui.button("Clear pending writes").clicked() {
        actions.push(PreferencesAction::ClearPendingWrites);
    }
    if ui.button("Clear all").clicked() {
        actions.push(PreferencesAction::ClearAll);
    }
    actions
}
