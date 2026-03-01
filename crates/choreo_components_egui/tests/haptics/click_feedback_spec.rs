use crate::haptics;
use haptics::Report;

#[test]
fn click_feedback_spec() {
    let suite = rspec::describe("haptics click feedback", (), |spec| {
        spec.it(
            "queues click effect only when supported and clears it after consume",
            |_| {
                let mut state = haptics::state::HapticsState::default();
                haptics::reducer::reduce(
                    &mut state,
                    haptics::actions::HapticsAction::Initialize,
                );

                haptics::reducer::reduce(
                    &mut state,
                    haptics::actions::HapticsAction::TriggerClick,
                );
                assert_eq!(state.trigger_count, 1);
                assert_eq!(state.delivered_count, 0);
                assert!(state.pending_effect.is_none());

                haptics::reducer::reduce(
                    &mut state,
                    haptics::actions::HapticsAction::SetSupported { supported: true },
                );
                haptics::reducer::reduce(
                    &mut state,
                    haptics::actions::HapticsAction::TriggerClick,
                );

                assert_eq!(state.trigger_count, 2);
                assert_eq!(state.delivered_count, 1);
                assert_eq!(
                    state.pending_effect,
                    Some(haptics::state::HapticEffect::Click)
                );

                haptics::reducer::reduce(
                    &mut state,
                    haptics::actions::HapticsAction::ConsumePendingEffect,
                );
                assert!(state.pending_effect.is_none());
            },
        );
    });
    let report = haptics::run_suite(&suite);
    assert!(report.is_success());
}
