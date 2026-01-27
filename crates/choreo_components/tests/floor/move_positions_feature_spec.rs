mod floor;

use choreo_components::floor::Report;
use choreo_components::floor::{Matrix, MovePositionsBehavior, Point};
use choreo_components::global::InteractionMode;
use choreo_state_machine::{MovePositionsStartedTrigger, StateKind};

fn setup_context() -> (floor::FloorTestContext, MovePositionsBehavior, choreo_models::SceneModel) {
    let mut context = floor::FloorTestContext::new();
    context.configure_canvas();

    let (choreography, scene) = floor::build_three_position_choreography();
    let scene_view_model = floor::map_scene_view_model(&scene);
    context.global_state.choreography = choreography;
    context.global_state.selected_scene = Some(scene_view_model.clone());
    context.global_state.scenes = vec![scene_view_model];
    context.global_state.interaction_mode = InteractionMode::Move;
    let _ = context.state_machine.try_apply(&MovePositionsStartedTrigger);

    (context, MovePositionsBehavior::default(), scene)
}

fn select_rectangle(
    behavior: &mut MovePositionsBehavior,
    context: &mut floor::FloorTestContext,
    start: Point,
    end: Point,
) {
    let view_start = floor::floor_to_view_point(&context.view_model, &context.global_state.choreography, start);
    let view_end = floor::floor_to_view_point(&context.view_model, &context.global_state.choreography, end);
    behavior.handle_pointer_pressed(&context.view_model, &mut context.global_state, &mut context.state_machine, floor::pointer_pressed(view_start));
    behavior.handle_pointer_moved(&context.view_model, &mut context.global_state, &mut context.state_machine, floor::pointer_moved(view_end));
    behavior.handle_pointer_released(&context.view_model, &mut context.global_state, &mut context.state_machine, floor::pointer_released(view_end));
}

fn drag_from_to(
    behavior: &mut MovePositionsBehavior,
    context: &mut floor::FloorTestContext,
    start: Point,
    end: Point,
) {
    let view_start = floor::floor_to_view_point(&context.view_model, &context.global_state.choreography, start);
    let view_end = floor::floor_to_view_point(&context.view_model, &context.global_state.choreography, end);
    behavior.handle_pointer_pressed(&context.view_model, &mut context.global_state, &mut context.state_machine, floor::pointer_pressed(view_start));
    behavior.handle_pointer_moved(&context.view_model, &mut context.global_state, &mut context.state_machine, floor::pointer_moved(view_end));
    behavior.handle_pointer_released(&context.view_model, &mut context.global_state, &mut context.state_machine, floor::pointer_released(view_end));
}

#[test]
fn move_positions_feature_spec() {
    let suite = rspec::describe("move positions feature", (), |spec| {
        spec.it("moves all selected positions by drag delta", |_| {
            let (mut context, mut behavior, _) = setup_context();
            let start_first = Point::new(-1.0, 1.0);
            let start_second = Point::new(1.0, 1.0);
            let start_third = Point::new(3.0, -2.0);

            select_rectangle(&mut behavior, &mut context, Point::new(-2.0, 2.0), Point::new(2.0, 0.0));
            drag_from_to(
                &mut behavior,
                &mut context,
                start_first,
                Point::new(start_first.x + 1.5, start_first.y - 1.0),
            );

            let scene = context.global_state.selected_scene.as_ref().expect("scene");
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
            let (mut context, mut behavior, _) = setup_context();
            select_rectangle(&mut behavior, &mut context, Point::new(-2.0, 2.0), Point::new(2.0, 0.0));

            drag_from_to(&mut behavior, &mut context, Point::new(4.0, 4.0), Point::new(4.0, 4.0));

            assert_eq!(context.global_state.selected_positions.len(), 0);
            assert!(context.global_state.selection_rectangle.is_none());
        });

        spec.it("moves a single position when dragging", |_| {
            let (mut context, mut behavior, _) = setup_context();
            let start_first = Point::new(-1.0, 1.0);
            let start_second = Point::new(1.0, 1.0);
            let start_third = Point::new(3.0, -2.0);

            drag_from_to(
                &mut behavior,
                &mut context,
                start_first,
                Point::new(start_first.x - 1.0, start_first.y + 2.0),
            );

            let scene = context.global_state.selected_scene.as_ref().expect("scene");
            let first = &scene.positions[0];
            let second = &scene.positions[1];
            let third = &scene.positions[2];

            floor::assert_close(first.x, start_first.x - 1.0, 0.0001);
            floor::assert_close(first.y, start_first.y + 2.0, 0.0001);
            floor::assert_close(second.x, start_second.x, 0.0001);
            floor::assert_close(second.y, start_second.y, 0.0001);
            floor::assert_close(third.x, start_third.x, 0.0001);
            floor::assert_close(third.y, start_third.y, 0.0001);
            assert_eq!(context.global_state.selected_positions.len(), 1);
        });

        spec.it("selects positions with mouse drag rectangle", |_| {
            let (mut context, mut behavior, _) = setup_context();
            assert_eq!(context.state_machine.state().kind(), StateKind::MovePositionsState);
            select_rectangle(&mut behavior, &mut context, Point::new(-2.0, 2.0), Point::new(2.0, 0.0));

            let selected = &context.global_state.selected_positions;
            assert_eq!(selected.len(), 2);
            assert!(selected.iter().any(|position| (position.x + 1.0).abs() < 0.0001 && (position.y - 1.0).abs() < 0.0001));
            assert!(selected.iter().any(|position| (position.x - 1.0).abs() < 0.0001 && (position.y - 1.0).abs() < 0.0001));
            assert!(!selected.iter().any(|position| (position.x - 3.0).abs() < 0.0001 && (position.y + 2.0).abs() < 0.0001));
        });

        spec.it("selects positions with mouse drag rectangle after translation", |_| {
            let (mut context, mut behavior, _) = setup_context();
            assert_eq!(context.state_machine.state().kind(), StateKind::MovePositionsState);
            context.view_model.set_transformation_matrix(Matrix::translation(10.0, -12.0));

            select_rectangle(&mut behavior, &mut context, Point::new(-2.0, 2.0), Point::new(2.0, 0.0));

            let selected = &context.global_state.selected_positions;
            assert_eq!(selected.len(), 2);
            assert!(selected.iter().any(|position| (position.x + 1.0).abs() < 0.0001 && (position.y - 1.0).abs() < 0.0001));
            assert!(selected.iter().any(|position| (position.x - 1.0).abs() < 0.0001 && (position.y - 1.0).abs() < 0.0001));
            assert!(!selected.iter().any(|position| (position.x - 3.0).abs() < 0.0001 && (position.y + 2.0).abs() < 0.0001));
        });
    });

    let report = floor::run_suite(&suite);
    assert!(report.is_success());
}
