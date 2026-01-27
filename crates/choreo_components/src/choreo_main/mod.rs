mod apply_interaction_mode_behavior;
mod hide_dialog_behavior;
mod main_page_binding;
mod main_view_model;
mod messages;
mod open_audio_behavior;
mod open_image_behavior;
mod open_svg_file_behavior;
mod show_dialog_behavior;

pub use apply_interaction_mode_behavior::ApplyInteractionModeBehavior;
pub use hide_dialog_behavior::HideDialogBehavior;
pub use main_page_binding::{MainPageActionHandlers, MainPageBinding, MainPageDependencies};
pub use main_view_model::{
    build_main_view_model, InteractionModeOption, MainDependencies, MainViewModel,
    MainViewModelActions,
};
pub use messages::{CloseDialogCommand, OpenSvgFileCommand, ShowDialogCommand};
pub use open_audio_behavior::OpenAudioBehavior;
pub use open_image_behavior::OpenImageBehavior;
pub use open_svg_file_behavior::OpenSvgFileBehavior;
pub use show_dialog_behavior::ShowDialogBehavior;

use std::cell::RefCell;
use std::rc::Rc;

use crossbeam_channel::{Receiver, Sender};
use choreo_state_machine::ApplicationStateMachine;

use crate::audio_player::OpenAudioFileCommand;
use crate::global::GlobalStateModel;
pub struct MainBehaviorDependencies {
    pub global_state: Rc<RefCell<GlobalStateModel>>,
    pub state_machine: Rc<RefCell<ApplicationStateMachine>>,
    pub open_audio_sender: Sender<OpenAudioFileCommand>,
    pub open_svg_sender: Sender<OpenSvgFileCommand>,
    pub open_svg_receiver: Receiver<OpenSvgFileCommand>,
    pub show_dialog_receiver: Receiver<ShowDialogCommand>,
    pub close_dialog_receiver: Receiver<CloseDialogCommand>,
    pub preferences: Rc<dyn crate::preferences::Preferences>,
}

pub struct MainBehaviors {
    pub apply_interaction_mode: ApplyInteractionModeBehavior,
    pub open_audio: OpenAudioBehavior,
    pub open_image: OpenImageBehavior,
    pub open_svg_file: OpenSvgFileBehavior,
    pub show_dialog: ShowDialogBehavior,
    pub hide_dialog: HideDialogBehavior,
}

pub fn build_main_behaviors(deps: MainBehaviorDependencies) -> MainBehaviors {
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
