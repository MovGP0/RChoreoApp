use std::time::Duration;

use choreo_components::material::styling::material_animations::MaterialAnimation;
use choreo_components::material::styling::material_animations::MaterialAnimations;

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

macro_rules! check_close {
    ($errors:expr, $left:expr, $right:expr, $tolerance:expr) => {
        if ($left - $right).abs() > $tolerance {
            $errors.push(format!(
                "{} not within {} of {} (left = {:?}, right = {:?})",
                stringify!($left),
                stringify!($tolerance),
                stringify!($right),
                $left,
                $right
            ));
        }
    };
}

#[test]
fn material_animation_durations_match_slint_contract() {
    let mut errors = Vec::new();

    check_eq!(
        errors,
        MaterialAnimations::EMPHASIZED_DURATION,
        Duration::from_millis(500)
    );
    check_eq!(
        errors,
        MaterialAnimations::EMPHASIZED_DECELERATE_DURATION,
        Duration::from_millis(400)
    );
    check_eq!(
        errors,
        MaterialAnimations::EMPHASIZED_ACCELERATE_DURATION,
        Duration::from_millis(200)
    );
    check_eq!(
        errors,
        MaterialAnimations::STANDARD_DECELERATE_DURATION,
        Duration::from_millis(250)
    );
    check_eq!(
        errors,
        MaterialAnimations::STANDARD_ACCELERATE_DURATION,
        Duration::from_millis(200)
    );
    check_eq!(
        errors,
        MaterialAnimations::STANDARD_FAST_DURATION,
        Duration::from_millis(150)
    );
    check_eq!(errors, MaterialAnimations::RIPPLE_DURATION, Duration::from_secs(2));
    check_eq!(
        errors,
        MaterialAnimations::OPACITY_DURATION,
        Duration::from_millis(250)
    );

    assert!(
        errors.is_empty(),
        "Assertion failures:\n{}",
        errors.join("\n")
    );
}

#[test]
fn standard_easing_tracks_cubic_bezier_0001_shape() {
    let easing = MaterialAnimations::STANDARD_EASING;
    let mut errors = Vec::new();

    check_close!(errors, easing.sample(0.0), 0.0, 0.000_001);
    check_close!(errors, easing.sample(0.5), 0.89, 0.02);
    check_close!(errors, easing.sample(1.0), 1.0, 0.000_001);

    assert!(
        errors.is_empty(),
        "Assertion failures:\n{}",
        errors.join("\n")
    );
}

#[test]
fn emphasized_easing_tracks_cubic_bezier_0050700110_shape() {
    let easing = MaterialAnimations::EMPHASIZED_EASING;
    let mut errors = Vec::new();

    check_close!(errors, easing.sample(0.0), 0.0, 0.000_001);
    check_close!(errors, easing.sample(0.5), 0.966, 0.02);
    check_close!(errors, easing.sample(1.0), 1.0, 0.000_001);

    assert!(
        errors.is_empty(),
        "Assertion failures:\n{}",
        errors.join("\n")
    );
}

#[test]
fn named_animation_specs_map_slint_motion_tokens() {
    let mut errors = Vec::new();

    let emphasized = MaterialAnimations::spec(MaterialAnimation::Emphasized);
    check_eq!(
        errors,
        emphasized.duration,
        MaterialAnimations::EMPHASIZED_DURATION
    );
    check_eq!(errors, emphasized.easing, MaterialAnimations::EMPHASIZED_EASING);

    let emphasized_decelerate = MaterialAnimations::spec(MaterialAnimation::EmphasizedDecelerate);
    check_eq!(
        errors,
        emphasized_decelerate.duration,
        MaterialAnimations::EMPHASIZED_DECELERATE_DURATION
    );
    check_eq!(
        errors,
        emphasized_decelerate.easing,
        MaterialAnimations::EMPHASIZED_EASING
    );

    let emphasized_accelerate = MaterialAnimations::spec(MaterialAnimation::EmphasizedAccelerate);
    check_eq!(
        errors,
        emphasized_accelerate.duration,
        MaterialAnimations::EMPHASIZED_ACCELERATE_DURATION
    );
    check_eq!(
        errors,
        emphasized_accelerate.easing,
        MaterialAnimations::EMPHASIZED_EASING
    );

    let standard_decelerate = MaterialAnimations::spec(MaterialAnimation::StandardDecelerate);
    check_eq!(
        errors,
        standard_decelerate.duration,
        MaterialAnimations::STANDARD_DECELERATE_DURATION
    );
    check_eq!(
        errors,
        standard_decelerate.easing,
        MaterialAnimations::STANDARD_EASING
    );

    let standard_accelerate = MaterialAnimations::spec(MaterialAnimation::StandardAccelerate);
    check_eq!(
        errors,
        standard_accelerate.duration,
        MaterialAnimations::STANDARD_ACCELERATE_DURATION
    );
    check_eq!(
        errors,
        standard_accelerate.easing,
        MaterialAnimations::STANDARD_EASING
    );

    let standard_fast = MaterialAnimations::spec(MaterialAnimation::StandardFast);
    check_eq!(
        errors,
        standard_fast.duration,
        MaterialAnimations::STANDARD_FAST_DURATION
    );
    check_eq!(errors, standard_fast.easing, MaterialAnimations::STANDARD_EASING);

    let ripple = MaterialAnimations::spec(MaterialAnimation::Ripple);
    check_eq!(errors, ripple.duration, MaterialAnimations::RIPPLE_DURATION);
    check_eq!(errors, ripple.easing, MaterialAnimations::RIPPLE_EASING);

    let opacity = MaterialAnimations::spec(MaterialAnimation::Opacity);
    check_eq!(errors, opacity.duration, MaterialAnimations::OPACITY_DURATION);
    check_eq!(errors, opacity.easing, MaterialAnimations::OPACITY_EASING);

    assert!(
        errors.is_empty(),
        "Assertion failures:\n{}",
        errors.join("\n")
    );
}
