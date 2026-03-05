use choreo_components_egui::number_picker::ui::clamped_value;
use choreo_components_egui::number_picker::ui::decrease_value;
use choreo_components_egui::number_picker::ui::increase_value;
use choreo_components_egui::number_picker::ui::normalized_step;

#[test]
fn normalized_step_matches_slint_minimum_of_one() {
    assert_eq!(normalized_step(-3), 1);
    assert_eq!(normalized_step(0), 1);
    assert_eq!(normalized_step(7), 7);
}

#[test]
fn increase_and_decrease_clamp_to_bounds() {
    assert_eq!(decrease_value(1, 1, 100, 1), None);
    assert_eq!(increase_value(100, 1, 100, 1), None);
    assert_eq!(decrease_value(11, 1, 100, 5), Some(6));
    assert_eq!(increase_value(96, 1, 100, 5), Some(100));
}

#[test]
fn clamped_value_respects_reversed_bounds() {
    assert_eq!(clamped_value(50, 100, 1), 50);
    assert_eq!(clamped_value(-5, 100, 1), 1);
    assert_eq!(clamped_value(200, 100, 1), 100);
}
