#[test]
fn click_feedback_spec() {
    use choreo_components_egui::haptics::HapticFeedback;
    use choreo_components_egui::haptics::NoopHapticFeedback;

    let haptic = NoopHapticFeedback::new();
    assert!(!haptic.is_supported());
    haptic.perform_click();
}
