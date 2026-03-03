use time::OffsetDateTime;

#[test]
fn system_clock_now_utc_returns_valid_timestamp() {
    let now = super::time_component::SystemClock::now_utc();
    assert!(now > OffsetDateTime::UNIX_EPOCH);
}

#[test]
fn parse_timestamp_seconds_parses_seconds_only() {
    let result = super::time_component::parse_timestamp_seconds("12.5");
    assert_eq!(result, Some(12.5));
}

#[test]
fn parse_timestamp_seconds_parses_minutes_and_seconds() {
    let result = super::time_component::parse_timestamp_seconds("2:03.250");
    let actual = result.expect("timestamp should parse");
    super::assert_close(actual, 123.25, 0.000_001);
}

#[test]
fn parse_timestamp_seconds_parses_hours_minutes_and_seconds_with_whitespace() {
    let result = super::time_component::parse_timestamp_seconds(" 1:02:03.5 ");
    let actual = result.expect("timestamp should parse");
    super::assert_close(actual, 3723.5, 0.000_001);
}

#[test]
fn parse_timestamp_seconds_rejects_empty_input() {
    let result = super::time_component::parse_timestamp_seconds("   ");
    assert_eq!(result, None);
}

#[test]
fn parse_timestamp_seconds_rejects_more_than_three_parts() {
    let result = super::time_component::parse_timestamp_seconds("1:2:3:4");
    assert_eq!(result, None);
}

#[test]
fn parse_timestamp_seconds_rejects_non_numeric_segment() {
    let result = super::time_component::parse_timestamp_seconds("1:aa");
    assert_eq!(result, None);
}

#[test]
fn format_seconds_rounds_to_three_decimals() {
    let result = super::time_component::format_seconds(1.234_56);
    assert_eq!(result, "1.235");
}

#[test]
fn format_seconds_trims_trailing_zeros() {
    let result = super::time_component::format_seconds(123.200);
    assert_eq!(result, "123.2");
}

#[test]
fn format_seconds_outputs_integer_without_decimal_point() {
    let result = super::time_component::format_seconds(65.0);
    assert_eq!(result, "65");
}

#[test]
fn format_seconds_keeps_zero_for_zero_value() {
    let result = super::time_component::format_seconds(0.0);
    assert_eq!(result, "0");
}
