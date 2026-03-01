use egui::Ui;

use super::actions::ObservabilityAction;
use super::state::ObservabilityState;

pub fn draw(ui: &mut Ui, state: &ObservabilityState) -> Vec<ObservabilityAction> {
    let mut actions: Vec<ObservabilityAction> = Vec::new();
    ui.heading("Observability");
    ui.label(format!("tracing enabled: {}", state.tracing_enabled));
    ui.label(format!(
        "active span: {}",
        state
            .active_span
            .as_ref()
            .map(|span| span.name.as_str())
            .unwrap_or("<none>")
    ));
    ui.label(format!("completed spans: {}", state.completed_spans.len()));
    if ui.button("Initialize").clicked() {
        actions.push(ObservabilityAction::Initialize);
    }
    if ui
        .button(if state.tracing_enabled {
            "Disable tracing"
        } else {
            "Enable tracing"
        })
        .clicked()
    {
        actions.push(ObservabilityAction::SetTracingEnabled {
            enabled: !state.tracing_enabled,
        });
    }
    if ui.button("Start UI span").clicked() {
        actions.push(ObservabilityAction::StartSpan {
            name: "observability.ui".to_owned(),
            trace_context: None,
        });
    }
    if ui.button("End active span").clicked() {
        actions.push(ObservabilityAction::EndActiveSpan);
    }
    actions
}
