#[path = "../../src/preferences/actions.rs"]
pub mod actions;
#[path = "../../src/preferences/reducer.rs"]
pub mod reducer;
#[path = "../../src/preferences/state.rs"]
pub mod state;

pub mod clear_preferences_spec;
pub mod initialize_preferences_spec;
pub mod load_preferences_spec;
pub mod remove_preferences_spec;
pub mod set_preferences_spec;
pub mod toggle_preferences_spec;

pub fn create_state() -> state::PreferencesState {
    state::PreferencesState::default()
}
