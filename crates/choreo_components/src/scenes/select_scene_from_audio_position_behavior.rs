use std::time::Duration;

use crossbeam_channel::{Receiver, Sender};
use nject::injectable;
use slint::TimerMode;

use crate::audio_player::AudioPlayerPositionChangedEvent;
use crate::behavior::{Behavior, CompositeDisposable, TimerDisposable};
use crate::choreography_settings::RedrawFloorCommand;
use crate::logging::BehaviorLog;
use crate::scenes::SelectedSceneChangedEvent;

use super::scenes_view_model::ScenesPaneViewModel;

#[injectable]
#[inject(
    |receiver: Receiver<AudioPlayerPositionChangedEvent>,
     selected_scene_changed_sender: Sender<SelectedSceneChangedEvent>,
     redraw_floor_sender: Sender<RedrawFloorCommand>| {
        Self::new(receiver, selected_scene_changed_sender, redraw_floor_sender)
    }
)]
pub struct SelectSceneFromAudioPositionBehavior {
    receiver: Receiver<AudioPlayerPositionChangedEvent>,
    selected_scene_changed_sender: Sender<SelectedSceneChangedEvent>,
    redraw_floor_sender: Sender<RedrawFloorCommand>,
}

impl SelectSceneFromAudioPositionBehavior {
    pub fn new(
        receiver: Receiver<AudioPlayerPositionChangedEvent>,
        selected_scene_changed_sender: Sender<SelectedSceneChangedEvent>,
        redraw_floor_sender: Sender<RedrawFloorCommand>,
    ) -> Self {
        Self {
            receiver,
            selected_scene_changed_sender,
            redraw_floor_sender,
        }
    }

    fn update_selection(view_model: &mut ScenesPaneViewModel, position_seconds: f64) -> bool {
        let previous_id = view_model.selected_scene().map(|scene| scene.scene_id);
        if view_model.scenes.is_empty() {
            return false;
        }

        for index in 0..view_model.scenes.len() {
            let current_scene = view_model.scenes[index].clone();
            let Some(current_timestamp) = current_scene.timestamp else {
                continue;
            };

            let next_index = index + 1;
            if next_index >= view_model.scenes.len() {
                return false;
            }

            let next_scene = view_model.scenes[next_index].clone();
            let Some(next_timestamp) = next_scene.timestamp else {
                continue;
            };
            if next_timestamp <= current_timestamp {
                continue;
            }

            if position_seconds >= current_timestamp && position_seconds <= next_timestamp {
                view_model.set_selected_scene(Some(current_scene.clone()));
                return previous_id != Some(current_scene.scene_id);
            }
        }

        false
    }
}

impl Behavior<ScenesPaneViewModel> for SelectSceneFromAudioPositionBehavior {
    fn activate(
        &self,
        view_model: &mut ScenesPaneViewModel,
        disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated(
            "SelectSceneFromAudioPositionBehavior",
            "ScenesPaneViewModel",
        );
        let Some(view_model_handle) = view_model.self_handle().and_then(|handle| handle.upgrade())
        else {
            return;
        };

        let receiver = self.receiver.clone();
        let selected_scene_changed_sender = self.selected_scene_changed_sender.clone();
        let redraw_floor_sender = self.redraw_floor_sender.clone();
        let timer = slint::Timer::default();
        timer.start(TimerMode::Repeated, Duration::from_millis(16), move || {
            while let Ok(event) = receiver.try_recv() {
                let mut view_model = view_model_handle.borrow_mut();
                let changed = Self::update_selection(&mut view_model, event.position_seconds);
                if changed {
                    let _ = selected_scene_changed_sender.send(SelectedSceneChangedEvent {
                        selected_scene: view_model.selected_scene(),
                    });
                    let _ = redraw_floor_sender.send(RedrawFloorCommand);
                }
            }
        });
        disposables.add(Box::new(TimerDisposable::new(timer)));
    }
}
