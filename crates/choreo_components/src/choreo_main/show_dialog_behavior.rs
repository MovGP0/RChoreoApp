use nject::injectable;

use crate::behavior::Behavior;
use crate::behavior::CompositeDisposable;
use crate::logging::BehaviorLog;

use super::main_view_model::MainViewModel;
use super::messages::ShowDialogCommand;

#[injectable]
#[inject(|| Self)]
#[derive(Clone, Default)]
pub struct ShowDialogBehavior;

impl ShowDialogBehavior {
    pub fn apply(view_model: &mut MainViewModel, command: ShowDialogCommand) {
        view_model.show_dialog(command);
    }
}

impl Behavior<MainViewModel> for ShowDialogBehavior {
    fn activate(&self, _view_model: &mut MainViewModel, _disposables: &mut CompositeDisposable) {
        BehaviorLog::behavior_activated("ShowDialogBehavior", "MainViewModel");
    }
}
