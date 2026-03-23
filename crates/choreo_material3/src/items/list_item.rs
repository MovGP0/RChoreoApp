use egui::Color32;
use egui::Image;

#[derive(Clone)]
pub struct ListItem {
    pub text: String,
    pub supporting_text: String,
    pub avatar_icon: Option<Image<'static>>,
    pub avatar_text: String,
    pub avatar_background: Option<Color32>,
    pub avatar_foreground: Option<Color32>,
    pub action_button_icon: Option<Image<'static>>,
}

impl ListItem {
    #[must_use]
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            supporting_text: String::new(),
            avatar_icon: None,
            avatar_text: String::new(),
            avatar_background: None,
            avatar_foreground: None,
            action_button_icon: None,
        }
    }
}
