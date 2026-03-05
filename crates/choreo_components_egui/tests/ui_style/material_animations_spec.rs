use std::time::Duration;

use choreo_components_egui::ui_style::material_animations::MaterialAnimation;
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

#[test]
fn named_animation_specs_map_slint_motion_tokens() {
    let emphasized = MaterialAnimations::spec(MaterialAnimation::Emphasized);
    assert_eq!(emphasized.duration, MaterialAnimations::EMPHASIZED_DURATION);
    assert_eq!(emphasized.easing, MaterialAnimations::EMPHASIZED_EASING);

    let emphasized_decelerate = MaterialAnimations::spec(MaterialAnimation::EmphasizedDecelerate);
    assert_eq!(
        emphasized_decelerate.duration,
        MaterialAnimations::EMPHASIZED_DECELERATE_DURATION
    );
    assert_eq!(
        emphasized_decelerate.easing,
        MaterialAnimations::EMPHASIZED_EASING
    );

    let emphasized_accelerate = MaterialAnimations::spec(MaterialAnimation::EmphasizedAccelerate);
    assert_eq!(
        emphasized_accelerate.duration,
        MaterialAnimations::EMPHASIZED_ACCELERATE_DURATION
    );
    assert_eq!(
        emphasized_accelerate.easing,
        MaterialAnimations::EMPHASIZED_EASING
    );

    let standard_decelerate = MaterialAnimations::spec(MaterialAnimation::StandardDecelerate);
    assert_eq!(
        standard_decelerate.duration,
        MaterialAnimations::STANDARD_DECELERATE_DURATION
    );
    assert_eq!(
        standard_decelerate.easing,
        MaterialAnimations::STANDARD_EASING
    );

    let standard_accelerate = MaterialAnimations::spec(MaterialAnimation::StandardAccelerate);
    assert_eq!(
        standard_accelerate.duration,
        MaterialAnimations::STANDARD_ACCELERATE_DURATION
    );
    assert_eq!(
        standard_accelerate.easing,
        MaterialAnimations::STANDARD_EASING
    );

    let standard_fast = MaterialAnimations::spec(MaterialAnimation::StandardFast);
    assert_eq!(
        standard_fast.duration,
        MaterialAnimations::STANDARD_FAST_DURATION
    );
    assert_eq!(standard_fast.easing, MaterialAnimations::STANDARD_EASING);

    let ripple = MaterialAnimations::spec(MaterialAnimation::Ripple);
    assert_eq!(ripple.duration, MaterialAnimations::RIPPLE_DURATION);
    assert_eq!(ripple.easing, MaterialAnimations::RIPPLE_EASING);

    let opacity = MaterialAnimations::spec(MaterialAnimation::Opacity);
    assert_eq!(opacity.duration, MaterialAnimations::OPACITY_DURATION);
    assert_eq!(opacity.easing, MaterialAnimations::OPACITY_EASING);
}
