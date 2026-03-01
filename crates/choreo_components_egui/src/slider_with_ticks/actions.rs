#[derive(Debug, Clone, PartialEq)]
pub enum SliderWithTicksAction {
    Initialize,
    SetRange {
        minimum: f64,
        maximum: f64,
    },
    SetValue {
        value: f64,
    },
    SetTickValues {
        tick_values: Vec<f64>,
    },
    SetTickColor {
        tick_color: Option<egui::Color32>,
    },
    SetEnabled {
        is_enabled: bool,
    },
}
