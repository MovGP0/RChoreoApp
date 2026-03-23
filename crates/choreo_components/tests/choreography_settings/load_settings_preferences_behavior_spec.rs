use super::actions::ChoreographySettingsAction;
use super::create_state;
use super::reducer::reduce;

#[test]
fn load_settings_preferences_updates_settings_fields() {
    let mut state = create_state();

    reduce(
        &mut state,
        ChoreographySettingsAction::LoadSettingsPreferences {
            show_timestamps: false,
            positions_at_side: false,
            snap_to_grid: true,
        },
    );

    assert!(!state.show_timestamps);
    assert!(!state.positions_at_side);
    assert!(state.snap_to_grid);
    assert!(!state.choreography.settings.show_timestamps);
    assert!(!state.choreography.settings.positions_at_side);
    assert!(state.choreography.settings.snap_to_grid);
}
