use slint::Color;

use super::{ColorChangedEvent, ColorPickerDock, Hsb};

pub struct ColorPickerViewModel {
    pub color: Color,
    pub hsb: Hsb,
    pub wheel_minimum_width: f64,
    pub wheel_minimum_height: f64,
    pub value_slider_position: ColorPickerDock,
    pub selection_thumb_size: f64,
    pub selection_stroke_thickness: f64,
    pub selection_stroke_color: Color,
}

impl ColorPickerViewModel {
    pub const DEFAULT_WHEEL_MINIMUM_SIZE: f64 = 160.0;
    pub const DEFAULT_SELECTION_THUMB_SIZE: f64 = 18.0;
    pub const DEFAULT_SELECTION_STROKE_THICKNESS: f64 = 2.0;

    pub fn new() -> Self {
        let color = Color::from_rgb_u8(0, 0, 0);
        let hsb = Hsb::from_color(color);

        Self {
            color,
            hsb,
            wheel_minimum_width: Self::DEFAULT_WHEEL_MINIMUM_SIZE,
            wheel_minimum_height: Self::DEFAULT_WHEEL_MINIMUM_SIZE,
            value_slider_position: ColorPickerDock::Bottom,
            selection_thumb_size: Self::DEFAULT_SELECTION_THUMB_SIZE,
            selection_stroke_thickness: Self::DEFAULT_SELECTION_STROKE_THICKNESS,
            selection_stroke_color: Color::from_rgb_u8(0xFF, 0xFF, 0xFF),
        }
    }

    pub fn set_color(&mut self, color: Color) -> Option<ColorChangedEvent> {
        if color == self.color {
            return None;
        }

        let old_color = self.color;
        self.color = color;
        self.hsb = Hsb::from_color(color);
        Some(ColorChangedEvent::new(old_color, color))
    }

    pub fn set_hsb(&mut self, hsb: Hsb) -> Option<ColorChangedEvent> {
        if hsb == self.hsb {
            return None;
        }

        let old_color = self.color;
        let normalized = hsb.normalized();
        let new_color = normalized.to_color();
        self.hsb = normalized;
        self.color = new_color;

        if new_color == old_color {
            return None;
        }

        Some(ColorChangedEvent::new(old_color, new_color))
    }

    pub fn update_from_slider(&mut self, brightness: f64) -> Option<ColorChangedEvent> {
        let next = Hsb {
            brightness,
            ..self.hsb
        };
        self.set_hsb(next)
    }

    pub fn update_from_wheel(&mut self, hue: f64, saturation: f64) -> Option<ColorChangedEvent> {
        let next = Hsb {
            hue,
            saturation,
            brightness: self.hsb.brightness,
        };
        self.set_hsb(next)
    }
}

impl Default for ColorPickerViewModel {
    fn default() -> Self {
        Self::new()
    }
}
