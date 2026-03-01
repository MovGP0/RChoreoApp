use super::state::HapticBackend;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HapticsAction {
    Initialize,
    SetBackend {
        backend: HapticBackend,
    },
    SetSupported {
        supported: bool,
    },
    TriggerClick,
    ConsumePendingEffect,
}
