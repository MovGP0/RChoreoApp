use choreo_components::logging::BehaviorLog;

#[test]
fn behavior_log_api_spec() {
    BehaviorLog::behavior_activated("LoadDancerSettingsBehavior", "DancerSettingsViewModel");
}
