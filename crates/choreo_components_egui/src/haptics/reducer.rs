use super::actions::HapticsAction;
use super::state::{HapticBackend, HapticEffect, HapticsState};

pub fn reduce(state: &mut HapticsState, action: HapticsAction) {
    match action {
        HapticsAction::Initialize => {
            state.pending_effect = None;
        }
        HapticsAction::SetBackend { backend } => {
            state.backend = backend;
            if backend == HapticBackend::Noop {
                state.supported = false;
            }
        }
        HapticsAction::SetSupported { supported } => {
            state.supported = supported;
        }
        HapticsAction::TriggerClick => {
            state.trigger_count += 1;
            if state.supported {
                state.delivered_count += 1;
                state.pending_effect = Some(HapticEffect::Click);
            }
        }
        HapticsAction::ConsumePendingEffect => {
            state.pending_effect = None;
        }
    }
}
