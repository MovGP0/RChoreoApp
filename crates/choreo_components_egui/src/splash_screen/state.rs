#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SplashScreenState {
    pub background_color: egui::Color32,
    pub splash_image_path: String,
}

impl SplashScreenState {
    #[must_use]
    pub fn new() -> Self {
        Self {
            background_color: egui::Color32::from_rgb(0xF8, 0xFA, 0xFD),
            splash_image_path: "splash.svg".to_owned(),
        }
    }
}

impl Default for SplashScreenState {
    fn default() -> Self {
        Self::new()
    }
}
