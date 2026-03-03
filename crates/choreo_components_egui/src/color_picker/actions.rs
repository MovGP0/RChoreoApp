#[derive(Debug, Clone, PartialEq)]
pub enum ColorPickerAction {
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
    UpdateFromWheelPoint {
        x: f32,
        y: f32,
        center_x: f32,
        center_y: f32,
        radius_px: f32,
    },
    SetValueSliderPosition {
        position: super::state::ColorPickerDock,
    },
}
