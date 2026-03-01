use egui::Color32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorPickerDock {
    Left,
    Top,
    Right,
    Bottom,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hsb {
    pub hue: f64,
    pub saturation: f64,
    pub brightness: f64,
}

impl Hsb {
    #[must_use]
    pub fn new(hue: f64, saturation: f64, brightness: f64) -> Self {
        Self {
            hue,
            saturation,
            brightness,
        }
    }

    #[must_use]
    pub fn normalized(self) -> Self {
        let mut hue = self.hue % 360.0;
        if hue < 0.0 {
            hue += 360.0;
        }

        Self {
            hue,
            saturation: self.saturation.clamp(0.0, 1.0),
            brightness: self.brightness.clamp(0.0, 1.0),
        }
    }

    #[must_use]
    pub fn from_color(color: Color32) -> Self {
        let red = f64::from(color.r()) / 255.0;
        let green = f64::from(color.g()) / 255.0;
        let blue = f64::from(color.b()) / 255.0;

        let max_channel = red.max(green).max(blue);
        let min_channel = red.min(green).min(blue);
        let delta = max_channel - min_channel;

        let hue = if delta <= f64::EPSILON {
            0.0
        } else if (max_channel - red).abs() <= f64::EPSILON {
            60.0 * ((green - blue) / delta).rem_euclid(6.0)
        } else if (max_channel - green).abs() <= f64::EPSILON {
            60.0 * (((blue - red) / delta) + 2.0)
        } else {
            60.0 * (((red - green) / delta) + 4.0)
        };

        let saturation = if max_channel <= f64::EPSILON {
            0.0
        } else {
            delta / max_channel
        };

        Self {
            hue,
            saturation,
            brightness: max_channel,
        }
    }

    #[must_use]
    pub fn to_color(self) -> Color32 {
        let normalized = self.normalized();
        let hue_section = normalized.hue / 60.0;
        let section_index = hue_section.floor();
        let section_fraction = hue_section - section_index;

        let p = normalized.brightness * (1.0 - normalized.saturation);
        let q = normalized.brightness * (1.0 - normalized.saturation * section_fraction);
        let t = normalized.brightness * (1.0 - normalized.saturation * (1.0 - section_fraction));

        let (red, green, blue) = if normalized.saturation <= f64::EPSILON {
            (normalized.brightness, normalized.brightness, normalized.brightness)
        } else if section_index < 1.0 {
            (normalized.brightness, t, p)
        } else if section_index < 2.0 {
            (q, normalized.brightness, p)
        } else if section_index < 3.0 {
            (p, normalized.brightness, t)
        } else if section_index < 4.0 {
            (p, q, normalized.brightness)
        } else if section_index < 5.0 {
            (t, p, normalized.brightness)
        } else {
            (normalized.brightness, p, q)
        };

        Color32::from_rgb(
            (red * 255.0).round().clamp(0.0, 255.0) as u8,
            (green * 255.0).round().clamp(0.0, 255.0) as u8,
            (blue * 255.0).round().clamp(0.0, 255.0) as u8,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColorChangedEvent {
    pub old_color: Color32,
    pub new_color: Color32,
}

impl ColorChangedEvent {
    #[must_use]
    pub fn new(old_color: Color32, new_color: Color32) -> Self {
        Self {
            old_color,
            new_color,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ColorPickerState {
    pub selected_color: Color32,
    pub hsb: Hsb,
    pub wheel_minimum_width: f32,
    pub wheel_minimum_height: f32,
    pub value_slider_position: ColorPickerDock,
    pub selection_thumb_size: f32,
    pub selection_stroke_thickness: f32,
    pub selection_stroke_color: Color32,
}

impl ColorPickerState {
    pub const DEFAULT_WHEEL_MINIMUM_SIZE: f32 = 160.0;
    pub const DEFAULT_SELECTION_THUMB_SIZE: f32 = 18.0;
    pub const DEFAULT_SELECTION_STROKE_THICKNESS: f32 = 2.0;

    #[must_use]
    pub fn new() -> Self {
        let selected_color = Color32::BLACK;
        let hsb = Hsb::from_color(selected_color);

        Self {
            selected_color,
            hsb,
            wheel_minimum_width: Self::DEFAULT_WHEEL_MINIMUM_SIZE,
            wheel_minimum_height: Self::DEFAULT_WHEEL_MINIMUM_SIZE,
            value_slider_position: ColorPickerDock::Bottom,
            selection_thumb_size: Self::DEFAULT_SELECTION_THUMB_SIZE,
            selection_stroke_thickness: Self::DEFAULT_SELECTION_STROKE_THICKNESS,
            selection_stroke_color: Color32::WHITE,
        }
    }
}

impl Default for ColorPickerState {
    fn default() -> Self {
        Self::new()
    }
}
