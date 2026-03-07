use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

use crate::behavior::Behavior;

use super::HideDialogBehavior;
use super::ShowDialogBehavior;
use super::actions::ChoreoMainAction;
use super::actions::OpenAudioRequested;
use super::actions::OpenChoreoRequested;
use super::behavior_pipeline::MainBehaviorDependencies;
use super::behavior_pipeline::MainBehaviorPipeline;
use super::main_view_model::MainViewModel;
use super::main_view_model_provider::MainViewModelProvider;
use super::main_view_model_provider::MainViewModelProviderDependencies;
use super::messages::CloseDialogCommand;
use super::messages::OpenImageRequested;
use super::messages::OpenSvgFileCommand;
use super::messages::ShowDialogCommand;
use super::runtime::consume_outgoing_commands;
use super::runtime::enqueue_open_audio_request;
use super::runtime::enqueue_open_image_request;

#[derive(Clone, Default)]
pub struct MainPageActionHandlers {
    pub pick_choreo_file: Option<Rc<dyn Fn() -> Option<OpenChoreoRequested>>>,
    pub pick_audio_path: Option<Rc<dyn Fn() -> Option<String>>>,
    pub pick_image_path: Option<Rc<dyn Fn() -> Option<String>>>,
    pub request_open_choreo: Option<Rc<dyn Fn(OpenChoreoRequested)>>,
    pub request_open_audio: Option<Rc<dyn Fn(OpenAudioRequested)>>,
    pub request_open_image: Option<Rc<dyn Fn(String)>>,
}

#[derive(Default)]
pub struct MainPageDependencies {
    pub action_handlers: MainPageActionHandlers,
    pub behaviors: Vec<Box<dyn Behavior<MainViewModel>>>,
    pub behavior_dependencies: MainBehaviorDependencies,
}

pub struct MainPageBinding {
    view_model: Rc<RefCell<MainViewModel>>,
    action_handlers: MainPageActionHandlers,
    behavior_pipeline: MainBehaviorPipeline,
}

impl MainPageBinding {
    pub fn new(deps: MainPageDependencies) -> Self {
        let MainPageDependencies {
            action_handlers,
            behaviors,
            behavior_dependencies,
        } = deps;
        let provider = MainViewModelProvider::new(MainViewModelProviderDependencies {
            behaviors,
            behavior_dependencies,
        });
        Self {
            view_model: provider.main_view_model(),
            action_handlers,
            behavior_pipeline: provider.behavior_pipeline().clone(),
        }
    }

    pub fn dispatch(&self, action: ChoreoMainAction) {
        let mut view_model = self.view_model.borrow_mut();
        let should_apply_interaction_mode = matches!(
            action,
            ChoreoMainAction::ApplyInteractionMode { .. } | ChoreoMainAction::SelectMode { .. }
        );
        let mode_to_apply = match &action {
            ChoreoMainAction::ApplyInteractionMode { mode, .. } => Some(*mode),
            ChoreoMainAction::SelectMode { .. } => None,
            _ => None,
        };

        view_model.dispatch(action);

        if should_apply_interaction_mode
            && let Some(behavior) = self
                .behavior_pipeline
                .apply_interaction_mode_behavior
                .as_ref()
        {
            if let Some(mode) = mode_to_apply {
                behavior.apply(map_interaction_mode(mode));
            } else {
                let current_mode = view_model.state().interaction_mode;
                behavior.apply(map_interaction_mode(current_mode));
            }
        }

        consume_outgoing_commands(
            &mut view_model,
            &self.action_handlers,
            &self.behavior_pipeline,
        );
    }

    pub fn show_dialog(&self, command: ShowDialogCommand) {
        let mut view_model = self.view_model.borrow_mut();
        ShowDialogBehavior::apply(&mut view_model, command);
    }

    pub fn hide_dialog(&self, command: CloseDialogCommand) {
        let mut view_model = self.view_model.borrow_mut();
        HideDialogBehavior::apply(&mut view_model, command);
    }

    pub fn request_open_audio(&self, request: OpenAudioRequested) {
        let mut view_model = self.view_model.borrow_mut();
        enqueue_open_audio_request(&mut view_model, request);
        consume_outgoing_commands(
            &mut view_model,
            &self.action_handlers,
            &self.behavior_pipeline,
        );
    }

    pub fn request_open_image(&self, request: OpenImageRequested) {
        let mut view_model = self.view_model.borrow_mut();
        enqueue_open_image_request(&mut view_model, request, &self.behavior_pipeline);
        consume_outgoing_commands(
            &mut view_model,
            &self.action_handlers,
            &self.behavior_pipeline,
        );
    }

    pub fn open_svg_file(&self, command: OpenSvgFileCommand) {
        let mut view_model = self.view_model.borrow_mut();
        if let Some(behavior) = self.behavior_pipeline.open_svg_file_behavior.as_ref() {
            behavior.apply(&mut view_model, command);
            return;
        }

        view_model.open_svg_file(command);
    }

    pub fn route_external_file_path(&self, file_path: &str) {
        if file_path.trim().is_empty() {
            return;
        }

        let extension = Path::new(file_path)
            .extension()
            .and_then(|value| value.to_str())
            .unwrap_or_default()
            .to_ascii_lowercase();

        if extension == "choreo" {
            let Ok(contents) = std::fs::read_to_string(file_path) else {
                return;
            };
            let file_name = Path::new(file_path)
                .file_name()
                .map(|value| value.to_string_lossy().into_owned());
            let command = OpenChoreoRequested {
                file_path: Some(file_path.to_string()),
                file_name,
                contents,
            };
            let mut view_model = self.view_model.borrow_mut();
            view_model.dispatch(ChoreoMainAction::RequestOpenChoreo(command));
            consume_outgoing_commands(
                &mut view_model,
                &self.action_handlers,
                &self.behavior_pipeline,
            );
            return;
        }

        if extension == "svg" {
            self.request_open_image(OpenImageRequested {
                file_path: file_path.to_string(),
            });
            return;
        }

        if extension == "mp3" {
            self.request_open_audio(OpenAudioRequested {
                file_path: file_path.to_string(),
                trace_context: None,
            });
        }
    }

    pub fn view_model(&self) -> Rc<RefCell<MainViewModel>> {
        Rc::clone(&self.view_model)
    }
}

fn map_interaction_mode(mode: super::state::InteractionMode) -> crate::global::InteractionMode {
    match mode {
        super::state::InteractionMode::View => crate::global::InteractionMode::View,
        super::state::InteractionMode::Move => crate::global::InteractionMode::Move,
        super::state::InteractionMode::RotateAroundCenter => {
            crate::global::InteractionMode::RotateAroundCenter
        }
        super::state::InteractionMode::RotateAroundDancer => {
            crate::global::InteractionMode::RotateAroundDancer
        }
        super::state::InteractionMode::Scale => crate::global::InteractionMode::Scale,
        super::state::InteractionMode::LineOfSight => crate::global::InteractionMode::LineOfSight,
    }
}
