use std::sync::mpsc::sync_channel;

use crate::floor::floor_component::CanvasViewHandle;
use crate::floor::floor_component::DrawFloorCommand;
use crate::floor::floor_component::FloorCanvasViewModel;
use crate::floor::floor_component::FloorPointerEventSenders;
use crate::floor::floor_component::Matrix;
use crate::floor::floor_component::Point;
use crate::floor::floor_component::PointerButton;
use crate::floor::floor_component::PointerEventArgs;
use crate::floor::floor_component::PointerPressedCommand;

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
fn floor_view_model_public_api_matches_non_ui_pipeline_contract() {
    let (draw_sender, _draw_receiver) = sync_channel::<DrawFloorCommand>(8);
    let (pressed_sender, pressed_receiver) = sync_channel::<PointerPressedCommand>(8);
    let mut view_model = FloorCanvasViewModel::new(
        draw_sender,
        FloorPointerEventSenders {
            pointer_pressed_senders: vec![pressed_sender],
            pointer_moved_senders: Vec::new(),
            pointer_released_senders: Vec::new(),
            pointer_wheel_changed_senders: Vec::new(),
            touch_senders: Vec::new(),
        },
    );

    view_model.set_transformation_matrix(Matrix::translation(24.0, 36.0));
    let _ = view_model.pointer_pressed(PointerPressedCommand {
        canvas_view: CanvasViewHandle { id: 7 },
        event_args: PointerEventArgs {
            position: Point::new(120.0, 144.0),
            button: PointerButton::Primary,
            is_in_contact: true,
        },
    });

    let command = pressed_receiver
        .try_recv()
        .expect("pointer pressed command should be routed through view-model senders");
    let mut errors = Vec::new();

    check_eq!(errors, command.canvas_view.id, 7);
    check_eq!(errors, command.event_args.position, Point::new(120.0, 144.0));

    assert_no_errors(errors);
}
