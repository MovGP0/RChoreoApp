use std::time::Duration;

use egui::Context;
use egui::Id;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CubicBezierEasing {
    p1x: f32,
    p1y: f32,
    p2x: f32,
    p2y: f32,
}

impl CubicBezierEasing {
    pub const fn new(p1x: f32, p1y: f32, p2x: f32, p2y: f32) -> Self {
        Self { p1x, p1y, p2x, p2y }
    }

    /// CSS `ease`.
    pub const fn ease() -> Self {
        Self::new(0.25, 0.1, 0.25, 1.0)
    }

    /// CSS `ease-out`.
    pub const fn ease_out() -> Self {
        Self::new(0.0, 0.0, 0.58, 1.0)
    }

    #[must_use]
    pub fn sample(self, time_fraction: f32) -> f32 {
        let x = time_fraction.clamp(0.0, 1.0);
        if x <= 0.0 {
            return 0.0;
        }
        if x >= 1.0 {
            return 1.0;
        }

        let parameter = self.solve_curve_x(x);
        self.sample_curve_y(parameter).clamp(0.0, 1.0)
    }

    fn sample_curve_x(self, t: f32) -> f32 {
        let one_minus_t = 1.0 - t;
        3.0 * one_minus_t * one_minus_t * t * self.p1x
            + 3.0 * one_minus_t * t * t * self.p2x
            + t * t * t
    }

    fn sample_curve_y(self, t: f32) -> f32 {
        let one_minus_t = 1.0 - t;
        3.0 * one_minus_t * one_minus_t * t * self.p1y
            + 3.0 * one_minus_t * t * t * self.p2y
            + t * t * t
    }

    fn sample_curve_derivative_x(self, t: f32) -> f32 {
        let one_minus_t = 1.0 - t;
        3.0 * one_minus_t * one_minus_t * self.p1x
            + 6.0 * one_minus_t * t * (self.p2x - self.p1x)
            + 3.0 * t * t * (1.0 - self.p2x)
    }

    fn solve_curve_x(self, x: f32) -> f32 {
        let mut t = x;
        let mut iteration = 0;
        while iteration < 6 {
            let x_delta = self.sample_curve_x(t) - x;
            if x_delta.abs() < 1e-5 {
                return t;
            }
            let derivative = self.sample_curve_derivative_x(t);
            if derivative.abs() < 1e-6 {
                break;
            }
            t = (t - x_delta / derivative).clamp(0.0, 1.0);
            iteration += 1;
        }

        let mut start = 0.0;
        let mut end = 1.0;
        let mut binary_iteration = 0;
        while binary_iteration < 12 {
            let mid = (start + end) * 0.5;
            let mid_x = self.sample_curve_x(mid);
            if (mid_x - x).abs() < 1e-5 {
                return mid;
            }
            if mid_x < x {
                start = mid;
            } else {
                end = mid;
            }
            binary_iteration += 1;
        }

        (start + end) * 0.5
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaterialAnimation {
    Emphasized,
    EmphasizedDecelerate,
    EmphasizedAccelerate,
    StandardDecelerate,
    StandardAccelerate,
    StandardFast,
    Ripple,
    Opacity,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MaterialAnimationSpec {
    pub duration: Duration,
    pub easing: CubicBezierEasing,
}

impl MaterialAnimationSpec {
    pub const fn new(duration: Duration, easing: CubicBezierEasing) -> Self {
        Self { duration, easing }
    }

    #[must_use]
    pub fn sample(self, time_fraction: f32) -> f32 {
        self.easing.sample(time_fraction)
    }

    #[must_use]
    pub fn animate_bool(self, ctx: &Context, id: Id, value: bool) -> f32 {
        let progress = ctx.animate_bool_with_time(id, value, self.duration.as_secs_f32());
        self.sample(progress)
    }

    #[must_use]
    pub fn animate_value(self, ctx: &Context, id: Id, value: f32) -> f32 {
        let progress = ctx.animate_value_with_time(id, value, self.duration.as_secs_f32());
        self.sample(progress)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MaterialAnimationTokens {
    pub emphasized_easing: CubicBezierEasing,
    pub emphasized_duration: Duration,
    pub emphasized_decelerate_duration: Duration,
    pub emphasized_accelerate_duration: Duration,
    pub standard_easing: CubicBezierEasing,
    pub standard_decelerate_duration: Duration,
    pub standard_accelerate_duration: Duration,
    pub standard_fast_duration: Duration,
    pub ripple_easing: CubicBezierEasing,
    pub ripple_duration: Duration,
    pub opacity_easing: CubicBezierEasing,
    pub opacity_duration: Duration,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MaterialAnimations;

impl MaterialAnimations {
    pub const EMPHASIZED_EASING: CubicBezierEasing = CubicBezierEasing::new(0.05, 0.7, 0.1, 1.0);
    pub const EMPHASIZED_DURATION: Duration = Duration::from_millis(500);
    pub const EMPHASIZED_DECELERATE_DURATION: Duration = Duration::from_millis(400);
    pub const EMPHASIZED_ACCELERATE_DURATION: Duration = Duration::from_millis(200);

    pub const STANDARD_EASING: CubicBezierEasing = CubicBezierEasing::new(0.0, 0.0, 0.0, 1.0);
    pub const STANDARD_DECELERATE_DURATION: Duration = Duration::from_millis(250);
    pub const STANDARD_ACCELERATE_DURATION: Duration = Duration::from_millis(200);
    pub const STANDARD_FAST_DURATION: Duration = Duration::from_millis(150);

    pub const RIPPLE_EASING: CubicBezierEasing = CubicBezierEasing::ease_out();
    pub const RIPPLE_DURATION: Duration = Duration::from_secs(2);

    pub const OPACITY_EASING: CubicBezierEasing = CubicBezierEasing::ease();
    pub const OPACITY_DURATION: Duration = Duration::from_millis(250);

    #[must_use]
    pub const fn tokens() -> MaterialAnimationTokens {
        MaterialAnimationTokens {
            emphasized_easing: Self::EMPHASIZED_EASING,
            emphasized_duration: Self::EMPHASIZED_DURATION,
            emphasized_decelerate_duration: Self::EMPHASIZED_DECELERATE_DURATION,
            emphasized_accelerate_duration: Self::EMPHASIZED_ACCELERATE_DURATION,
            standard_easing: Self::STANDARD_EASING,
            standard_decelerate_duration: Self::STANDARD_DECELERATE_DURATION,
            standard_accelerate_duration: Self::STANDARD_ACCELERATE_DURATION,
            standard_fast_duration: Self::STANDARD_FAST_DURATION,
            ripple_easing: Self::RIPPLE_EASING,
            ripple_duration: Self::RIPPLE_DURATION,
            opacity_easing: Self::OPACITY_EASING,
            opacity_duration: Self::OPACITY_DURATION,
        }
    }

    #[must_use]
    pub const fn spec(animation: MaterialAnimation) -> MaterialAnimationSpec {
        match animation {
            MaterialAnimation::Emphasized => {
                MaterialAnimationSpec::new(Self::EMPHASIZED_DURATION, Self::EMPHASIZED_EASING)
            }
            MaterialAnimation::EmphasizedDecelerate => MaterialAnimationSpec::new(
                Self::EMPHASIZED_DECELERATE_DURATION,
                Self::EMPHASIZED_EASING,
            ),
            MaterialAnimation::EmphasizedAccelerate => MaterialAnimationSpec::new(
                Self::EMPHASIZED_ACCELERATE_DURATION,
                Self::EMPHASIZED_EASING,
            ),
            MaterialAnimation::StandardDecelerate => MaterialAnimationSpec::new(
                Self::STANDARD_DECELERATE_DURATION,
                Self::STANDARD_EASING,
            ),
            MaterialAnimation::StandardAccelerate => MaterialAnimationSpec::new(
                Self::STANDARD_ACCELERATE_DURATION,
                Self::STANDARD_EASING,
            ),
            MaterialAnimation::StandardFast => {
                MaterialAnimationSpec::new(Self::STANDARD_FAST_DURATION, Self::STANDARD_EASING)
            }
            MaterialAnimation::Ripple => {
                MaterialAnimationSpec::new(Self::RIPPLE_DURATION, Self::RIPPLE_EASING)
            }
            MaterialAnimation::Opacity => {
                MaterialAnimationSpec::new(Self::OPACITY_DURATION, Self::OPACITY_EASING)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::CubicBezierEasing;
    use super::MaterialAnimations;

    #[test]
    fn css_named_easing_helpers_match_slint_defaults() {
        assert_eq!(
            CubicBezierEasing::ease(),
            MaterialAnimations::OPACITY_EASING
        );
        assert_eq!(
            CubicBezierEasing::ease_out(),
            MaterialAnimations::RIPPLE_EASING
        );
    }

    #[test]
    fn token_snapshot_matches_slint_global_material_animations() {
        let tokens = MaterialAnimations::tokens();

        assert_eq!(
            tokens.emphasized_easing,
            MaterialAnimations::EMPHASIZED_EASING
        );
        assert_eq!(tokens.emphasized_duration, Duration::from_millis(500));
        assert_eq!(
            tokens.emphasized_decelerate_duration,
            Duration::from_millis(400)
        );
        assert_eq!(
            tokens.emphasized_accelerate_duration,
            Duration::from_millis(200)
        );
        assert_eq!(tokens.standard_easing, MaterialAnimations::STANDARD_EASING);
        assert_eq!(
            tokens.standard_decelerate_duration,
            Duration::from_millis(250)
        );
        assert_eq!(
            tokens.standard_accelerate_duration,
            Duration::from_millis(200)
        );
        assert_eq!(tokens.standard_fast_duration, Duration::from_millis(150));
        assert_eq!(tokens.ripple_easing, CubicBezierEasing::ease_out());
        assert_eq!(tokens.ripple_duration, Duration::from_secs(2));
        assert_eq!(tokens.opacity_easing, CubicBezierEasing::ease());
        assert_eq!(tokens.opacity_duration, Duration::from_millis(250));
    }

    #[test]
    fn spec_returns_the_matching_token_for_each_animation_variant() {
        use super::MaterialAnimation;
        use super::MaterialAnimationSpec;

        let cases = [
            (
                MaterialAnimation::Emphasized,
                MaterialAnimationSpec::new(
                    MaterialAnimations::EMPHASIZED_DURATION,
                    MaterialAnimations::EMPHASIZED_EASING,
                ),
            ),
            (
                MaterialAnimation::EmphasizedDecelerate,
                MaterialAnimationSpec::new(
                    MaterialAnimations::EMPHASIZED_DECELERATE_DURATION,
                    MaterialAnimations::EMPHASIZED_EASING,
                ),
            ),
            (
                MaterialAnimation::EmphasizedAccelerate,
                MaterialAnimationSpec::new(
                    MaterialAnimations::EMPHASIZED_ACCELERATE_DURATION,
                    MaterialAnimations::EMPHASIZED_EASING,
                ),
            ),
            (
                MaterialAnimation::StandardDecelerate,
                MaterialAnimationSpec::new(
                    MaterialAnimations::STANDARD_DECELERATE_DURATION,
                    MaterialAnimations::STANDARD_EASING,
                ),
            ),
            (
                MaterialAnimation::StandardAccelerate,
                MaterialAnimationSpec::new(
                    MaterialAnimations::STANDARD_ACCELERATE_DURATION,
                    MaterialAnimations::STANDARD_EASING,
                ),
            ),
            (
                MaterialAnimation::StandardFast,
                MaterialAnimationSpec::new(
                    MaterialAnimations::STANDARD_FAST_DURATION,
                    MaterialAnimations::STANDARD_EASING,
                ),
            ),
            (
                MaterialAnimation::Ripple,
                MaterialAnimationSpec::new(
                    MaterialAnimations::RIPPLE_DURATION,
                    MaterialAnimations::RIPPLE_EASING,
                ),
            ),
            (
                MaterialAnimation::Opacity,
                MaterialAnimationSpec::new(
                    MaterialAnimations::OPACITY_DURATION,
                    MaterialAnimations::OPACITY_EASING,
                ),
            ),
        ];

        for (animation, expected) in cases {
            assert_eq!(MaterialAnimations::spec(animation), expected);
        }
    }
}
