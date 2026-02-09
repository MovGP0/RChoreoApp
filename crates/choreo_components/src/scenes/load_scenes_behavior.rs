use std::rc::Rc;
use std::time::Duration;

use crossbeam_channel::{Receiver, Sender};
use nject::injectable;
use slint::TimerMode;

use crate::behavior::{Behavior, CompositeDisposable, TimerDisposable};
use crate::global::GlobalStateActor;
use crate::logging::BehaviorLog;
use crate::observability::start_internal_span;

use super::mapper::SceneMapper;
use super::messages::{ReloadScenesCommand, SelectedSceneChangedEvent};
use super::scenes_view_model::{SceneViewModel, ScenesPaneViewModel};

#[injectable]
#[inject(
    |global_state: Rc<GlobalStateActor>,
     receiver: Receiver<ReloadScenesCommand>,
     selected_scene_changed_sender: Sender<SelectedSceneChangedEvent>| {
        Self::new(global_state, receiver, selected_scene_changed_sender)
    }
)]
pub struct LoadScenesBehavior {
    global_state: Rc<GlobalStateActor>,
    receiver: Receiver<ReloadScenesCommand>,
    selected_scene_changed_sender: Sender<SelectedSceneChangedEvent>,
}

impl LoadScenesBehavior {
    pub fn new(
        global_state: Rc<GlobalStateActor>,
        receiver: Receiver<ReloadScenesCommand>,
        selected_scene_changed_sender: Sender<SelectedSceneChangedEvent>,
    ) -> Self {
        Self {
            global_state,
            receiver,
            selected_scene_changed_sender,
        }
    }

    fn load(
        global_state: &Rc<GlobalStateActor>,
        view_model: &mut ScenesPaneViewModel,
    ) -> Option<SceneViewModel> {
        let mut span = start_internal_span("scenes.load_scenes", None);
        let scenes = global_state.try_with_state(|global_state| {
            let mapper = SceneMapper;
            global_state
                .choreography
                .scenes
                .iter()
                .map(|scene| {
                    let mut view_model = SceneViewModel::new(
                        scene.scene_id,
                        scene.name.clone(),
                        scene.color.clone(),
                    );
                    mapper.map_model_to_view_model(scene, &mut view_model);
                    view_model
                })
                .collect::<Vec<_>>()
        })?;
        span.set_f64_attribute("choreo.scenes.count", scenes.len() as f64);

        let selected_scene = scenes.first().cloned();
        let updated = global_state.try_update(|global_state| {
            global_state.scenes = scenes;
            global_state.selected_scene = selected_scene.clone();
        });
        if !updated {
            span.set_bool_attribute("choreo.success", false);
            return None;
        }
        span.set_bool_attribute("choreo.success", true);

        view_model.refresh_scenes();
        view_model.set_selected_scene(selected_scene);
        let selected = view_model.selected_scene();
        if let Some(scene) = selected.as_ref() {
            span.set_string_attribute("choreo.scene.id", format!("{:?}", scene.scene_id));
        }
        selected
    }
}

impl Behavior<ScenesPaneViewModel> for LoadScenesBehavior {
    fn activate(
        &self,
        view_model: &mut ScenesPaneViewModel,
        disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated("LoadScenesBehavior", "ScenesPaneViewModel");
        let selected_scene = Self::load(&self.global_state, view_model);
        if let Some(selected_scene) = selected_scene {
            let _ = self
                .selected_scene_changed_sender
                .send(SelectedSceneChangedEvent {
                    selected_scene: Some(selected_scene),
                });
        }

        let Some(view_model_handle) = view_model.self_handle().and_then(|handle| handle.upgrade())
        else {
            return;
        };

        let receiver = self.receiver.clone();
        let selected_scene_changed_sender = self.selected_scene_changed_sender.clone();
        let global_state = self.global_state.clone();
        let timer = slint::Timer::default();
        timer.start(TimerMode::Repeated, Duration::from_millis(16), move || {
            while receiver.try_recv().is_ok() {
                let _span = start_internal_span("scenes.reload_scenes.command_handled", None);
                let mut view_model = view_model_handle.borrow_mut();
                let selected_scene = Self::load(&global_state, &mut view_model);
                if let Some(selected_scene) = selected_scene {
                    let _ = selected_scene_changed_sender.send(SelectedSceneChangedEvent {
                        selected_scene: Some(selected_scene),
                    });
                }
            }
        });
        disposables.add(Box::new(TimerDisposable::new(timer)));
    }
}
