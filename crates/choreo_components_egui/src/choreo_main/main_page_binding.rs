use std::cell::RefCell;
use std::rc::Rc;

use crate::behavior::Behavior;

use super::actions::ChoreoMainAction;
use super::actions::OpenAudioRequested;
use super::main_view_model::MainViewModel;
use super::main_view_model_provider::MainViewModelProvider;
use super::main_view_model_provider::MainViewModelProviderDependencies;
use super::messages::CloseDialogCommand;
use super::messages::OpenImageRequested;
use super::messages::OpenSvgFileCommand;
use super::messages::ShowDialogCommand;
use super::runtime::consume_outgoing_commands;
use super::runtime::enqueue_open_audio_request;

#[derive(Clone, Default)]
pub struct MainPageActionHandlers {
    pub pick_audio_path: Option<Rc<dyn Fn() -> Option<String>>>,
    pub pick_image_path: Option<Rc<dyn Fn() -> Option<String>>>,
    pub request_open_audio: Option<Rc<dyn Fn(OpenAudioRequested)>>,
    pub request_open_image: Option<Rc<dyn Fn(String)>>,
}

#[derive(Default)]
pub struct MainPageDependencies {
    pub action_handlers: MainPageActionHandlers,
    pub behaviors: Vec<Box<dyn Behavior<MainViewModel>>>,
}

pub struct MainPageBinding {
    view_model: Rc<RefCell<MainViewModel>>,
    action_handlers: MainPageActionHandlers,
}

impl MainPageBinding {
    pub fn new(deps: MainPageDependencies) -> Self {
        let provider = MainViewModelProvider::new(MainViewModelProviderDependencies {
            behaviors: deps.behaviors,
        });
        Self {
            view_model: provider.main_view_model(),
            action_handlers: deps.action_handlers,
        }
    }

    pub fn dispatch(&self, action: ChoreoMainAction) {
        let mut view_model = self.view_model.borrow_mut();
        view_model.dispatch(action);
        consume_outgoing_commands(&mut view_model, &self.action_handlers);
    }

    pub fn show_dialog(&self, command: ShowDialogCommand) {
        self.view_model.borrow_mut().show_dialog(command);
    }

    pub fn hide_dialog(&self, command: CloseDialogCommand) {
        self.view_model.borrow_mut().hide_dialog(command);
    }

    pub fn request_open_audio(&self, request: OpenAudioRequested) {
        let mut view_model = self.view_model.borrow_mut();
        enqueue_open_audio_request(&mut view_model, request);
        consume_outgoing_commands(&mut view_model, &self.action_handlers);
    }

    pub fn request_open_image(&self, request: OpenImageRequested) {
        let mut view_model = self.view_model.borrow_mut();
        view_model.request_open_image(request);
        consume_outgoing_commands(&mut view_model, &self.action_handlers);
    }

    pub fn open_svg_file(&self, command: OpenSvgFileCommand) {
        self.view_model.borrow_mut().open_svg_file(command);
    }

    pub fn view_model(&self) -> Rc<RefCell<MainViewModel>> {
        Rc::clone(&self.view_model)
    }
}
