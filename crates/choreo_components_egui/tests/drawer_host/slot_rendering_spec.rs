use std::cell::RefCell;
use std::rc::Rc;

use crate::drawer_host::state::DrawerHostState;
use crate::drawer_host::ui::draw_with_slots;

#[test]
fn drawer_host_slot_rendering_matches_open_panels() {
    let state = DrawerHostState {
        inline_left: true,
        is_left_open: true,
        is_right_open: false,
        is_top_open: true,
        is_bottom_open: false,
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
                "drawer_host_slot_spec",
                &state,
                move |_| content_calls.borrow_mut().push("content"),
                move |_| left_calls.borrow_mut().push("left"),
                move |_| right_calls.borrow_mut().push("right"),
                move |_| top_calls.borrow_mut().push("top"),
                move |_| bottom_calls.borrow_mut().push("bottom"),
            );
        });
    });

    assert_eq!(calls.borrow().as_slice(), &["content", "left", "top"]);
}
