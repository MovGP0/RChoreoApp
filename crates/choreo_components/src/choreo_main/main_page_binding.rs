use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

use crate::audio_player::runtime::AudioPlayerRuntime;
use super::behaviors::ChoreoMainBehaviorDependencies;
use super::behaviors::ChoreoMainBehaviors;
use super::actions::ChoreoMainAction;
use super::actions::OpenAudioRequested;
use super::actions::OpenChoreoRequested;
use super::messages::CloseDialogCommand;
use super::messages::OpenImageRequested;
use super::messages::OpenSvgFileCommand;
use super::messages::ShowDialogCommand;
use super::reducer::reduce_with_behaviors;
use super::state::ChoreoMainState;
use super::runtime::apply_audio_action_side_effects;
use super::runtime::consume_outgoing_commands;
use super::runtime::enqueue_open_audio_request;
use super::runtime::poll_audio_runtime;

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
    pub behavior_dependencies: ChoreoMainBehaviorDependencies,
}

pub struct MainPageBinding {
    state: Rc<RefCell<ChoreoMainState>>,
    action_handlers: MainPageActionHandlers,
    behaviors: ChoreoMainBehaviors,
    audio_runtime: RefCell<AudioPlayerRuntime>,
}

impl MainPageBinding {
    pub fn new(deps: MainPageDependencies) -> Self {
        let MainPageDependencies {
            action_handlers,
            behavior_dependencies,
        } = deps;
        let behaviors = ChoreoMainBehaviors::from_dependencies(behavior_dependencies);
        let state = Rc::new(RefCell::new(ChoreoMainState::default()));
        let audio_backend = state.borrow().settings_state.audio_player_backend;
        Self {
            state,
            action_handlers,
            behaviors,
            audio_runtime: RefCell::new(AudioPlayerRuntime::new(audio_backend)),
        }
    }

    pub fn dispatch(&self, action: ChoreoMainAction) {
        let mut state = self.state.borrow_mut();
        let mut audio_runtime = self.audio_runtime.borrow_mut();
        reduce_with_behaviors(&mut state, action.clone(), Some(&self.behaviors));
        apply_audio_action_side_effects(&mut state, &mut audio_runtime, &action);

        consume_outgoing_commands(
            &mut state,
            &self.action_handlers,
            &self.behaviors,
            &mut audio_runtime,
        );
    }

    #[must_use]
    pub fn tick_audio_runtime(&self) -> bool {
        let mut state = self.state.borrow_mut();
        let mut audio_runtime = self.audio_runtime.borrow_mut();
        poll_audio_runtime(&mut state, &mut audio_runtime)
    }

    #[must_use]
    pub fn audio_runtime_is_active(&self) -> bool {
        let state = self.state.borrow();
        let audio_state = &state.audio_player_state;
        audio_state.has_player
            && (audio_state.is_playing || audio_state.pending_seek_position.is_some())
    }

    pub fn show_dialog(&self, command: ShowDialogCommand) {
        let mut state = self.state.borrow_mut();
        super::ShowDialogBehavior::apply(&mut state, command);
    }

    pub fn hide_dialog(&self, command: CloseDialogCommand) {
        let mut state = self.state.borrow_mut();
        super::HideDialogBehavior::apply(&mut state, command);
    }

    pub fn request_open_audio(&self, request: OpenAudioRequested) {
        let mut state = self.state.borrow_mut();
        let mut audio_runtime = self.audio_runtime.borrow_mut();
        enqueue_open_audio_request(&mut state, request);
        consume_outgoing_commands(
            &mut state,
            &self.action_handlers,
            &self.behaviors,
            &mut audio_runtime,
        );
    }

    pub fn request_open_image(&self, request: OpenImageRequested) {
        let mut state = self.state.borrow_mut();
        let mut audio_runtime = self.audio_runtime.borrow_mut();
        reduce_with_behaviors(
            &mut state,
            ChoreoMainAction::RequestOpenImage {
                file_path: request.file_path,
            },
            Some(&self.behaviors),
        );
        consume_outgoing_commands(
            &mut state,
            &self.action_handlers,
            &self.behaviors,
            &mut audio_runtime,
        );
    }

    pub fn open_svg_file(&self, command: OpenSvgFileCommand) {
        let mut state = self.state.borrow_mut();
        reduce_with_behaviors(&mut state, ChoreoMainAction::ApplyOpenSvgFile(command), Some(&self.behaviors));
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
            let mut state = self.state.borrow_mut();
            let mut audio_runtime = self.audio_runtime.borrow_mut();
            reduce_with_behaviors(
                &mut state,
                ChoreoMainAction::RequestOpenChoreo(command),
                Some(&self.behaviors),
            );
            consume_outgoing_commands(
                &mut state,
                &self.action_handlers,
                &self.behaviors,
                &mut audio_runtime,
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

    pub fn state(&self) -> Rc<RefCell<ChoreoMainState>> {
        Rc::clone(&self.state)
    }
}
