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
use super::open_audio_behavior::OpenAudioBehavior;
use super::open_choreo_file_behavior::OpenChoreoFileBehavior;
use super::open_svg_file_behavior::OpenSvgFileBehavior;

#[derive(Clone, Default)]
pub struct ChoreoMainBehaviorDependencies {
    pub global_state_store: Option<Rc<GlobalStateActor>>,
    pub state_machine: Option<Rc<RefCell<ApplicationStateMachine>>>,
    pub open_audio_sender: Option<Sender<OpenAudioFileCommand>>,
    pub preferences: Option<Rc<dyn Preferences>>,
    pub draw_floor_sender: Option<SyncSender<DrawFloorCommand>>,
}

#[derive(Clone, Default)]
pub struct ChoreoMainBehaviors {
    pub apply_interaction_mode: Option<ApplyInteractionModeBehavior>,
    pub open_audio: Option<OpenAudioBehavior>,
    pub open_choreo_file: Option<OpenChoreoFileBehavior>,
    pub open_svg_file: Option<OpenSvgFileBehavior>,
}

impl ChoreoMainBehaviors {
    #[must_use]
    pub fn from_dependencies(deps: ChoreoMainBehaviorDependencies) -> Self {
        let apply_interaction_mode = deps.global_state_store.as_ref().and_then(|store| {
            deps.state_machine.as_ref().map(|state_machine| {
                ApplyInteractionModeBehavior::new(Rc::clone(store), Rc::clone(state_machine))
            })
        });
        let open_audio = deps.open_audio_sender.map(OpenAudioBehavior::new);
        let open_choreo_file = deps.preferences.as_ref().map(|preferences| {
            OpenChoreoFileBehavior::new(Rc::clone(preferences))
        });
        let open_svg_file = deps.global_state_store.as_ref().and_then(|store| {
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
            apply_interaction_mode,
            open_audio,
            open_choreo_file,
            open_svg_file,
        }
    }
}
