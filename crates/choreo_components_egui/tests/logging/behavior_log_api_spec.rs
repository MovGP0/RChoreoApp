use crate::logging;
use logging::Report;

#[test]
fn behavior_log_api_spec() {
    let suite = rspec::describe("logging behavior log api", (), |spec| {
        spec.it("exposes BehaviorLog::behavior_activated parity api", |_| {
            logging::types::BehaviorLog::behavior_activated(
                "LoadDancerSettingsBehavior",
                "DancerSettingsViewModel",
            );
        });

        spec.it("keeps activation message format parity", |_| {
            let message = logging::types::activation_message(
                "LoadDancerSettingsBehavior",
                "DancerSettingsViewModel",
            );
            assert_eq!(
                message,
                "behavior activated: LoadDancerSettingsBehavior -> DancerSettingsViewModel"
            );
        });
    });
    let report = logging::run_suite(&suite);
    assert!(report.is_success());
}
