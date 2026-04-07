#[test]
fn support_and_backend_spec() {
    use choreo_components::haptics::HapticFeedback;
    use choreo_components::haptics::NoopHapticFeedback;
    use choreo_components::haptics::PlatformHapticFeedback;

    macro_rules! check {
        ($errors:expr, $condition:expr) => {
            if !$condition {
                $errors.push(format!("condition failed: {}", stringify!($condition)));
            }
        };
    }

    fn assert_no_errors(errors: Vec<String>) {
        assert!(
            errors.is_empty(),
            "Assertion failures:\n{}",
            errors.join("\n")
        );
    }

    let noop = NoopHapticFeedback::new();
    let mut errors = Vec::new();

    check!(errors, !noop.is_supported());
    noop.perform_click();

    let platform = PlatformHapticFeedback::default();
    check!(errors, !platform.is_supported());

    assert_no_errors(errors);
}
