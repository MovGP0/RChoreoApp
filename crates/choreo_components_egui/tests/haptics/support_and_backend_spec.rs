use crate::haptics;
use haptics::Report;

#[test]
fn support_and_backend_spec() {
    let suite = rspec::describe("haptics support and backend", (), |spec| {
        spec.it("resets support when backend becomes noop", |_| {
            let mut state = haptics::state::HapticsState::default();

            haptics::reducer::reduce(
                &mut state,
                haptics::actions::HapticsAction::SetBackend {
                    backend: haptics::state::HapticBackend::Platform,
                },
            );
            haptics::reducer::reduce(
                &mut state,
                haptics::actions::HapticsAction::SetSupported { supported: true },
            );
            assert!(state.supported);
            assert_eq!(state.backend, haptics::state::HapticBackend::Platform);

            haptics::reducer::reduce(
                &mut state,
                haptics::actions::HapticsAction::SetBackend {
                    backend: haptics::state::HapticBackend::Noop,
                },
            );

            assert!(!state.supported);
            assert_eq!(state.backend, haptics::state::HapticBackend::Noop);
        });
    });
    let report = haptics::run_suite(&suite);
    assert!(report.is_success());
}
