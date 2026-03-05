use std::time::Duration;

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

    pub const RIPPLE_EASING: CubicBezierEasing = CubicBezierEasing::new(0.0, 0.0, 0.58, 1.0);
    pub const RIPPLE_DURATION: Duration = Duration::from_secs(2);

    pub const OPACITY_EASING: CubicBezierEasing = CubicBezierEasing::new(0.25, 0.1, 0.25, 1.0);
    pub const OPACITY_DURATION: Duration = Duration::from_millis(250);
}
