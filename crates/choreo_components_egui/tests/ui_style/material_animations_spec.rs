use std::time::Duration;

use choreo_components_egui::ui_style::material_animations::MaterialAnimations;

#[test]
fn material_animation_durations_match_slint_contract() {
    assert_eq!(
        MaterialAnimations::EMPHASIZED_DURATION,
        Duration::from_millis(500)
    );
    assert_eq!(
        MaterialAnimations::EMPHASIZED_DECELERATE_DURATION,
        Duration::from_millis(400)
    );
    assert_eq!(
        MaterialAnimations::EMPHASIZED_ACCELERATE_DURATION,
        Duration::from_millis(200)
    );
    assert_eq!(
        MaterialAnimations::STANDARD_DECELERATE_DURATION,
        Duration::from_millis(250)
    );
    assert_eq!(
        MaterialAnimations::STANDARD_ACCELERATE_DURATION,
        Duration::from_millis(200)
    );
    assert_eq!(
        MaterialAnimations::STANDARD_FAST_DURATION,
        Duration::from_millis(150)
    );
    assert_eq!(MaterialAnimations::RIPPLE_DURATION, Duration::from_secs(2));
    assert_eq!(
        MaterialAnimations::OPACITY_DURATION,
        Duration::from_millis(250)
    );
}

#[test]
fn standard_easing_tracks_cubic_bezier_0001_shape() {
    let easing = MaterialAnimations::STANDARD_EASING;
    assert_eq!(easing.sample(0.0), 0.0);
    assert!((easing.sample(0.5) - 0.89).abs() < 0.02);
    assert_eq!(easing.sample(1.0), 1.0);
}

#[test]
fn emphasized_easing_tracks_cubic_bezier_0050700110_shape() {
    let easing = MaterialAnimations::EMPHASIZED_EASING;
    assert_eq!(easing.sample(0.0), 0.0);
    assert!((easing.sample(0.5) - 0.966).abs() < 0.02);
    assert_eq!(easing.sample(1.0), 1.0);
}
