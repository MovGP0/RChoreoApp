#[test]
fn click_feedback_spec() {
    use choreo_components::haptics::HapticFeedback;
    use choreo_components::haptics::NoopHapticFeedback;

    let haptic = NoopHapticFeedback::new();
    assert!(!haptic.is_supported());
    haptic.perform_click();
}
