use std::rc::Rc;
use std::time::Duration;

use crossbeam_channel::Receiver;
use nject::injectable;
use slint::TimerMode;

use crate::behavior::{Behavior, CompositeDisposable, TimerDisposable};
use crate::global::{GlobalStateActor, GlobalStateModel};
use crate::logging::BehaviorLog;
use crate::time::format_seconds;

use super::audio_player_linking::{build_tick_values, can_link_scene, try_get_linked_timestamp};
use super::audio_player_view_model::AudioPlayerViewModel;
use super::messages::LinkSceneToPositionCommand;

#[injectable]
#[inject(
    |global_state: Rc<GlobalStateActor>,
     receiver: Receiver<LinkSceneToPositionCommand>| {
        Self::new(global_state, receiver)
    }
)]
pub struct AudioPlayerLinkSceneBehavior {
    global_state: Rc<GlobalStateActor>,
    receiver: Receiver<LinkSceneToPositionCommand>,
}

impl AudioPlayerLinkSceneBehavior {
    pub fn new(
        global_state: Rc<GlobalStateActor>,
        receiver: Receiver<LinkSceneToPositionCommand>,
    ) -> Self {
        Self {
            global_state,
            receiver,
        }
    }
}

impl Behavior<AudioPlayerViewModel> for AudioPlayerLinkSceneBehavior {
    fn activate(
        &self,
        view_model: &mut AudioPlayerViewModel,
        disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated("AudioPlayerLinkSceneBehavior", "AudioPlayerViewModel");
        let Some(view_model_handle) = view_model.self_handle().and_then(|handle| handle.upgrade())
        else {
            return;
        };
        let global_state = Rc::clone(&self.global_state);
        let receiver = self.receiver.clone();
        let timer = slint::Timer::default();
        timer.start(TimerMode::Repeated, Duration::from_millis(16), move || {
            while receiver.try_recv().is_ok() {
                let Ok(mut view_model) = view_model_handle.try_borrow_mut() else {
                    continue;
                };
                let updated = global_state.try_update(|global_state| {
                    handle_link_scene_to_position(&mut view_model, global_state);
                });
                if !updated {
                    return;
                }
            }
        });
        disposables.add(Box::new(TimerDisposable::new(timer)));
    }
}

fn handle_link_scene_to_position(
    view_model: &mut AudioPlayerViewModel,
    global_state: &mut GlobalStateModel,
) {
    let Some(selected_scene) = global_state.selected_scene.as_mut() else {
        return;
    };

    let Some(rounded_timestamp) =
        try_get_linked_timestamp(view_model.position, selected_scene, &global_state.scenes)
    else {
        return;
    };

    selected_scene.timestamp = Some(rounded_timestamp);
    if let Some(scene) = global_state
        .scenes
        .iter_mut()
        .find(|scene| scene.scene_id == selected_scene.scene_id)
    {
        scene.timestamp = Some(rounded_timestamp);
    }

    if let Some(model_scene) = global_state
        .choreography
        .scenes
        .iter_mut()
        .find(|scene| scene.scene_id == selected_scene.scene_id)
    {
        model_scene.timestamp = Some(format_seconds(rounded_timestamp));
    }

    view_model.tick_values = build_tick_values(view_model.duration, &global_state.scenes);
    view_model.can_link_scene_to_position = can_link_scene(
        view_model.position,
        global_state.selected_scene.as_ref(),
        &global_state.scenes,
    );
}
