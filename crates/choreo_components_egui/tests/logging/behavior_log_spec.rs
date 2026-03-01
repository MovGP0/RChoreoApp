use crate::logging;
use logging::Report;

#[test]
fn behavior_log_spec() {
    let suite = rspec::describe("logging behavior activation", (), |spec| {
        spec.it("records the expected activation message", |_| {
            let mut state = logging::state::LoggingState::default();

            logging::reducer::reduce(
                &mut state,
                logging::actions::LoggingAction::BehaviorActivated {
                    name: "LoadDancerSettingsBehavior".to_string(),
                    view_model: "DancerSettingsViewModel".to_string(),
                },
            );

            assert_eq!(state.entries.len(), 1);
            assert_eq!(state.entries[0].level, logging::state::LogLevel::Debug);
            assert_eq!(
                state.entries[0].message,
                "behavior activated: LoadDancerSettingsBehavior -> DancerSettingsViewModel"
            );
        });
    });
    let report = logging::run_suite(&suite);
    assert!(report.is_success());
}
