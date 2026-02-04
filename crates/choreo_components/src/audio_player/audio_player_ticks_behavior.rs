use std::rc::Rc;
use std::time::Duration;

use nject::injectable;
use slint::TimerMode;

use crate::behavior::{Behavior, CompositeDisposable, TimerDisposable};
use crate::global::GlobalStateStore;
use crate::logging::BehaviorLog;

use super::audio_player_link_scene_behavior::{update_can_link, update_ticks};
use super::audio_player_view_model::AudioPlayerViewModel;

#[injectable]
#[inject(|global_state: Rc<GlobalStateStore>| Self::new(global_state))]
pub struct AudioPlayerTicksBehavior {
    global_state: Rc<GlobalStateStore>,
}

impl AudioPlayerTicksBehavior {
    pub fn new(global_state: Rc<GlobalStateStore>) -> Self
    {
        Self { global_state }
    }
}

impl Behavior<AudioPlayerViewModel> for AudioPlayerTicksBehavior {
    fn activate(&self, view_model: &mut AudioPlayerViewModel, disposables: &mut CompositeDisposable)
    {
        BehaviorLog::behavior_activated("AudioPlayerTicksBehavior", "AudioPlayerViewModel");
        let Some(view_model_handle) = view_model.self_handle().and_then(|handle| handle.upgrade())
        else {
            return;
        };
        let global_state = Rc::clone(&self.global_state);
        let timer = slint::Timer::default();
        timer.start(TimerMode::Repeated, Duration::from_millis(100), move || {
            let Some(snapshot) = global_state.try_with_state(|global_state| {
                (global_state.scenes.clone(), global_state.selected_scene.clone())
            }) else {
                return;
            };
            let mut view_model = view_model_handle.borrow_mut();
            update_ticks(&mut view_model, &snapshot.0);
            update_can_link(&mut view_model, snapshot.1.as_ref(), &snapshot.0);
        });
        disposables.add(Box::new(TimerDisposable::new(timer)));
    }
}
