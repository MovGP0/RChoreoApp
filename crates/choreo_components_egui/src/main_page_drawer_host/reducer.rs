use super::actions::MainPageDrawerHostAction;
use super::state::MainPageDrawerHostState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MainPageDrawerHostEffect {
    LeftCloseRequested,
    RightCloseRequested,
    OverlayClicked,
}

pub fn reduce(
    state: &mut MainPageDrawerHostState,
    action: MainPageDrawerHostAction,
) -> Vec<MainPageDrawerHostEffect> {
    match action {
        MainPageDrawerHostAction::Initialize => Vec::new(),
        MainPageDrawerHostAction::SetInlineLeft { inline } => {
            state.inline_left = inline;
            Vec::new()
        }
        MainPageDrawerHostAction::SetLeftOpen { is_open } => {
            state.is_left_open = is_open;
            Vec::new()
        }
        MainPageDrawerHostAction::SetRightOpen { is_open } => {
            state.is_right_open = is_open;
            Vec::new()
        }
        MainPageDrawerHostAction::SetTopInset { top_inset } => {
            state.top_inset = top_inset.max(0.0);
            Vec::new()
        }
        MainPageDrawerHostAction::OverlayClicked => {
            let mut effects: Vec<MainPageDrawerHostEffect> = Vec::new();
            if state.left_close_on_click_away && state.is_left_open {
                effects.push(MainPageDrawerHostEffect::LeftCloseRequested);
                state.is_left_open = false;
            }
            if state.right_close_on_click_away && state.is_right_open {
                effects.push(MainPageDrawerHostEffect::RightCloseRequested);
                state.is_right_open = false;
            }
            effects.push(MainPageDrawerHostEffect::OverlayClicked);
            effects
        }
    }
}
