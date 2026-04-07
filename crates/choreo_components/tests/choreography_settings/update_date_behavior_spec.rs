use super::actions::ChoreographySettingsAction;
use super::create_state;
use super::reducer::reduce;

macro_rules! check_eq {
    ($errors:expr, $left:expr, $right:expr) => {
        if $left != $right {
            $errors.push(format!(
                "{} != {} (left = {:?}, right = {:?})",
                stringify!($left),
                stringify!($right),
                $left,
                $right
            ));
        }
    };
}

macro_rules! check {
    ($errors:expr, $condition:expr) => {
        if !$condition {
            $errors.push(format!("condition failed: {}", stringify!($condition)));
        }
    };
}

fn assert_no_errors(errors: Vec<String>) {
    assert!(
        errors.is_empty(),
        "Assertion failures:\n{}",
        errors.join("\n")
    );
}

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

    let mut errors = Vec::new();

    check_eq!(errors, state.date.year, 2026);
    check_eq!(errors, state.date.month, 1);
    check_eq!(errors, state.date.day, 25);
    check_eq!(
        errors,
        state
            .choreography
            .date
            .map(|date| (date.year(), date.month() as u8, date.day())),
        Some((2026, 1, 25)),
    );
    check!(errors, state.redraw_requested);

    assert_no_errors(errors);
}

#[test]
fn update_date_ignores_invalid_calendar_dates() {
    let mut state = create_state();
    let original_date = state.date;
    let original_model_date = state
        .choreography
        .date
        .map(|date| (date.year(), date.month() as u8, date.day()));

    reduce(
        &mut state,
        ChoreographySettingsAction::UpdateDate {
            year: 2026,
            month: 2,
            day: 31,
        },
    );

    let mut errors = Vec::new();

    check_eq!(errors, state.date, original_date);
    check_eq!(
        errors,
        state
            .choreography
            .date
            .map(|date| (date.year(), date.month() as u8, date.day())),
        original_model_date,
    );
    check!(errors, !state.redraw_requested);

    assert_no_errors(errors);
}
