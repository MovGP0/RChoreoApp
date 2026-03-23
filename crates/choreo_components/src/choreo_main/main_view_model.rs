use std::rc::Rc;

use nject::injectable;

use crate::behavior::Behavior;
use crate::behavior::CompositeDisposable;

use super::actions::ChoreoMainAction;
use super::messages::CloseDialogCommand;
use super::messages::OpenAudioRequested;
use super::messages::OpenImageRequested;
use super::messages::OpenSvgFileCommand;
use super::messages::ShowDialogCommand;
use super::reducer::reduce;
use super::state::ChoreoMainState;

#[injectable]
#[inject(|behaviors: Vec<Box<dyn Behavior<MainViewModel>>>| Self::new(behaviors))]
pub struct MainViewModel {
    state: ChoreoMainState,
    behaviors: Vec<Box<dyn Behavior<MainViewModel>>>,
    disposables: CompositeDisposable,
    on_change: Option<Rc<dyn Fn()>>,
}

impl MainViewModel {
    pub fn new(behaviors: Vec<Box<dyn Behavior<MainViewModel>>>) -> Self {
        Self {
            state: ChoreoMainState::default(),
            behaviors,
            disposables: CompositeDisposable::new(),
            on_change: None,
        }
    }

    pub fn activate(&mut self) {
        let behaviors = std::mem::take(&mut self.behaviors);
        let mut disposables = CompositeDisposable::new();
        for behavior in behaviors {
            behavior.activate(self, &mut disposables);
        }
        self.disposables = disposables;
    }

    pub fn dispatch(&mut self, action: ChoreoMainAction) {
        reduce(&mut self.state, action);
        self.notify_changed();
    }

    pub fn show_dialog(&mut self, command: ShowDialogCommand) {
        self.dispatch(ChoreoMainAction::ShowDialog {
            content: command.content,
        });
    }

    pub fn hide_dialog(&mut self, _command: CloseDialogCommand) {
        self.dispatch(ChoreoMainAction::HideDialog);
    }

    pub fn request_open_audio(&mut self, request: OpenAudioRequested) {
        self.dispatch(ChoreoMainAction::RequestOpenAudio(request));
    }

    pub fn request_open_image(&mut self, request: OpenImageRequested) {
        self.dispatch(ChoreoMainAction::RequestOpenImage {
            file_path: request.file_path,
        });
    }

    pub fn open_svg_file(&mut self, command: OpenSvgFileCommand) {
        self.dispatch(ChoreoMainAction::ApplyOpenSvgFile(command));
    }

    pub fn state(&self) -> &ChoreoMainState {
        &self.state
    }

    pub fn state_mut(&mut self) -> &mut ChoreoMainState {
        &mut self.state
    }

    pub fn set_on_change(&mut self, handler: Option<Rc<dyn Fn()>>) {
        self.on_change = handler;
    }

    fn notify_changed(&self) {
        if let Some(handler) = self.on_change.as_ref() {
            handler();
        }
    }
}

impl Drop for MainViewModel {
    fn drop(&mut self) {
        self.disposables.dispose_all();
    }
}
