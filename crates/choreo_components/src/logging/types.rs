pub struct BehaviorLog;

impl BehaviorLog {
    pub fn behavior_activated(name: &str, view_model: &str) {
        log::debug!("behavior activated: {name} -> {view_model}");
    }
}
