#[test]
fn support_and_backend_spec() {
    use choreo_components::haptics::HapticFeedback;
    use choreo_components::haptics::NoopHapticFeedback;
    use choreo_components::haptics::PlatformHapticFeedback;

    let noop = NoopHapticFeedback::new();
    assert!(!noop.is_supported());
    noop.perform_click();

    let platform = PlatformHapticFeedback::default();
    assert!(!platform.is_supported());
}
