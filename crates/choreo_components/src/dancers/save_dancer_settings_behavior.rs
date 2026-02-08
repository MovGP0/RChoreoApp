use std::collections::HashMap;
use std::rc::Rc;
use std::time::Duration;

use choreo_master_mobile_json::DancerId;
use choreo_models::DancerModel;
use crossbeam_channel::Receiver;
use nject::injectable;
use slint::TimerMode;

use crate::behavior::{Behavior, CompositeDisposable, TimerDisposable};
use crate::global::GlobalStateActor;
use crate::logging::BehaviorLog;

use super::dancer_settings_view_model::DancerSettingsViewModel;
use super::mapper::update_scene_dancers;
use super::messages::SaveDancerSettingsCommand;

#[injectable]
#[inject(
    |global_state: Rc<GlobalStateActor>,
     receiver: Receiver<SaveDancerSettingsCommand>| {
        Self::new(global_state, receiver)
    }
)]
pub struct SaveDancerSettingsBehavior {
    global_state: Rc<GlobalStateActor>,
    receiver: Receiver<SaveDancerSettingsCommand>,
}

impl SaveDancerSettingsBehavior {
    pub(super) fn new(
        global_state: Rc<GlobalStateActor>,
        receiver: Receiver<SaveDancerSettingsCommand>,
    ) -> Self {
        Self {
            global_state,
            receiver,
        }
    }

    fn apply_changes(
        global_state: &GlobalStateActor,
        view_model: &DancerSettingsViewModel,
    ) -> bool {
        global_state.try_update(|global_state| {
            let choreography = &mut global_state.choreography;
            choreography.roles.clear();
            choreography.roles.extend(view_model.roles.iter().cloned());

            choreography.dancers.clear();
            choreography
                .dancers
                .extend(view_model.dancers.iter().cloned());

            let dancer_map: HashMap<DancerId, Rc<DancerModel>> = view_model
                .dancers
                .iter()
                .map(|dancer| (dancer.dancer_id, dancer.clone()))
                .collect();

            for scene in &mut choreography.scenes {
                update_scene_dancers(scene, &dancer_map);
            }
        })
    }
}

impl Behavior<DancerSettingsViewModel> for SaveDancerSettingsBehavior {
    fn activate(
        &self,
        view_model: &mut DancerSettingsViewModel,
        disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated("SaveDancerSettingsBehavior", "DancerSettingsViewModel");
        let Some(view_model_handle) = view_model.self_handle().and_then(|handle| handle.upgrade())
        else {
            return;
        };
        let receiver = self.receiver.clone();
        let global_state = Rc::clone(&self.global_state);
        let timer = slint::Timer::default();
        timer.start(TimerMode::Repeated, Duration::from_millis(16), move || {
            while receiver.try_recv().is_ok() {
                let view_model = view_model_handle.borrow();
                if !Self::apply_changes(&global_state, &view_model) {
                    return;
                }
            }
        });
        disposables.add(Box::new(TimerDisposable::new(timer)));
    }
}
