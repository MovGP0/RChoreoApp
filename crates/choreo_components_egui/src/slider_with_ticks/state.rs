#[derive(Debug, Clone)]
pub struct SliderWithTicksState {
    pub minimum: f64,
    pub maximum: f64,
    pub value: f64,
    pub tick_values: Vec<f64>,
    pub tick_color: Option<egui::Color32>,
    pub is_enabled: bool,
}

impl SliderWithTicksState {
    #[must_use]
    pub fn new() -> Self {
        Self {
            minimum: 0.0,
            maximum: 1.0,
            value: 0.0,
            tick_values: Vec::new(),
            tick_color: None,
            is_enabled: true,
        }
    }
}

impl Default for SliderWithTicksState {
    fn default() -> Self {
        Self::new()
    }
}
