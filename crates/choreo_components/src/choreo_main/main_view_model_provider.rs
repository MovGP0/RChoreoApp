use std::cell::RefCell;
use std::rc::Rc;

use crossbeam_channel::{bounded, unbounded, Sender};

use choreo_state_machine::ApplicationStateMachine;

use crate::audio_player::{AudioPlayerViewModel, OpenAudioFileCommand};
use crate::behavior::Behavior;
use crate::global::{GlobalStateModel, InteractionMode};
use crate::preferences::Preferences;

use super::messages::{OpenAudioRequested, OpenImageRequested};
use super::{
    ApplyInteractionModeBehavior,
    HideDialogBehavior,
    MainViewModel,
    MainViewModelActions,
    OpenAudioBehavior,
    OpenImageBehavior,
    OpenSvgFileBehavior,
    ShowDialogBehavior,
};

pub struct MainViewModelProviderDependencies {
    pub global_state: Rc<RefCell<GlobalStateModel>>,
    pub state_machine: Rc<RefCell<ApplicationStateMachine>>,
    pub audio_player: Rc<RefCell<AudioPlayerViewModel>>,
    pub open_audio_sender: Sender<OpenAudioFileCommand>,
    pub preferences: Rc<dyn Preferences>,
    pub draw_floor_sender: Sender<crate::floor::DrawFloorCommand>,
    pub haptic_feedback: Option<Box<dyn crate::haptics::HapticFeedback>>,
}

pub struct MainViewModelProvider {
    global_state: Rc<RefCell<GlobalStateModel>>,
    state_machine: Rc<RefCell<ApplicationStateMachine>>,
    audio_player: Rc<RefCell<AudioPlayerViewModel>>,
    interaction_mode_sender: Sender<InteractionMode>,
    open_audio_request_sender: Sender<OpenAudioRequested>,
    open_image_request_sender: Sender<OpenImageRequested>,
    main_behaviors: Vec<Box<dyn Behavior<MainViewModel>>>,
    haptic_feedback: Option<Box<dyn crate::haptics::HapticFeedback>>,
}

impl MainViewModelProvider {
    pub fn new(deps: MainViewModelProviderDependencies) -> Self
    {
        const MAIN_EVENT_BUFFER: usize = 64;
        let (interaction_mode_sender, interaction_mode_receiver) = bounded(MAIN_EVENT_BUFFER);
        let (open_audio_request_sender, open_audio_request_receiver) = bounded(MAIN_EVENT_BUFFER);
        let (open_image_request_sender, open_image_request_receiver) = bounded(MAIN_EVENT_BUFFER);
        let (open_svg_sender, open_svg_receiver) = unbounded();
        let (_show_dialog_sender, show_dialog_receiver) = unbounded();
        let (_close_dialog_sender, close_dialog_receiver) = unbounded();
        let main_behaviors: Vec<Box<dyn Behavior<MainViewModel>>> = vec![
            Box::new(ApplyInteractionModeBehavior::new(
                Rc::clone(&deps.global_state),
                Rc::clone(&deps.state_machine),
                interaction_mode_receiver.clone(),
            )),
            Box::new(OpenAudioBehavior::new(
                deps.open_audio_sender.clone(),
                open_audio_request_receiver.clone(),
            )),
            Box::new(OpenImageBehavior::new(
                open_svg_sender.clone(),
                open_image_request_receiver.clone(),
            )),
            Box::new(OpenSvgFileBehavior::new(
                Rc::clone(&deps.global_state),
                Rc::clone(&deps.preferences),
                open_svg_receiver.clone(),
                deps.draw_floor_sender.clone(),
            )),
            Box::new(ShowDialogBehavior::new(show_dialog_receiver.clone())),
            Box::new(HideDialogBehavior::new(close_dialog_receiver.clone())),
        ];

        Self {
            global_state: deps.global_state,
            state_machine: deps.state_machine,
            audio_player: deps.audio_player,
            interaction_mode_sender,
            open_audio_request_sender,
            open_image_request_sender,
            main_behaviors,
            haptic_feedback: deps.haptic_feedback,
        }
    }

    pub fn create_main_view_model(&mut self) -> MainViewModel {
        let behaviors = std::mem::take(&mut self.main_behaviors);
        let haptic_feedback = self.haptic_feedback.take();
        MainViewModel::new(
            Rc::clone(&self.global_state),
            Rc::clone(&self.state_machine),
            Rc::clone(&self.audio_player),
            haptic_feedback,
            behaviors,
            MainViewModelActions::default(),
        )
    }

    pub fn interaction_mode_sender(&self) -> Sender<InteractionMode> {
        self.interaction_mode_sender.clone()
    }

    pub fn open_audio_request_sender(&self) -> Sender<OpenAudioRequested> {
        self.open_audio_request_sender.clone()
    }

    pub fn open_image_request_sender(&self) -> Sender<OpenImageRequested> {
        self.open_image_request_sender.clone()
    }
}
