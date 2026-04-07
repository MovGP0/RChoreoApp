use egui::Rect;
use egui::pos2;

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

fn assert_no_errors(errors: Vec<String>) {
    assert!(
        errors.is_empty(),
        "Assertion failures:\n{}",
        errors.join("\n")
    );
}

#[test]
fn dialog_host_panel_respects_24px_margin() {
    let bounds = Rect::from_min_max(pos2(0.0, 0.0), pos2(480.0, 320.0));

    let panel = crate::dialog_host::ui::dialog_panel_rect(bounds, 24.0);

    let mut errors = Vec::new();

    check_eq!(errors, panel.left(), 24.0);
    check_eq!(errors, panel.top(), 24.0);
    check_eq!(errors, panel.right(), 456.0);
    check_eq!(errors, panel.bottom(), 296.0);

    assert_no_errors(errors);
}
