#[test]
fn system_now_unix_nanos_returns_timestamp() {
    let now = super::state::system_now_unix_nanos();
    assert!(now > 0);
}

#[test]
fn parse_timestamp_seconds_parses_seconds_only() {
    let result = super::state::parse_timestamp_seconds("12.5");
    assert_eq!(result, Some(12.5));
}

#[test]
fn parse_timestamp_seconds_parses_minutes_and_seconds() {
    let result = super::state::parse_timestamp_seconds("2:03.250");
    let actual = result.expect("timestamp should parse");
    super::assert_close(actual, 123.25, 0.000_001);
}

#[test]
fn parse_timestamp_seconds_parses_hours_minutes_and_seconds_with_whitespace() {
    let result = super::state::parse_timestamp_seconds(" 1:02:03.5 ");
    let actual = result.expect("timestamp should parse");
    super::assert_close(actual, 3723.5, 0.000_001);
}

#[test]
fn parse_timestamp_seconds_rejects_empty_input() {
    let result = super::state::parse_timestamp_seconds("   ");
    assert_eq!(result, None);
}

#[test]
fn parse_timestamp_seconds_rejects_more_than_three_parts() {
    let result = super::state::parse_timestamp_seconds("1:2:3:4");
    assert_eq!(result, None);
}

#[test]
fn parse_timestamp_seconds_rejects_non_numeric_segment() {
    let result = super::state::parse_timestamp_seconds("1:aa");
    assert_eq!(result, None);
}

#[test]
fn format_seconds_rounds_to_three_decimals() {
    let result = super::state::format_seconds(1.234_56);
    assert_eq!(result, "1.235");
}

#[test]
fn format_seconds_trims_trailing_zeros() {
    let result = super::state::format_seconds(123.200);
    assert_eq!(result, "123.2");
}

#[test]
fn format_seconds_outputs_integer_without_decimal_point() {
    let result = super::state::format_seconds(65.0);
    assert_eq!(result, "65");
}

#[test]
fn format_seconds_keeps_zero_for_zero_value() {
    let result = super::state::format_seconds(0.0);
    assert_eq!(result, "0");
}

#[test]
fn reducer_parses_input_and_formats_value() {
    let mut state = super::state::TimeState::default();
    super::reducer::reduce(
        &mut state,
        super::actions::TimeAction::SetTimestampInput {
            value: "2:03.250".to_string(),
        },
    );
    super::reducer::reduce(&mut state, super::actions::TimeAction::ParseTimestampInput);

    let parsed = state.parsed_seconds.expect("value should parse");
    super::assert_close(parsed, 123.25, 0.000_001);
    assert_eq!(state.formatted_seconds, "123.25");
}

#[test]
fn reducer_initializes_now_timestamp() {
    let mut state = super::state::TimeState::default();
    super::reducer::reduce(&mut state, super::actions::TimeAction::InitializeCurrentTime);
    assert!(state.last_now_unix_nanos > 0);
}

#[test]
fn reducer_sets_seconds_and_formats_value() {
    let mut state = super::state::TimeState::default();
    super::reducer::reduce(
        &mut state,
        super::actions::TimeAction::SetSeconds { value: 42.5 },
    );
    assert_eq!(state.parsed_seconds, Some(42.5));
    assert_eq!(state.formatted_seconds, "42.5");
}
