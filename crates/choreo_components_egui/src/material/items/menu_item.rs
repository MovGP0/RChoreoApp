use egui::Image;

#[derive(Clone)]
pub struct MenuItem {
    pub icon: Option<Image<'static>>,
    pub text: String,
    pub trailing_text: String,
    pub enabled: bool,
}

impl MenuItem {
    #[must_use]
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            icon: None,
            text: text.into(),
            trailing_text: String::new(),
            enabled: true,
        }
    }
}
