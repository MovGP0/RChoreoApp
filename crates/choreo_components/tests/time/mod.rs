pub mod time_mod_spec;

#[path = "../../src/time/mod.rs"]
pub mod time_component;

pub fn assert_close(actual: f64, expected: f64, epsilon: f64) {
    assert!(
        (actual - expected).abs() <= epsilon,
        "expected {expected} +/- {epsilon}, got {actual}"
    );
}
