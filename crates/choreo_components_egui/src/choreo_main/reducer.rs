use super::actions::ChoreoMainAction;
use super::state::ChoreoMainState;

pub fn reduce(state: &mut ChoreoMainState, action: ChoreoMainAction) {
    match action {
        ChoreoMainAction::Initialize => {}
        ChoreoMainAction::ToggleFlag { key } => {
            let previous = state.flags.get(&key).copied().unwrap_or(false);
            state.flags.insert(key, !previous);
        }
    }
}
