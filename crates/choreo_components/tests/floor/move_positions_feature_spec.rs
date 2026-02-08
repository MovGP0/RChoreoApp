use crate::floor;

use floor::Report;
use choreo_components::floor::{Matrix, Point};
use choreo_components::global::InteractionMode;
use choreo_state_machine::{MovePositionsStartedTrigger, StateKind};

fn setup_context() -> floor::FloorTestContext {
    let context = floor::FloorTestContext::new();
    context.configure_canvas();

    let (choreography, scene) = floor::build_three_position_choreography();
    let scene_view_model = floor::map_scene_view_model(&scene);

    context.update_global_state(|state| {
        state.choreography = choreography;
        state.selected_scene = Some(scene_view_model.clone());
        state.scenes = vec![scene_view_model];
        state.interaction_mode = InteractionMode::Move;
    });

    context.update_state_machine(|state_machine| {
        let _ = state_machine.try_apply(&MovePositionsStartedTrigger);
    });

    context
}

fn select_rectangle(context: &floor::FloorTestContext, start: Point, end: Point) {
    let view_start = floor::floor_to_view_point(context, start);
    let view_end = floor::floor_to_view_point(context, end);
    context.send_pointer_pressed(view_start);
    context.send_pointer_moved(view_end);
    context.send_pointer_released(view_end);
}

fn drag_from_to(context: &floor::FloorTestContext, start: Point, end: Point) {
    let view_start = floor::floor_to_view_point(context, start);
    let view_end = floor::floor_to_view_point(context, end);
    context.send_pointer_pressed(view_start);
    context.send_pointer_moved(view_end);
    context.send_pointer_released(view_end);
}

#[test]
fn move_positions_feature_spec() {
    let suite = rspec::describe("move positions feature", (), |spec| {
        spec.it("moves all selected positions by drag delta", |_| {
            let context = setup_context();
            let start_first = Point::new(-1.0, 1.0);
            let start_second = Point::new(1.0, 1.0);
            let start_third = Point::new(3.0, -2.0);

            select_rectangle(&context, Point::new(-2.0, 2.0), Point::new(2.0, 0.0));
            drag_from_to(
                &context,
                start_first,
                Point::new(start_first.x + 1.5, start_first.y - 1.0),
            );

            let scene = context.read_global_state(|state| state.selected_scene.clone().expect("scene"));
            let first = &scene.positions[0];
            let second = &scene.positions[1];
            let third = &scene.positions[2];

            floor::assert_close(first.x, start_first.x + 1.5, 0.0001);
            floor::assert_close(first.y, start_first.y - 1.0, 0.0001);
            floor::assert_close(second.x, start_second.x + 1.5, 0.0001);
            floor::assert_close(second.y, start_second.y - 1.0, 0.0001);
            floor::assert_close(third.x, start_third.x, 0.0001);
            floor::assert_close(third.y, start_third.y, 0.0001);
        });

        spec.it("clears selection when clicking outside", |_| {
            let context = setup_context();
            select_rectangle(&context, Point::new(-2.0, 2.0), Point::new(2.0, 0.0));

            drag_from_to(&context, Point::new(4.0, 4.0), Point::new(4.0, 4.0));

            let selected_count = context.read_global_state(|state| state.selected_positions.len());
            let has_rectangle = context.read_global_state(|state| state.selection_rectangle.is_some());

            assert_eq!(selected_count, 0);
            assert!(!has_rectangle);
        });

        spec.it("moves a single position when dragging", |_| {
            let context = setup_context();
            let start_first = Point::new(-1.0, 1.0);
            let start_second = Point::new(1.0, 1.0);
            let start_third = Point::new(3.0, -2.0);

            drag_from_to(
                &context,
                start_first,
                Point::new(start_first.x - 1.0, start_first.y + 2.0),
            );

            let scene = context.read_global_state(|state| state.selected_scene.clone().expect("scene"));
            let selected_count = context.read_global_state(|state| state.selected_positions.len());
            let first = &scene.positions[0];
            let second = &scene.positions[1];
            let third = &scene.positions[2];

            floor::assert_close(first.x, start_first.x - 1.0, 0.0001);
            floor::assert_close(first.y, start_first.y + 2.0, 0.0001);
            floor::assert_close(second.x, start_second.x, 0.0001);
            floor::assert_close(second.y, start_second.y, 0.0001);
            floor::assert_close(third.x, start_third.x, 0.0001);
            floor::assert_close(third.y, start_third.y, 0.0001);
            assert_eq!(selected_count, 1);
        });

        spec.it("selects positions with mouse drag rectangle", |_| {
            let context = setup_context();
            assert_eq!(context.state_kind(), StateKind::MovePositionsState);
            select_rectangle(&context, Point::new(-2.0, 2.0), Point::new(2.0, 0.0));

            let selected = context.read_global_state(|state| state.selected_positions.clone());
            assert_eq!(selected.len(), 2);
            assert!(selected.iter().any(|position| (position.x + 1.0).abs() < 0.0001 && (position.y - 1.0).abs() < 0.0001));
            assert!(selected.iter().any(|position| (position.x - 1.0).abs() < 0.0001 && (position.y - 1.0).abs() < 0.0001));
            assert!(!selected.iter().any(|position| (position.x - 3.0).abs() < 0.0001 && (position.y + 2.0).abs() < 0.0001));
        });

        spec.it("selects positions with mouse drag rectangle after translation", |_| {
            let context = setup_context();
            assert_eq!(context.state_kind(), StateKind::MovePositionsState);
            context.set_transformation_matrix(Matrix::translation(10.0, -12.0));

            select_rectangle(&context, Point::new(-2.0, 2.0), Point::new(2.0, 0.0));

            let selected = context.read_global_state(|state| state.selected_positions.clone());
            assert_eq!(selected.len(), 2);
            assert!(selected.iter().any(|position| (position.x + 1.0).abs() < 0.0001 && (position.y - 1.0).abs() < 0.0001));
            assert!(selected.iter().any(|position| (position.x - 1.0).abs() < 0.0001 && (position.y - 1.0).abs() < 0.0001));
            assert!(!selected.iter().any(|position| (position.x - 3.0).abs() < 0.0001 && (position.y + 2.0).abs() < 0.0001));
        });
    });

    let report = floor::run_suite(&suite);
    assert!(report.is_success());
}
