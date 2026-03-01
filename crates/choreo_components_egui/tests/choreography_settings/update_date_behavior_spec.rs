use super::actions::ChoreographySettingsAction;
use super::create_state;
use super::reducer::reduce;

#[test]
fn update_date_updates_state_date_parts_and_redraw() {
    let mut state = create_state();

    reduce(
        &mut state,
        ChoreographySettingsAction::UpdateDate {
            year: 2026,
            month: 1,
            day: 25,
        },
    );

    assert_eq!(state.date.year, 2026);
    assert_eq!(state.date.month, 1);
    assert_eq!(state.date.day, 25);
    assert_eq!(
        state
            .choreography
            .date
            .map(|date| (date.year(), date.month() as u8, date.day())),
        Some((2026, 1, 25)),
    );
    assert!(state.redraw_requested);
}
