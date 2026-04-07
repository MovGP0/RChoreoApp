use choreo_components::behavior::Behavior;
use choreo_components::slider_with_ticks::SliderWithTicksBehavior;
use choreo_components::slider_with_ticks::SliderWithTicksViewModel;
use choreo_components::slider_with_ticks::ui::slider_value_change_is_dragging;
use choreo_components::slider_with_ticks::ui::visible_tick_fractions;

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
fn slider_with_ticks_behavior_activates_without_mutating_defaults() {
    let behavior = SliderWithTicksBehavior;
    let view_model =
        SliderWithTicksViewModel::new(vec![Box::new(behavior) as Box<dyn Behavior<_>>]);

    let mut errors = Vec::new();

    check_eq!(errors, view_model.minimum, 0.0);
    check_eq!(errors, view_model.maximum, 1.0);
    check_eq!(errors, view_model.value, 0.0);
    check!(errors, view_model.tick_values.is_empty());
    check!(errors, view_model.tick_color.is_none());
    check!(errors, view_model.is_enabled);

    assert_no_errors(errors);
}

#[test]
fn set_range_clamps_existing_value_to_new_bounds() {
    let mut view_model = SliderWithTicksViewModel::default();
    view_model.value = 5.0;

    view_model.set_range(0.0, 2.0);

    let mut errors = Vec::new();

    check_eq!(errors, view_model.minimum, 0.0);
    check_eq!(errors, view_model.maximum, 2.0);
    check_eq!(errors, view_model.value, 2.0);

    assert_no_errors(errors);
}

#[test]
fn set_value_clamps_to_range() {
    let mut view_model = SliderWithTicksViewModel::default();
    let mut errors = Vec::new();

    view_model.set_value(-1.0);
    check_eq!(errors, view_model.value, 0.0);

    view_model.set_value(2.0);
    check_eq!(errors, view_model.value, 1.0);

    assert_no_errors(errors);
}

#[test]
fn visible_tick_fractions_filters_values_outside_range() {
    let fractions = visible_tick_fractions(0.0, 10.0, &[-1.0, 0.0, 2.5, 10.0, 12.0]);
    assert_eq!(fractions, vec![0.0, 0.25, 1.0]);
}

#[test]
fn slider_value_change_is_dragging_is_false_for_keyboard_or_programmatic_changes() {
    assert!(!slider_value_change_is_dragging(false, false, false));
}

#[test]
fn slider_value_change_is_dragging_is_true_when_drag_lifecycle_is_active() {
    assert!(slider_value_change_is_dragging(false, true, false));
    assert!(slider_value_change_is_dragging(true, false, false));
    assert!(slider_value_change_is_dragging(false, false, true));
}
