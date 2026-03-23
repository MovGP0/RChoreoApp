use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc::SyncSender;

use choreo_state_machine::ApplicationStateMachine;
use crossbeam_channel::Sender;

use crate::audio_player::OpenAudioFileCommand;
use crate::floor::DrawFloorCommand;
use crate::global::GlobalStateActor;
use crate::preferences::Preferences;

use super::apply_interaction_mode_behavior::ApplyInteractionModeBehavior;
use super::hide_dialog_behavior::HideDialogBehavior;
use super::open_audio_behavior::OpenAudioBehavior;
use super::open_image_behavior::OpenImageBehavior;
use super::open_svg_file_behavior::OpenSvgFileBehavior;
use super::show_dialog_behavior::ShowDialogBehavior;

#[derive(Clone, Default)]
pub struct MainBehaviorDependencies {
    pub global_state_store: Option<Rc<GlobalStateActor>>,
    pub state_machine: Option<Rc<RefCell<ApplicationStateMachine>>>,
    pub open_audio_sender: Option<Sender<OpenAudioFileCommand>>,
    pub preferences: Option<Rc<dyn Preferences>>,
    pub draw_floor_sender: Option<SyncSender<DrawFloorCommand>>,
}

#[derive(Clone, Default)]
pub struct MainBehaviorPipeline {
    pub apply_interaction_mode_behavior: Option<ApplyInteractionModeBehavior>,
    pub open_audio_behavior: Option<OpenAudioBehavior>,
    pub open_image_behavior: Option<OpenImageBehavior>,
    pub open_svg_file_behavior: Option<OpenSvgFileBehavior>,
    pub show_dialog_behavior: ShowDialogBehavior,
    pub hide_dialog_behavior: HideDialogBehavior,
}

impl MainBehaviorPipeline {
    pub fn from_dependencies(deps: MainBehaviorDependencies) -> Self {
        let apply_interaction_mode_behavior = deps.global_state_store.as_ref().and_then(|store| {
            deps.state_machine.as_ref().map(|state_machine| {
                ApplyInteractionModeBehavior::new(Rc::clone(store), Rc::clone(state_machine))
            })
        });
        let open_audio_behavior = deps.open_audio_sender.map(OpenAudioBehavior::new);
        let open_svg_file_behavior = deps.global_state_store.as_ref().and_then(|store| {
            deps.preferences.as_ref().and_then(|preferences| {
                deps.draw_floor_sender.as_ref().map(|draw_floor_sender| {
                    OpenSvgFileBehavior::new(
                        Rc::clone(store),
                        Rc::clone(preferences),
                        draw_floor_sender.clone(),
                    )
                })
            })
        });

        Self {
            apply_interaction_mode_behavior,
            open_audio_behavior,
            open_image_behavior: Some(OpenImageBehavior),
            open_svg_file_behavior,
            show_dialog_behavior: ShowDialogBehavior,
            hide_dialog_behavior: HideDialogBehavior,
        }
    }
}
