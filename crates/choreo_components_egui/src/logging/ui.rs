use egui::Ui;

use super::actions::LoggingAction;
use super::state::LoggingState;

pub fn draw(ui: &mut Ui, state: &LoggingState) -> Vec<LoggingAction> {
    let mut actions: Vec<LoggingAction> = Vec::new();
    ui.heading("Logging");
    ui.label(format!("Entries: {}", state.entries.len()));
    ui.label(format!("Dropped: {}", state.dropped_entries));

    if ui.button("Clear").clicked() {
        actions.push(LoggingAction::ClearEntries);
    }
    if ui.button("Test debug log").clicked() {
        actions.push(LoggingAction::RecordDebug {
            message: "debug event".to_string(),
        });
    }
    actions
}
