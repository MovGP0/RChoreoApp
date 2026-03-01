#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SplashScreenAction {
    Initialize,
    SetBackgroundColor {
        color: egui::Color32,
    },
    SetSplashImagePath {
        path: String,
    },
}
