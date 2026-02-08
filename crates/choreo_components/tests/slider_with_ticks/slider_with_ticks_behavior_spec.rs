use crate::slider_with_ticks;

use choreo_components::behavior::Behavior;
use choreo_components::slider_with_ticks::SliderWithTicksBehavior;
use choreo_components::slider_with_ticks::SliderWithTicksViewModel;
use slider_with_ticks::Report;

#[test]
#[serial_test::serial]
fn slider_with_ticks_behavior_spec() {
    let suite = rspec::describe("slider with ticks behavior", (), |spec| {
        spec.it("activates without mutating default slider state", |_| {
            let behavior = SliderWithTicksBehavior;
            let view_model = SliderWithTicksViewModel::new(vec![Box::new(behavior) as Box<dyn Behavior<_>>]);

            assert_eq!(view_model.minimum, 0.0);
            assert_eq!(view_model.maximum, 1.0);
            assert_eq!(view_model.value, 0.0);
            assert!(view_model.tick_values.is_empty());
            assert!(view_model.tick_color.is_none());
            assert!(view_model.is_enabled);
        });
    });

    let report = slider_with_ticks::run_suite(&suite);
    assert!(report.is_success());
}
