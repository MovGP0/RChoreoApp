use crate::nav_bar::nav_bar_component::state::InteractionMode;
use crate::nav_bar::nav_bar_component::state::mode_index;
use crate::nav_bar::nav_bar_component::state::mode_option_from_index;

#[test]
fn mode_mapping_round_trips_all_modes() {
    let modes = [
        InteractionMode::View,
        InteractionMode::Move,
        InteractionMode::RotateAroundCenter,
        InteractionMode::RotateAroundDancer,
        InteractionMode::Scale,
        InteractionMode::LineOfSight,
    ];

    for mode in modes {
        let index = mode_index(mode);
        assert!(index >= 0);
        assert_eq!(mode_option_from_index(index), Some(mode));
    }

    assert_eq!(mode_option_from_index(-1), None);
    assert_eq!(mode_option_from_index(99), None);
}
