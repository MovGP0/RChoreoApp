use crate::logging;
use logging::Report;

#[test]
fn bounded_log_spec() {
    let suite = rspec::describe("logging bounded buffer", (), |spec| {
        spec.it("drops oldest entries when max is exceeded", |_| {
            let mut state = logging::state::LoggingState::default();
            logging::reducer::reduce(&mut state, logging::actions::LoggingAction::Initialize);

            logging::reducer::reduce(
                &mut state,
                logging::actions::LoggingAction::SetMaxEntries { max_entries: 2 },
            );
            logging::reducer::reduce(
                &mut state,
                logging::actions::LoggingAction::RecordDebug {
                    message: "debug".to_string(),
                },
            );
            logging::reducer::reduce(
                &mut state,
                logging::actions::LoggingAction::RecordWarn {
                    message: "warn".to_string(),
                },
            );
            logging::reducer::reduce(
                &mut state,
                logging::actions::LoggingAction::RecordError {
                    message: "error".to_string(),
                },
            );
            logging::reducer::reduce(
                &mut state,
                logging::actions::LoggingAction::RecordInfo {
                    message: "first".to_string(),
                },
            );
            logging::reducer::reduce(
                &mut state,
                logging::actions::LoggingAction::RecordInfo {
                    message: "second".to_string(),
                },
            );
            logging::reducer::reduce(
                &mut state,
                logging::actions::LoggingAction::RecordInfo {
                    message: "third".to_string(),
                },
            );

            assert_eq!(state.entries.len(), 2);
            assert_eq!(state.entries[0].message, "second");
            assert_eq!(state.entries[1].message, "third");
            assert_eq!(state.dropped_entries, 4);

            logging::reducer::reduce(&mut state, logging::actions::LoggingAction::ClearEntries);
            assert!(state.entries.is_empty());
        });
    });
    let report = logging::run_suite(&suite);
    assert!(report.is_success());
}
