use nject::injectable;

use crate::behavior::Behavior;
use crate::behavior::CompositeDisposable;
use crate::logging::BehaviorLog;

use super::main_view_model::MainViewModel;
use super::messages::CloseDialogCommand;

#[injectable]
#[inject(|| Self)]
#[derive(Clone, Default)]
pub struct HideDialogBehavior;

impl HideDialogBehavior {
    pub fn apply(view_model: &mut MainViewModel, command: CloseDialogCommand) {
        view_model.hide_dialog(command);
    }
}

impl Behavior<MainViewModel> for HideDialogBehavior {
    fn activate(&self, _view_model: &mut MainViewModel, _disposables: &mut CompositeDisposable) {
        BehaviorLog::behavior_activated("HideDialogBehavior", "MainViewModel");
    }
}
