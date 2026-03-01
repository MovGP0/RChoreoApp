pub struct BehaviorLog;

impl BehaviorLog {
    pub fn behavior_activated(name: &str, view_model: &str) {
        log::debug!("{}", activation_message(name, view_model));
    }
}

#[must_use]
pub fn activation_message(name: &str, view_model: &str) -> String {
    format!("behavior activated: {name} -> {view_model}")
}
