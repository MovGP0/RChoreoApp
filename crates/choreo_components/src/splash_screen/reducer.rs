use super::actions::SplashScreenAction;
use super::state::SplashScreenState;

pub fn reduce(state: &mut SplashScreenState, action: SplashScreenAction) {
    match action {
        SplashScreenAction::Initialize => {}
        SplashScreenAction::SetBackgroundColor { color } => state.background_color = color,
        SplashScreenAction::SetSplashImagePath { path } => state.splash_image_path = path,
    }
}
