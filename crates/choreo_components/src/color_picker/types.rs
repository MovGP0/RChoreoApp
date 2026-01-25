use slint::Color;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorPickerDock {
    Left,
    Top,
    Right,
    Bottom,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Hsb {
    pub hue: f64,
    pub saturation: f64,
    pub brightness: f64,
}

impl Hsb {
    pub fn new(hue: f64, saturation: f64, brightness: f64) -> Self {
        Self {
            hue,
            saturation,
            brightness,
        }
    }

    pub fn normalized(self) -> Self {
        let mut hue = self.hue % 360.0;
        if hue < 0.0 {
            hue += 360.0;
        }

        Self {
            hue,
            saturation: clamp_unit(self.saturation),
            brightness: clamp_unit(self.brightness),
        }
    }

    pub fn from_color(color: Color) -> Self {
        let hsva = color.to_hsva();
        Self {
            hue: hsva.hue as f64,
            saturation: hsva.saturation as f64,
            brightness: hsva.value as f64,
        }
    }

    pub fn to_color(self) -> Color {
        let normalized = self.normalized();
        Color::from_hsva(
            normalized.hue as f32,
            normalized.saturation as f32,
            normalized.brightness as f32,
            1.0,
        )
    }
}

fn clamp_unit(value: f64) -> f64 {
    value.clamp(0.0, 1.0)
}
