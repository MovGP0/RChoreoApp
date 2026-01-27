use time::{Date, Month, OffsetDateTime};

pub(crate) fn today_date() -> Date
{
    OffsetDateTime::now_utc().date()
}

pub(crate) fn build_date(year: i32, month: i32, day: i32) -> Option<Date>
{
    if !is_valid_date(year, month, day) {
        return None;
    }

    date_from_parts(year, month, day)
}


fn date_from_parts(year: i32, month: i32, day: i32) -> Option<Date>
{
    let month = month_from_i32(month)?;
    Date::from_calendar_date(year, month, day as u8).ok()
}

fn month_from_i32(month: i32) -> Option<Month>
{
    match month {
        1 => Some(Month::January),
        2 => Some(Month::February),
        3 => Some(Month::March),
        4 => Some(Month::April),
        5 => Some(Month::May),
        6 => Some(Month::June),
        7 => Some(Month::July),
        8 => Some(Month::August),
        9 => Some(Month::September),
        10 => Some(Month::October),
        11 => Some(Month::November),
        12 => Some(Month::December),
        _ => None,
    }
}


fn is_valid_date(year: i32, month: i32, day: i32) -> bool
{
    if year <= 0 {
        return false;
    }

    if !(1..=12).contains(&month) {
        return false;
    }

    if day <= 0 {
        return false;
    }

    day <= days_in_month(year, month)
}

fn days_in_month(year: i32, month: i32) -> i32
{
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => 0,
    }
}

fn is_leap_year(year: i32) -> bool
{
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}
