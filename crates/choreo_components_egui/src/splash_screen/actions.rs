#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SplashScreenAction {
    Initialize,
    ToggleFlag { key: String },
}
