#[test]
fn support_and_backend_spec() {
    use choreo_components_egui::haptics::HapticFeedback;
    use choreo_components_egui::haptics::NoopHapticFeedback;
    use choreo_components_egui::haptics::PlatformHapticFeedback;

    let noop = NoopHapticFeedback::new();
    assert!(!noop.is_supported());
    noop.perform_click();

    let platform = PlatformHapticFeedback::default();
    assert!(!platform.is_supported());
}
