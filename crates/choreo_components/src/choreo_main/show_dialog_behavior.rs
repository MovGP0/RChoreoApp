use nject::injectable;

use super::messages::ShowDialogCommand;
use super::state::ChoreoMainState;

#[injectable]
#[inject(|| Self)]
#[derive(Clone, Default)]
pub struct ShowDialogBehavior;

impl ShowDialogBehavior {
    pub fn apply(state: &mut ChoreoMainState, command: ShowDialogCommand) {
        state.dialog_content = command.content;
        state.is_dialog_open = state.dialog_content.is_some();
    }
}
