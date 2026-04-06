use nject::injectable;

use super::messages::CloseDialogCommand;
use super::state::ChoreoMainState;

#[injectable]
#[inject(|| Self)]
#[derive(Clone, Default)]
pub struct HideDialogBehavior;

impl HideDialogBehavior {
    pub fn apply(state: &mut ChoreoMainState, _command: CloseDialogCommand) {
        state.dialog_content = None;
        state.is_dialog_open = false;
    }
}
