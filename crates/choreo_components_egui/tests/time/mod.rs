pub mod time_mod_spec;

#[path = "../../src/time/actions.rs"]
pub mod actions;
#[path = "../../src/time/reducer.rs"]
pub mod reducer;
#[path = "../../src/time/state.rs"]
pub mod state;

pub fn assert_close(actual: f64, expected: f64, epsilon: f64) {
    assert!(
        (actual - expected).abs() <= epsilon,
        "expected {expected} +/- {epsilon}, got {actual}"
    );
}
