use super::actions::DrawerHostAction;
use super::state::DrawerHostState;

pub fn reduce(state: &mut DrawerHostState, action: DrawerHostAction) {
    match action {
        DrawerHostAction::OverlayClicked => {
            if state.left_close_on_click_away {
                state.is_left_open = false;
            }
            if state.right_close_on_click_away {
                state.is_right_open = false;
            }
            if state.top_close_on_click_away {
                state.is_top_open = false;
            }
            if state.bottom_close_on_click_away {
                state.is_bottom_open = false;
            }
        }
    }
}
