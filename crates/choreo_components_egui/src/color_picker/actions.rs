#[derive(Debug, Clone, PartialEq)]
pub enum ColorPickerAction {
    Initialize,
    SetColor {
        color: egui::Color32,
    },
    SetHsb {
        hsb: super::state::Hsb,
    },
    UpdateFromSlider {
        brightness: f64,
    },
    UpdateFromWheel {
        hue: f64,
        saturation: f64,
    },
    SetValueSliderPosition {
        position: super::state::ColorPickerDock,
    },
}
