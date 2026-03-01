#[path = "../../src/slider_with_ticks/actions.rs"]
mod actions;
#[path = "../../src/slider_with_ticks/reducer.rs"]
mod reducer;
#[path = "../../src/slider_with_ticks/state.rs"]
mod state;

use egui::Color32;

use actions::SliderWithTicksAction;
use reducer::reduce;
use state::SliderWithTicksState;

#[test]
fn slider_with_ticks_defaults_match_source_component() {
    let state = SliderWithTicksState::new();

    assert_eq!(state.minimum, 0.0);
    assert_eq!(state.maximum, 1.0);
    assert_eq!(state.value, 0.0);
    assert!(state.tick_values.is_empty());
    assert!(state.tick_color.is_none());
    assert!(state.is_enabled);
}

#[test]
fn set_range_clamps_existing_value_to_new_bounds() {
    let mut state = SliderWithTicksState::new();
    state.value = 5.0;

    reduce(
        &mut state,
        SliderWithTicksAction::SetRange {
            minimum: 0.0,
            maximum: 2.0,
        },
    );

    assert_eq!(state.minimum, 0.0);
    assert_eq!(state.maximum, 2.0);
    assert_eq!(state.value, 2.0);
}

#[test]
fn set_value_clamps_to_range() {
    let mut state = SliderWithTicksState::new();

    reduce(&mut state, SliderWithTicksAction::SetValue { value: -1.0 });
    assert_eq!(state.value, 0.0);

    reduce(&mut state, SliderWithTicksAction::SetValue { value: 2.0 });
    assert_eq!(state.value, 1.0);
}

#[test]
fn set_ticks_color_and_enabled_updates_state() {
    let mut state = SliderWithTicksState::new();

    reduce(
        &mut state,
        SliderWithTicksAction::SetTickValues {
            tick_values: vec![0.0, 0.5, 1.0],
        },
    );
    reduce(
        &mut state,
        SliderWithTicksAction::SetTickColor {
            tick_color: Some(Color32::from_rgb(255, 0, 0)),
        },
    );
    reduce(
        &mut state,
        SliderWithTicksAction::SetEnabled { is_enabled: false },
    );

    assert_eq!(state.tick_values, vec![0.0, 0.5, 1.0]);
    assert_eq!(state.tick_color, Some(Color32::from_rgb(255, 0, 0)));
    assert!(!state.is_enabled);
}

#[test]
fn initialize_action_is_supported() {
    let mut state = SliderWithTicksState::new();
    reduce(&mut state, SliderWithTicksAction::Initialize);
    assert_eq!(state.value, 0.0);
}
