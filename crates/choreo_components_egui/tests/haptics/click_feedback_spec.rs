#[test]
fn click_feedback_spec() {
    use choreo_components_egui::haptics::HapticFeedbackHandle;
    use choreo_components_egui::haptics::NoopHapticFeedback;
    use choreo_components_egui::haptics::perform_click_if_supported;

    let haptic: HapticFeedbackHandle = Some(Box::new(NoopHapticFeedback::new()));
    perform_click_if_supported(haptic.as_deref());
    perform_click_if_supported(None);
}
