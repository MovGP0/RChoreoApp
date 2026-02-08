use std::rc::Rc;
use std::sync::Mutex;

use crate::floor;

use choreo_components::floor::DrawFloorBehavior;
use choreo_components::floor::DrawFloorCommand;
use choreo_components::floor::FloorRenderGate;
use crossbeam_channel::unbounded;
use floor::Report;

#[derive(Default)]
struct TestRenderGate {
    rendered: Mutex<bool>,
    marks: Mutex<usize>,
}

impl FloorRenderGate for TestRenderGate {
    fn is_rendered(&self) -> bool {
        *self.rendered.lock().expect("lock should succeed")
    }

    fn mark_rendered(&self) {
        *self.rendered.lock().expect("lock should succeed") = true;
        *self.marks.lock().expect("lock should succeed") += 1;
    }

    fn wait_for_first_render(&self) {}
}

#[test]
#[serial_test::serial]
fn draw_floor_behavior_spec() {
    let suite = rspec::describe("draw floor behavior", (), |spec| {
        spec.it(
            "marks render gate once when draw command is handled",
            |_| {
                let (sender, receiver) = unbounded::<DrawFloorCommand>();
                let gate = Rc::new(TestRenderGate::default());
                let mut behavior = DrawFloorBehavior::new(receiver, Some(gate.clone()));

                sender
                    .send(DrawFloorCommand)
                    .expect("draw command send should succeed");
                assert!(behavior.try_handle());
                assert!(gate.is_rendered());
                assert_eq!(*gate.marks.lock().expect("lock should succeed"), 1);

                sender
                    .send(DrawFloorCommand)
                    .expect("draw command send should succeed");
                assert!(behavior.try_handle());
                assert_eq!(*gate.marks.lock().expect("lock should succeed"), 1);
            },
        );

        spec.it("returns false when no draw command is pending", |_| {
            let (_sender, receiver) = unbounded::<DrawFloorCommand>();
            let mut behavior = DrawFloorBehavior::new(receiver, None);
            assert!(!behavior.try_handle());
        });
    });

    let report = floor::run_suite(&suite);
    assert!(report.is_success());
}
