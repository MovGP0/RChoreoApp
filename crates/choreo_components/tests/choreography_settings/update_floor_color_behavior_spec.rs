use super::actions::ChoreographySettingsAction;
use super::color;
use super::create_state;
use super::reducer::reduce;

#[test]
fn update_floor_color_updates_choreography_and_view_state() {
    let mut state = create_state();
    let floor_color = color(255, 12, 34, 56);

    reduce(
        &mut state,
        ChoreographySettingsAction::UpdateFloorColor(floor_color.clone()),
    );

    assert_eq!(state.floor_color, floor_color);
    assert_eq!(state.choreography.settings.floor_color, state.floor_color);
    assert!(state.redraw_requested);
}
