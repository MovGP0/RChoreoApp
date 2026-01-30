mod apply_placement_mode_behavior;
mod filter_scenes_behavior;
mod insert_scene_behavior;
mod load_scenes_behavior;
mod mapper;
mod messages;
mod open_choreo_behavior;
mod publish_scene_selected_behavior;
mod save_choreo_behavior;
mod scenes_view_model;
mod select_scene_behavior;
mod select_scene_from_audio_position_behavior;
mod show_scene_timestamps_behavior;
mod scenes_provider;

pub use apply_placement_mode_behavior::ApplyPlacementModeBehavior;
pub use filter_scenes_behavior::FilterScenesBehavior;
pub use insert_scene_behavior::InsertSceneBehavior;
pub use load_scenes_behavior::LoadScenesBehavior;
pub use mapper::SceneMapper;
pub use messages::{
    CloseDialogCommand,
    CopyScenePositionsDecision,
    CopyScenePositionsDecisionEvent,
    DialogRequest,
    SceneSelectedEvent,
    SelectedSceneChangedEvent,
    ShowDialogCommand,
};
pub use open_choreo_behavior::{OpenChoreoActions, OpenChoreoBehavior};
pub use publish_scene_selected_behavior::PublishSceneSelectedBehavior;
pub use save_choreo_behavior::SaveChoreoBehavior;
pub use scenes_view_model::{SceneViewModel, ScenesPaneViewModel};
pub use scenes_provider::ScenesProvider;
pub use select_scene_behavior::SelectSceneBehavior;
pub use select_scene_from_audio_position_behavior::SelectSceneFromAudioPositionBehavior;
pub use show_scene_timestamps_behavior::ShowSceneTimestampsBehavior;

use std::cell::RefCell;
use std::rc::Rc;

use crossbeam_channel::Sender;
use choreo_state_machine::ApplicationStateMachine;

use crate::audio_player::{CloseAudioFileCommand, OpenAudioFileCommand};
use crate::behavior::Behavior;
use crate::global::GlobalStateModel;
pub struct ScenesDependencies {
    pub global_state: Rc<RefCell<GlobalStateModel>>,
    pub state_machine: Option<Rc<RefCell<ApplicationStateMachine>>>,
    pub preferences: Rc<dyn crate::preferences::Preferences>,
    pub show_dialog_sender: Sender<ShowDialogCommand>,
    pub close_dialog_sender: Sender<CloseDialogCommand>,
    pub haptic_feedback: Option<Box<dyn crate::haptics::HapticFeedback>>,
    pub open_audio_sender: Sender<OpenAudioFileCommand>,
    pub close_audio_sender: Sender<CloseAudioFileCommand>,
    pub actions: OpenChoreoActions,
}

pub fn build_scenes_view_model(deps: ScenesDependencies) -> ScenesPaneViewModel {
    let (scene_selected_sender, scene_selected_receiver) = crossbeam_channel::unbounded();
    let (selected_scene_changed_sender, selected_scene_changed_receiver) = crossbeam_channel::unbounded();

    let behaviors: Vec<Box<dyn Behavior<ScenesPaneViewModel>>> = vec![
        Box::new(LoadScenesBehavior::new(deps.global_state.clone())),
        Box::new(FilterScenesBehavior),
        Box::new(InsertSceneBehavior::new(deps.global_state.clone())),
        Box::new(ShowSceneTimestampsBehavior::new(deps.global_state.clone())),
        Box::new(PublishSceneSelectedBehavior::new(
            scene_selected_sender.clone(),
        )),
        Box::new(SelectSceneBehavior::new(
            scene_selected_receiver,
            selected_scene_changed_sender,
            selected_scene_changed_receiver.clone(),
            deps.global_state.clone(),
            deps.state_machine.clone(),
            scene_selected_sender,
        )),
        Box::new(ApplyPlacementModeBehavior::new(
            deps.global_state.clone(),
            deps.state_machine.clone(),
            selected_scene_changed_receiver,
        )),
        Box::new(SelectSceneFromAudioPositionBehavior),
        Box::new(OpenChoreoBehavior::new(
            deps.global_state.clone(),
            deps.preferences.clone(),
            deps.open_audio_sender,
            deps.close_audio_sender,
            deps.actions,
        )),
        Box::new(SaveChoreoBehavior::new(
            deps.global_state.clone(),
            deps.preferences.clone(),
        )),
    ];

    ScenesPaneViewModel::new(
        deps.global_state,
        deps.preferences,
        deps.show_dialog_sender,
        deps.close_dialog_sender,
        deps.haptic_feedback,
        behaviors,
    )
}
