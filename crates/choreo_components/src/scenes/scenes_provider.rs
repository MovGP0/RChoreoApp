use std::cell::RefCell;
use std::rc::Rc;

use choreo_state_machine::ApplicationStateMachine;
use crossbeam_channel::Sender;

use crate::audio_player::AudioPlayerPositionChangedEvent;
use crate::audio_player::{CloseAudioFileCommand, OpenAudioFileCommand};
use crate::behavior::Behavior;
use crate::choreography_settings::RedrawFloorCommand;
use crate::choreography_settings::ShowTimestampsChangedEvent;
use crate::global::{GlobalStateActor, GlobalStateModel};

use super::OpenChoreoRequested;
use super::apply_placement_mode_behavior::ApplyPlacementModeBehavior;
use super::filter_scenes_behavior::FilterScenesBehavior;
use super::insert_scene_behavior::InsertSceneBehavior;
use super::load_scenes_behavior::LoadScenesBehavior;
use super::messages::{CloseDialogCommand, ShowDialogCommand};
use super::open_choreo_behavior::{
    OpenChoreoActions, OpenChoreoBehavior, OpenChoreoBehaviorDependencies,
};
use super::save_choreo_behavior::SaveChoreoBehavior;
use super::scenes_view_model::ScenesPaneViewModel;
use super::select_scene_behavior::SelectSceneBehavior;
use super::select_scene_from_audio_position_behavior::SelectSceneFromAudioPositionBehavior;
use super::show_scene_timestamps_behavior::ShowSceneTimestampsBehavior;

pub struct ScenesDependencies {
    pub global_state: Rc<RefCell<GlobalStateModel>>,
    pub global_state_store: Rc<GlobalStateActor>,
    pub state_machine: Option<Rc<RefCell<ApplicationStateMachine>>>,
    pub preferences: Rc<dyn crate::preferences::Preferences>,
    pub show_dialog_sender: Sender<ShowDialogCommand>,
    pub close_dialog_sender: Sender<CloseDialogCommand>,
    pub haptic_feedback: Option<Box<dyn crate::haptics::HapticFeedback>>,
    pub open_audio_sender: Sender<OpenAudioFileCommand>,
    pub close_audio_sender: Sender<CloseAudioFileCommand>,
    pub audio_position_receiver: crossbeam_channel::Receiver<AudioPlayerPositionChangedEvent>,
    pub show_timestamps_sender: Sender<ShowTimestampsChangedEvent>,
    pub show_timestamps_receiver: crossbeam_channel::Receiver<ShowTimestampsChangedEvent>,
    pub redraw_floor_sender: Sender<RedrawFloorCommand>,
    pub actions: OpenChoreoActions,
}

pub struct ScenesProvider {
    scenes_view_model: Rc<RefCell<ScenesPaneViewModel>>,
    open_choreo_sender: Sender<OpenChoreoRequested>,
}

impl ScenesProvider {
    pub fn new(deps: ScenesDependencies) -> Self {
        let (select_scene_sender, select_scene_receiver) = crossbeam_channel::unbounded();
        let (selected_scene_changed_sender, selected_scene_changed_receiver) =
            crossbeam_channel::unbounded();
        let (reload_scenes_sender, reload_scenes_receiver) = crossbeam_channel::unbounded();
        let (open_choreo_sender, open_choreo_receiver) = crossbeam_channel::unbounded();

        let behaviors: Vec<Box<dyn Behavior<ScenesPaneViewModel>>> = vec![
            Box::new(LoadScenesBehavior::new(
                deps.global_state_store.clone(),
                reload_scenes_receiver,
                selected_scene_changed_sender.clone(),
            )),
            Box::new(FilterScenesBehavior),
            Box::new(InsertSceneBehavior::new(deps.global_state_store.clone())),
            Box::new(ShowSceneTimestampsBehavior::new(
                deps.global_state_store.clone(),
                deps.show_timestamps_receiver,
            )),
            Box::new(SelectSceneBehavior::new(
                select_scene_sender,
                select_scene_receiver,
                selected_scene_changed_sender.clone(),
                deps.redraw_floor_sender.clone(),
            )),
            Box::new(ApplyPlacementModeBehavior::new(
                deps.global_state_store.clone(),
                deps.state_machine.clone(),
                selected_scene_changed_receiver,
            )),
            Box::new(SelectSceneFromAudioPositionBehavior::new(
                deps.audio_position_receiver,
                selected_scene_changed_sender,
                deps.redraw_floor_sender.clone(),
            )),
            Box::new(OpenChoreoBehavior::new(OpenChoreoBehaviorDependencies {
                global_state: deps.global_state_store.clone(),
                preferences: deps.preferences.clone(),
                open_audio_sender: deps.open_audio_sender,
                close_audio_sender: deps.close_audio_sender,
                reload_scenes_sender,
                show_timestamps_sender: deps.show_timestamps_sender,
                actions: deps.actions,
                open_choreo_sender: open_choreo_sender.clone(),
                open_choreo_receiver,
            })),
            Box::new(SaveChoreoBehavior::new(
                deps.global_state_store.clone(),
                deps.preferences.clone(),
            )),
        ];

        let scenes_view_model = Rc::new(RefCell::new(ScenesPaneViewModel::new(
            deps.global_state,
            deps.preferences,
            deps.show_dialog_sender,
            deps.close_dialog_sender,
            deps.haptic_feedback,
        )));

        scenes_view_model
            .borrow_mut()
            .set_self_handle(Rc::downgrade(&scenes_view_model));
        ScenesPaneViewModel::activate(&scenes_view_model, behaviors);

        Self {
            scenes_view_model,
            open_choreo_sender,
        }
    }

    pub fn scenes_view_model(&self) -> Rc<RefCell<ScenesPaneViewModel>> {
        Rc::clone(&self.scenes_view_model)
    }

    pub fn open_choreo_sender(&self) -> Sender<OpenChoreoRequested> {
        self.open_choreo_sender.clone()
    }
}
