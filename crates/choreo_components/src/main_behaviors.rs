use std::cell::RefCell;
use std::rc::Rc;

use crossbeam_channel::{Receiver, Sender};
use choreo_state_machine::{
    ApplicationStateMachine, MovePositionsCompletedTrigger, MovePositionsStartedTrigger,
    RotateAroundCenterCompletedTrigger, RotateAroundCenterSelectionCompletedTrigger,
    RotateAroundCenterStartedTrigger, ScaleAroundDancerCompletedTrigger,
    ScaleAroundDancerSelectionCompletedTrigger, ScaleAroundDancerStartedTrigger,
    ScalePositionsCompletedTrigger, ScalePositionsSelectionCompletedTrigger,
    ScalePositionsStartedTrigger,
};

use crate::audio_player::OpenAudioFileCommand;
use crate::global::{GlobalStateModel, InteractionMode};
use crate::main_view_model::MainViewModel;
use crate::preferences::Preferences;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CloseDialogCommand;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenSvgFileCommand {
    pub file_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShowDialogCommand {
    pub content: Option<String>,
}

pub struct ApplyInteractionModeBehavior {
    global_state: Rc<RefCell<GlobalStateModel>>,
    state_machine: Rc<RefCell<ApplicationStateMachine>>,
}

impl ApplyInteractionModeBehavior {
    pub fn new(
        global_state: Rc<RefCell<GlobalStateModel>>,
        state_machine: Rc<RefCell<ApplicationStateMachine>>,
    ) -> Self {
        Self {
            global_state,
            state_machine,
        }
    }

    pub fn apply_mode(&self, mode: InteractionMode) {
        let selected_positions = self.global_state.borrow().selected_positions.len();
        let mut state_machine = self.state_machine.borrow_mut();

        match mode {
            InteractionMode::Move => {
                state_machine.try_apply(&RotateAroundCenterCompletedTrigger);
                state_machine.try_apply(&ScalePositionsCompletedTrigger);
                state_machine.try_apply(&ScaleAroundDancerCompletedTrigger);
                state_machine.try_apply(&MovePositionsStartedTrigger);
            }
            InteractionMode::RotateAroundCenter if selected_positions == 0 => {
                state_machine.try_apply(&ScalePositionsCompletedTrigger);
                state_machine.try_apply(&ScaleAroundDancerCompletedTrigger);
                state_machine.try_apply(&RotateAroundCenterStartedTrigger);
            }
            InteractionMode::RotateAroundCenter => {
                state_machine.try_apply(&ScalePositionsCompletedTrigger);
                state_machine.try_apply(&ScaleAroundDancerCompletedTrigger);
                state_machine.try_apply(&RotateAroundCenterStartedTrigger);
                state_machine.try_apply(&RotateAroundCenterSelectionCompletedTrigger);
            }
            InteractionMode::RotateAroundDancer if selected_positions == 0 => {
                state_machine.try_apply(&RotateAroundCenterCompletedTrigger);
                state_machine.try_apply(&ScalePositionsCompletedTrigger);
                state_machine.try_apply(&ScaleAroundDancerStartedTrigger);
            }
            InteractionMode::RotateAroundDancer => {
                state_machine.try_apply(&RotateAroundCenterCompletedTrigger);
                state_machine.try_apply(&ScalePositionsCompletedTrigger);
                state_machine.try_apply(&ScaleAroundDancerStartedTrigger);
                state_machine.try_apply(&ScaleAroundDancerSelectionCompletedTrigger);
            }
            InteractionMode::Scale if selected_positions == 0 => {
                state_machine.try_apply(&RotateAroundCenterCompletedTrigger);
                state_machine.try_apply(&ScaleAroundDancerCompletedTrigger);
                state_machine.try_apply(&ScalePositionsStartedTrigger);
            }
            InteractionMode::Scale => {
                state_machine.try_apply(&RotateAroundCenterCompletedTrigger);
                state_machine.try_apply(&ScaleAroundDancerCompletedTrigger);
                state_machine.try_apply(&ScalePositionsStartedTrigger);
                state_machine.try_apply(&ScalePositionsSelectionCompletedTrigger);
            }
            _ => {
                state_machine.try_apply(&MovePositionsCompletedTrigger);
                state_machine.try_apply(&RotateAroundCenterCompletedTrigger);
                state_machine.try_apply(&ScalePositionsCompletedTrigger);
                state_machine.try_apply(&ScaleAroundDancerCompletedTrigger);
            }
        }
    }
}

pub struct MainBehaviorDependencies<P: Preferences> {
    pub global_state: Rc<RefCell<GlobalStateModel>>,
    pub state_machine: Rc<RefCell<ApplicationStateMachine>>,
    pub open_audio_sender: Sender<OpenAudioFileCommand>,
    pub open_svg_sender: Sender<OpenSvgFileCommand>,
    pub open_svg_receiver: Receiver<OpenSvgFileCommand>,
    pub show_dialog_receiver: Receiver<ShowDialogCommand>,
    pub close_dialog_receiver: Receiver<CloseDialogCommand>,
    pub preferences: P,
}

pub struct MainBehaviors<P: Preferences> {
    pub apply_interaction_mode: ApplyInteractionModeBehavior,
    pub open_audio: OpenAudioBehavior,
    pub open_image: OpenImageBehavior,
    pub open_svg_file: OpenSvgFileBehavior<P>,
    pub show_dialog: ShowDialogBehavior,
    pub hide_dialog: HideDialogBehavior,
}

pub fn build_main_behaviors<P: Preferences>(deps: MainBehaviorDependencies<P>) -> MainBehaviors<P> {
    MainBehaviors {
        apply_interaction_mode: ApplyInteractionModeBehavior::new(
            deps.global_state.clone(),
            deps.state_machine,
        ),
        open_audio: OpenAudioBehavior::new(deps.open_audio_sender),
        open_image: OpenImageBehavior::new(deps.open_svg_sender),
        open_svg_file: OpenSvgFileBehavior::new(
            deps.global_state,
            deps.preferences,
            deps.open_svg_receiver,
        ),
        show_dialog: ShowDialogBehavior::new(deps.show_dialog_receiver),
        hide_dialog: HideDialogBehavior::new(deps.close_dialog_receiver),
    }
}

pub struct HideDialogBehavior {
    receiver: Receiver<CloseDialogCommand>,
}

impl HideDialogBehavior {
    pub fn new(receiver: Receiver<CloseDialogCommand>) -> Self {
        Self { receiver }
    }

    pub fn try_handle(&self, view_model: &mut MainViewModel) -> bool {
        match self.receiver.try_recv() {
            Ok(_) => {
                view_model.is_dialog_open = false;
                view_model.dialog_content = None;
                true
            }
            Err(_) => false,
        }
    }
}

pub struct ShowDialogBehavior {
    receiver: Receiver<ShowDialogCommand>,
}

impl ShowDialogBehavior {
    pub fn new(receiver: Receiver<ShowDialogCommand>) -> Self {
        Self { receiver }
    }

    pub fn try_handle(&self, view_model: &mut MainViewModel) -> bool {
        match self.receiver.try_recv() {
            Ok(command) => {
                view_model.dialog_content = command.content.clone();
                view_model.is_dialog_open = command.content.is_some();
                true
            }
            Err(_) => false,
        }
    }
}

pub struct OpenAudioBehavior {
    sender: Sender<OpenAudioFileCommand>,
}

impl OpenAudioBehavior {
    pub fn new(sender: Sender<OpenAudioFileCommand>) -> Self {
        Self { sender }
    }

    pub fn open_audio(&self, view_model: &mut MainViewModel, path: String) {
        let _ = self.sender.send(OpenAudioFileCommand { file_path: path });
        view_model.is_audio_player_open = true;
    }
}

pub struct OpenImageBehavior {
    sender: Sender<OpenSvgFileCommand>,
}

impl OpenImageBehavior {
    pub fn new(sender: Sender<OpenSvgFileCommand>) -> Self {
        Self { sender }
    }

    pub fn open_svg(&self, path: String) {
        let _ = self.sender.send(OpenSvgFileCommand { file_path: path });
    }
}

pub struct OpenSvgFileBehavior<P: Preferences> {
    global_state: Rc<RefCell<GlobalStateModel>>,
    preferences: P,
    receiver: Receiver<OpenSvgFileCommand>,
}

impl<P: Preferences> OpenSvgFileBehavior<P> {
    pub fn new(
        global_state: Rc<RefCell<GlobalStateModel>>,
        preferences: P,
        receiver: Receiver<OpenSvgFileCommand>,
    ) -> Self {
        Self {
            global_state,
            preferences,
            receiver,
        }
    }

    pub fn try_handle(&self) -> bool {
        match self.receiver.try_recv() {
            Ok(command) => {
                let mut global_state = self.global_state.borrow_mut();
                global_state.svg_file_path = Some(command.file_path.clone());
                self.preferences.set_string(
                    choreo_models::SettingsPreferenceKeys::LAST_OPENED_SVG_FILE,
                    command.file_path,
                );
                true
            }
            Err(_) => false,
        }
    }
}
