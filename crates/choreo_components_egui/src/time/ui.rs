use egui::Ui;

use super::actions::TimeAction;
use super::state::TimeState;

pub fn draw(ui: &mut Ui, state: &TimeState) -> Vec<TimeAction> {
    let mut actions: Vec<TimeAction> = Vec::new();
    ui.heading("Time");
    ui.label(format!("Input: {}", state.timestamp_input));
    ui.label(format!(
        "Parsed seconds: {}",
        state
            .parsed_seconds
            .map(|value| value.to_string())
            .unwrap_or_else(|| "n/a".to_string())
    ));
    ui.label(format!("Formatted: {}", state.formatted_seconds));

    if ui.button("Now").clicked() {
        actions.push(TimeAction::InitializeCurrentTime);
    }
    if ui.button("Parse input").clicked() {
        actions.push(TimeAction::ParseTimestampInput);
    }
    actions
}
