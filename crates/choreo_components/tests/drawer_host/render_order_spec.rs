use std::cell::RefCell;
use std::rc::Rc;

use crate::drawer_host::state::DrawerHostState;
use crate::drawer_host::ui::draw_with_slots;
use crate::drawer_host::ui::overlay_visible;

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

#[test]
fn open_right_panel_still_renders_while_overlay_is_visible() {
    let state = DrawerHostState {
        is_right_open: true,
        right_close_on_click_away: true,
        ..DrawerHostState::default()
    };
    let calls: Rc<RefCell<Vec<&'static str>>> = Rc::new(RefCell::new(Vec::new()));
    let context = egui::Context::default();
    let _ = context.run(egui::RawInput::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let content_calls = Rc::clone(&calls);
            let left_calls = Rc::clone(&calls);
            let right_calls = Rc::clone(&calls);
            let top_calls = Rc::clone(&calls);
            let bottom_calls = Rc::clone(&calls);

            let _ = draw_with_slots(
                ui,
                "drawer_host_render_order",
                &state,
                move |_| content_calls.borrow_mut().push("content"),
                move |_| left_calls.borrow_mut().push("left"),
                move |_| right_calls.borrow_mut().push("right"),
                move |_| top_calls.borrow_mut().push("top"),
                move |_| bottom_calls.borrow_mut().push("bottom"),
            );
        });
    });

    let mut errors = Vec::new();

    check!(errors, overlay_visible(&state, 1280.0));
    check_eq!(errors, calls.borrow().as_slice(), &["content", "right"]);

    assert_no_errors(errors);
}
