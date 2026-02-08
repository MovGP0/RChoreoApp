use slint::Color;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorChangedEvent {
    pub old_color: Color,
    pub new_color: Color,
}

impl ColorChangedEvent {
    pub fn new(old_color: Color, new_color: Color) -> Self {
        Self {
            old_color,
            new_color,
        }
    }
}
