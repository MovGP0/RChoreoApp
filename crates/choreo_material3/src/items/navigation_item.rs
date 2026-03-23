use egui::Image;

#[derive(Clone)]
pub struct NavigationItem {
    pub icon: Option<Image<'static>>,
    pub selected_icon: Option<Image<'static>>,
    pub text: String,
    pub show_badge: bool,
    pub badge: String,
}

impl NavigationItem {
    #[must_use]
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            icon: None,
            selected_icon: None,
            text: text.into(),
            show_badge: false,
            badge: String::new(),
        }
    }
}

#[derive(Clone, Default)]
pub struct NavigationGroup {
    pub title: String,
    pub items: Vec<NavigationItem>,
}
