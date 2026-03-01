use super::actions::SplashScreenAction;
use super::state::SplashScreenState;

pub fn reduce(state: &mut SplashScreenState, action: SplashScreenAction) {
    match action {
        SplashScreenAction::Initialize => {}
        SplashScreenAction::ToggleFlag { key } => {
            let previous = state.flags.get(&key).copied().unwrap_or(false);
            state.flags.insert(key, !previous);
        }
    }
}
