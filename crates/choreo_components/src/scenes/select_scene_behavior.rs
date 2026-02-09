use std::rc::Rc;
use std::time::Duration;

use crossbeam_channel::{Receiver, Sender};
use nject::injectable;
use slint::TimerMode;

use super::messages::{SelectSceneCommand, SelectedSceneChangedEvent};
use super::scenes_view_model::ScenesPaneViewModel;
use crate::behavior::{Behavior, CompositeDisposable, TimerDisposable};
use crate::choreography_settings::RedrawFloorCommand;
use crate::logging::BehaviorLog;
use crate::observability::start_internal_span;
use crate::scenes::SceneViewModel;

#[injectable]
#[inject(
    |select_scene_sender: Sender<SelectSceneCommand>,
     select_scene_receiver: Receiver<SelectSceneCommand>,
     selected_scene_changed_sender: Sender<SelectedSceneChangedEvent>,
     redraw_floor_sender: Sender<RedrawFloorCommand>| {
        Self::new(
            select_scene_sender,
            select_scene_receiver,
            selected_scene_changed_sender,
            redraw_floor_sender,
        )
    }
)]
pub struct SelectSceneBehavior {
    select_scene_sender: Sender<SelectSceneCommand>,
    receiver: Receiver<SelectSceneCommand>,
    selected_scene_changed_sender: Sender<SelectedSceneChangedEvent>,
    redraw_floor_sender: Sender<RedrawFloorCommand>,
}

impl SelectSceneBehavior {
    pub fn new(
        select_scene_sender: Sender<SelectSceneCommand>,
        receiver: Receiver<SelectSceneCommand>,
        selected_scene_changed_sender: Sender<SelectedSceneChangedEvent>,
        redraw_floor_sender: Sender<RedrawFloorCommand>,
    ) -> Self {
        Self {
            select_scene_sender,
            receiver,
            selected_scene_changed_sender,
            redraw_floor_sender,
        }
    }

    fn handle_selection(
        view_model: &mut ScenesPaneViewModel,
        index: usize,
    ) -> Option<SceneViewModel> {
        let scene = view_model.scenes.get(index).cloned()?;
        view_model.set_selected_scene(Some(scene));
        view_model.selected_scene()
    }
}

impl Behavior<ScenesPaneViewModel> for SelectSceneBehavior {
    fn activate(
        &self,
        view_model: &mut ScenesPaneViewModel,
        disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated("SelectSceneBehavior", "ScenesPaneViewModel");

        let Some(view_model_handle) = view_model.self_handle().and_then(|handle| handle.upgrade())
        else {
            return;
        };

        let select_scene_sender = self.select_scene_sender.clone();
        view_model.set_select_scene_handler(Some(Rc::new(move |_view_model, index| {
            let mut span = start_internal_span("scenes.select_scene.command_enqueued", None);
            span.set_f64_attribute("choreo.scenes.index", index as f64);
            let _ = select_scene_sender.send(SelectSceneCommand { index });
        })));

        let receiver = self.receiver.clone();
        let selected_scene_changed_sender = self.selected_scene_changed_sender.clone();
        let redraw_floor_sender = self.redraw_floor_sender.clone();
        let timer = slint::Timer::default();
        timer.start(TimerMode::Repeated, Duration::from_millis(16), move || {
            while let Ok(command) = receiver.try_recv() {
                let mut span = start_internal_span("scenes.select_scene.command_handled", None);
                span.set_f64_attribute("choreo.scenes.index", command.index as f64);
                let mut view_model = view_model_handle.borrow_mut();
                if let Some(selected_scene) = Self::handle_selection(&mut view_model, command.index)
                {
                    span.set_bool_attribute("choreo.success", true);
                    span.set_string_attribute(
                        "choreo.scene.id",
                        format!("{:?}", selected_scene.scene_id),
                    );
                    let _ = selected_scene_changed_sender.send(SelectedSceneChangedEvent {
                        selected_scene: Some(selected_scene),
                    });
                    let _ = redraw_floor_sender.send(RedrawFloorCommand);
                } else {
                    span.set_bool_attribute("choreo.success", false);
                }
            }
        });
        disposables.add(Box::new(TimerDisposable::new(timer)));
    }
}
