use nject::injectable;

use crate::behavior::Behavior;
use crate::behavior::CompositeDisposable;
use crate::logging::BehaviorLog;

use super::main_view_model::MainViewModel;
use super::messages::OpenImageRequested;
use super::messages::OpenSvgFileCommand;

#[injectable]
#[inject(|| Self)]
#[derive(Clone)]
pub struct OpenImageBehavior;

impl OpenImageBehavior {
    #[must_use]
    pub fn apply(&self, request: OpenImageRequested) -> OpenSvgFileCommand {
        OpenSvgFileCommand {
            file_path: request.file_path,
        }
    }
}

impl Behavior<MainViewModel> for OpenImageBehavior {
    fn activate(&self, _view_model: &mut MainViewModel, _disposables: &mut CompositeDisposable) {
        BehaviorLog::behavior_activated("OpenImageBehavior", "MainViewModel");
    }
}
