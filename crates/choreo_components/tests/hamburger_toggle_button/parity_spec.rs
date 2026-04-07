use material3::components::hamburger_toggle_button::HamburgerToggleButton;
use material3::components::hamburger_toggle_button::minimum_button_size_token;
use material3::components::hamburger_toggle_button::next_checked_state;

macro_rules! check_eq {
    ($errors:expr, $left:expr, $right:expr) => {
        if $left != $right {
            $errors.push(format!(
                "{} != {} (left = {:?}, right = {:?})",
                stringify!($left),
                stringify!($right),
                $left,
                $right
            ));
        }
    };
}

#[test]
fn next_checked_state_matches_enabled_and_toggle_on_click_semantics() {
    let mut errors = Vec::new();

    check_eq!(errors, next_checked_state(false, true, true, true), true);
    check_eq!(errors, next_checked_state(false, false, true, true), false);
    check_eq!(errors, next_checked_state(true, true, false, true), true);
    check_eq!(errors, next_checked_state(false, true, true, false), false);

    assert!(errors.is_empty(), "Assertion failures:\n{}", errors.join("\n"));
}

#[test]
fn widget_show_clamps_size_to_slint_minimum_without_click_side_effects() {
    let context = egui::Context::default();
    let mut next_checked = None;
    let mut allocated_size = None;

    let _ = context.run(egui::RawInput::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let result = HamburgerToggleButton::new(true)
                .enabled(true)
                .toggle_on_click(false)
                .tooltip("Menu")
                .size(egui::vec2(24.0, 30.0))
                .show(ui);
            next_checked = Some(result.checked);
            allocated_size = Some(result.response.rect.size());
        });
    });

    let mut errors = Vec::new();

    check_eq!(errors, next_checked, Some(true));

    let size = allocated_size.expect("widget should allocate a button rect");
    check_eq!(errors, size.x, minimum_button_size_token());
    check_eq!(errors, size.y, minimum_button_size_token());

    assert!(errors.is_empty(), "Assertion failures:\n{}", errors.join("\n"));
}
