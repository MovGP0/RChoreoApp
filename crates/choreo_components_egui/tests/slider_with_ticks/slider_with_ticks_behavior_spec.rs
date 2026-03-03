use choreo_components_egui::behavior::Behavior;
use choreo_components_egui::slider_with_ticks::SliderWithTicksBehavior;
use choreo_components_egui::slider_with_ticks::SliderWithTicksViewModel;
use choreo_components_egui::slider_with_ticks::ui::visible_tick_fractions;

#[test]
fn slider_with_ticks_behavior_activates_without_mutating_defaults() {
    let behavior = SliderWithTicksBehavior;
    let view_model =
        SliderWithTicksViewModel::new(vec![Box::new(behavior) as Box<dyn Behavior<_>>]);

    assert_eq!(view_model.minimum, 0.0);
    assert_eq!(view_model.maximum, 1.0);
    assert_eq!(view_model.value, 0.0);
    assert!(view_model.tick_values.is_empty());
    assert!(view_model.tick_color.is_none());
    assert!(view_model.is_enabled);
}

#[test]
fn set_range_clamps_existing_value_to_new_bounds() {
    let mut view_model = SliderWithTicksViewModel::default();
    view_model.value = 5.0;

    view_model.set_range(0.0, 2.0);

    assert_eq!(view_model.minimum, 0.0);
    assert_eq!(view_model.maximum, 2.0);
    assert_eq!(view_model.value, 2.0);
}

#[test]
fn set_value_clamps_to_range() {
    let mut view_model = SliderWithTicksViewModel::default();

    view_model.set_value(-1.0);
    assert_eq!(view_model.value, 0.0);

    view_model.set_value(2.0);
    assert_eq!(view_model.value, 1.0);
}

#[test]
fn visible_tick_fractions_filters_values_outside_range() {
    let fractions = visible_tick_fractions(0.0, 10.0, &[-1.0, 0.0, 2.5, 10.0, 12.0]);
    assert_eq!(fractions, vec![0.0, 0.25, 1.0]);
}
