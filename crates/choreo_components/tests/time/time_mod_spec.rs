#[path = "mod.rs"]
mod time;

#[test]
fn parse_timestamp_seconds_parses_seconds_only() {
    // arrange
    let subject = "12.5";

    // act
    let result = time::subject::parse_timestamp_seconds(subject);

    // assert
    assert_eq!(result, Some(12.5));
}

#[test]
fn parse_timestamp_seconds_parses_minutes_and_seconds() {
    // arrange
    let subject = "2:03.250";

    // act
    let result = time::subject::parse_timestamp_seconds(subject);

    // assert
    let actual = result.expect("timestamp should parse");
    time::assert_close(actual, 123.25, 0.000_001);
}

#[test]
fn parse_timestamp_seconds_parses_hours_minutes_and_seconds_with_whitespace() {
    // arrange
    let subject = " 1:02:03.5 ";

    // act
    let result = time::subject::parse_timestamp_seconds(subject);

    // assert
    let actual = result.expect("timestamp should parse");
    time::assert_close(actual, 3723.5, 0.000_001);
}

#[test]
fn parse_timestamp_seconds_rejects_empty_input() {
    // arrange
    let subject = "   ";

    // act
    let result = time::subject::parse_timestamp_seconds(subject);

    // assert
    assert_eq!(result, None);
}

#[test]
fn parse_timestamp_seconds_rejects_more_than_three_parts() {
    // arrange
    let subject = "1:2:3:4";

    // act
    let result = time::subject::parse_timestamp_seconds(subject);

    // assert
    assert_eq!(result, None);
}

#[test]
fn parse_timestamp_seconds_rejects_non_numeric_segment() {
    // arrange
    let subject = "1:aa";

    // act
    let result = time::subject::parse_timestamp_seconds(subject);

    // assert
    assert_eq!(result, None);
}

#[test]
fn format_seconds_rounds_to_three_decimals() {
    // arrange
    let subject = 1.234_56;

    // act
    let result = time::subject::format_seconds(subject);

    // assert
    assert_eq!(result, "1.235");
}

#[test]
fn format_seconds_trims_trailing_zeros() {
    // arrange
    let subject = 123.200;

    // act
    let result = time::subject::format_seconds(subject);

    // assert
    assert_eq!(result, "123.2");
}

#[test]
fn format_seconds_outputs_integer_without_decimal_point() {
    // arrange
    let subject = 65.0;

    // act
    let result = time::subject::format_seconds(subject);

    // assert
    assert_eq!(result, "65");
}

#[test]
fn format_seconds_keeps_zero_for_zero_value() {
    // arrange
    let subject = 0.0;

    // act
    let result = time::subject::format_seconds(subject);

    // assert
    assert_eq!(result, "0");
}
