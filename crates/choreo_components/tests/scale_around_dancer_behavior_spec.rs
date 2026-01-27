mod common;

use std::sync::{Arc, Mutex};
use std::time::Duration;

use common::Report;
use choreo_components::floor::{Point, ScaleAroundDancerBehavior, TimeProvider};
use choreo_components::global::InteractionMode;
use choreo_state_machine::ScaleAroundDancerStartedTrigger;

#[derive(Debug)]
struct TestTimeProvider {
    now: Mutex<Duration>,
}

impl TestTimeProvider {
    fn new(initial: Duration) -> Self {
        Self {
            now: Mutex::new(initial),
        }
    }

    fn advance(&self, delta: Duration) {
        let mut guard = self.now.lock().expect("time lock");
        *guard += delta;
    }
}

impl TimeProvider for TestTimeProvider {
    fn now(&self) -> Duration {
        *self.now.lock().expect("time lock")
    }
}

struct SharedTimeProvider {
    inner: Arc<TestTimeProvider>,
}

impl TimeProvider for SharedTimeProvider {
    fn now(&self) -> Duration {
        self.inner.now()
    }
}

fn setup_context() -> (common::FloorTestContext, ScaleAroundDancerBehavior, Arc<TestTimeProvider>) {
    let mut context = common::FloorTestContext::new();
    context.configure_canvas();

    let (choreography, scene) = common::build_three_position_choreography();
    let scene_view_model = common::map_scene_view_model(&scene);
    context.global_state.choreography = choreography;
    context.global_state.selected_scene = Some(scene_view_model.clone());
    context.global_state.scenes = vec![scene_view_model];
    context.global_state.interaction_mode = InteractionMode::RotateAroundDancer;
    let _ = context.state_machine.try_apply(&ScaleAroundDancerStartedTrigger);

    let time_provider = Arc::new(TestTimeProvider::new(Duration::from_secs(0)));
    let behavior = ScaleAroundDancerBehavior::with_time_provider(Box::new(SharedTimeProvider {
        inner: time_provider.clone(),
    }));

    (context, behavior, time_provider)
}

fn select_rectangle(
    behavior: &mut ScaleAroundDancerBehavior,
    context: &mut common::FloorTestContext,
    start: Point,
    end: Point,
) {
    let view_start = common::floor_to_view_point(&context.view_model, &context.global_state.choreography, start);
    let view_end = common::floor_to_view_point(&context.view_model, &context.global_state.choreography, end);
    behavior.handle_pointer_pressed(&context.view_model, &mut context.global_state, &mut context.state_machine, common::pointer_pressed(view_start));
    behavior.handle_pointer_moved(&context.view_model, &mut context.global_state, &mut context.state_machine, common::pointer_moved(view_end));
    behavior.handle_pointer_released(&context.view_model, &mut context.global_state, &mut context.state_machine, common::pointer_released(view_end));
}

fn double_tap(
    behavior: &mut ScaleAroundDancerBehavior,
    context: &mut common::FloorTestContext,
    time_provider: &TestTimeProvider,
    point: Point,
) {
    let view_point = common::floor_to_view_point(&context.view_model, &context.global_state.choreography, point);
    behavior.handle_pointer_pressed(&context.view_model, &mut context.global_state, &mut context.state_machine, common::pointer_pressed(view_point));
    behavior.handle_pointer_released(&context.view_model, &mut context.global_state, &mut context.state_machine, common::pointer_released(view_point));
    time_provider.advance(Duration::from_millis(100));
    behavior.handle_pointer_pressed(&context.view_model, &mut context.global_state, &mut context.state_machine, common::pointer_pressed(view_point));
    behavior.handle_pointer_released(&context.view_model, &mut context.global_state, &mut context.state_machine, common::pointer_released(view_point));
}

fn drag_from_to(
    behavior: &mut ScaleAroundDancerBehavior,
    context: &mut common::FloorTestContext,
    start: Point,
    end: Point,
) {
    let view_start = common::floor_to_view_point(&context.view_model, &context.global_state.choreography, start);
    let view_end = common::floor_to_view_point(&context.view_model, &context.global_state.choreography, end);
    behavior.handle_pointer_pressed(&context.view_model, &mut context.global_state, &mut context.state_machine, common::pointer_pressed(view_start));
    behavior.handle_pointer_moved(&context.view_model, &mut context.global_state, &mut context.state_machine, common::pointer_moved(view_end));
    behavior.handle_pointer_released(&context.view_model, &mut context.global_state, &mut context.state_machine, common::pointer_released(view_end));
}

#[test]
fn scale_around_dancer_behavior_spec() {
    let suite = rspec::describe("scale around dancer behavior", (), |spec| {
        spec.it("rotates around tapped dancer", |_| {
            let (mut context, mut behavior, time_provider) = setup_context();
            select_rectangle(&mut behavior, &mut context, Point::new(-2.0, 2.0), Point::new(2.0, 0.0));
            double_tap(&mut behavior, &mut context, &time_provider, Point::new(-1.0, 1.0));
            drag_from_to(&mut behavior, &mut context, Point::new(-1.0, 2.0), Point::new(0.0, 1.0));

            let scene = context.global_state.selected_scene.as_ref().expect("scene");
            let first = &scene.positions[0];
            let second = &scene.positions[1];
            let third = &scene.positions[2];

            common::assert_close(first.x, -1.0, 0.0001);
            common::assert_close(first.y, 1.0, 0.0001);
            common::assert_close(second.x, -1.0, 0.0001);
            common::assert_close(second.y, -1.0, 0.0001);
            common::assert_close(third.x, 3.0, 0.0001);
            common::assert_close(third.y, -2.0, 0.0001);
        });

        spec.it("rotates around tapped dancer with mouse", |_| {
            let (mut context, mut behavior, time_provider) = setup_context();
            select_rectangle(&mut behavior, &mut context, Point::new(-2.0, 2.0), Point::new(2.0, 0.0));
            double_tap(&mut behavior, &mut context, &time_provider, Point::new(-1.0, 1.0));
            drag_from_to(&mut behavior, &mut context, Point::new(-1.0, 2.0), Point::new(0.0, 1.0));

            let scene = context.global_state.selected_scene.as_ref().expect("scene");
            let second = &scene.positions[1];
            common::assert_close(second.x, -1.0, 0.0001);
            common::assert_close(second.y, -1.0, 0.0001);
        });
    });

    let report = common::run_suite(&suite);
    assert!(report.is_success());
}
