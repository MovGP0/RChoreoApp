use super::actions::NavBarAction;
use super::state::NavBarState;

pub fn reduce(state: &mut NavBarState, action: NavBarAction) {
    match action {
        NavBarAction::Initialize => {}
        NavBarAction::ToggleFlag { key } => {
            let previous = state.flags.get(&key).copied().unwrap_or(false);
            state.flags.insert(key, !previous);
        }
    }
}
