#![cfg(test)]

use std::{error::Error, result};

use proptest::prelude::*;
use test_case::test_case;

use crate::{
    calendar::{traits::ConvertDate, Calendar},
    datetime::{
        traits::{ModifyDate, ModifyDateTime, ShowDate, ShowTime},
        DateTime, RawDate, RawTime, Time, TimeFormat, WaitTarget,
    },
};

type Result = result::Result<(), Box<dyn Error>>;

#[test_case("10-09-12", 10, 9, 12)]
#[test_case("1-1-1", 1, 1, 1)]
#[test_case("0001-10-12", 1, 10, 12)]
pub fn parses_a_date(string: &str, year: i64, month: u32, day: u32) -> Result {
    let cal = Calendar::default();
    let date = cal.parse(string, None)?;

    assert_eq!(date.year(), year);
    assert_eq!(date.month(), month);
    assert_eq!(date.day(), day);
    assert_eq!(date.hour(), 0);

    Ok(())
}

#[test_case(10, (0, 10))]
#[test_case(40, (1, 9))]
#[test_case(273, (9, 0))]
#[test_case(304, (10, 0))]
#[test_case(370, (12, 5))]
pub fn days_to_months(days: u32, expected: (u32, u32)) -> Result {
    let cal = Calendar::default();
    let result = cal.days_to_months(days);

    assert_eq!(result, expected);
    Ok(())
}

#[test_case("5y", 6, 1, 1, 0, 0, 0 ; "years")]
#[test_case("1mo", 1, 2, 1, 0, 0, 0 ; "1 months")]
#[test_case("2mo", 1, 3, 1, 0, 0, 0 ; "2 months")]
#[test_case("3mo", 1, 4, 1, 0, 0, 0 ; "3 months")]
#[test_case("6mo", 1, 7, 1, 0, 0, 0 ; "6 months")]
#[test_case("9mo", 1, 10, 1, 0, 0, 0 ; "9 months")]
#[test_case("11mo", 1, 12, 1, 0, 0, 0 ; "11 months")]
#[test_case("12mo", 2, 1, 1, 0, 0, 0 ; "12 months")]
#[test_case("14mo", 2, 3, 1, 0, 0, 0 ; "14 months")]
#[test_case("2w", 1, 1, 15, 0, 0, 0 ; "weeks")]
#[test_case("8h", 1, 1, 1, 8, 0, 0 ; "hours")]
#[test_case("20m", 1, 1, 1, 0, 20, 0 ; "minutes")]
#[test_case("20s", 1, 1, 1, 0, 0, 20 ; "seconds")]
#[test_case("2y4mo2h5m20s", 3, 5, 1, 2, 5, 20 ; "combination")]
pub fn parses_a_relative(
    string: &str,
    year: i64,
    month: u32,
    month_day: u32,
    hour: u32,
    minute: u32,
    second: u32,
) -> Result {
    let cal = Calendar::default();
    let date = cal.parse(string, DateTime::from_seconds(0, &cal))?;

    assert_eq!(date.year(), year);
    assert_eq!(date.month(), month);
    assert_eq!(date.day(), month_day);
    assert_eq!(date.hour(), hour);
    assert_eq!(date.minute(), minute);
    assert_eq!(date.second(), second);

    Ok(())
}

#[test_case("10-09-wrong" ; "invalid day")]
#[test_case("1-ouch-100" ; "invalid month")]
#[test_case("1-1-100" ; "out of bounds day")]
#[test_case("1-20-01" ; "out of bounds month")]
pub fn parses_date_graceful_fail(string: &str) {
    let cal = Calendar::default();
    let date = cal.parse(string, None);
    assert_eq!(date.is_err(), true);
}

#[test_case("short rest", 4 ; "short rest")]
#[test_case("long rest", 8 ; "long rest")]
#[test_case("midday", 12 ; "midday")]
#[test_case("midnight", 0 ; "midnight")]
pub fn parses_a_relative_time(string: &str, hour: u32) -> Result {
    let cal = Calendar::default();
    let date = cal.parse(string, DateTime::from_seconds(0, &cal))?;

    assert_eq!(date.hour(), hour);

    Ok(())
}

#[test_case(4, 8 ; "forward")]
#[test_case(1, 14 ; "across noon")]
#[test_case(13, 2 ; "across midnight")]
pub fn sets_time_forward(start_time: u32, target_time: u32) -> Result {
    let cal = Calendar::default();
    let date = DateTime::from_seconds(0, &cal).add_hours(start_time);

    let t = WaitTarget::Time(Time::from_hms(target_time, 0, 0, &cal, TimeFormat::AM)?);

    let date = date.wait_until(t)?;
    assert_eq!(date.hour(), target_time);

    Ok(())
}

#[test_case(1, 86400 * 31 ; "january")]
#[test_case(2, 86400 * (31 + 28) ; "january + february")]
pub fn months_to_seconds(months: u32, seconds: u32) -> Result {
    let cal = Calendar::default();
    assert_eq!(cal.months_to_seconds(months), seconds);

    Ok(())
}

proptest! {
    #[test]
    fn parses_exact_dates(s in "[0-9]{4}-([1-9]|10|11|12){1}-([1-9]|10|11|12){1}") {
        parse_time_test(&s)
    }

    #[test]
    fn parses_durations(s in "(([0-9]{1,3})([ywdhms]|mo))+") {
        parse_time_test(&s)
    }

    #[test]
    fn parses_valid_times(s in "([0-9]|10|11){1}(am|pm){1}") {
        parse_time_test(&s)
    }

    #[test]
    fn parses_times(s in "([0-9]{1,3})(am|pm){1}") {
        parse_time_test(&s)
    }
}

fn parse_time_test(s: &str) {
    let cal = Calendar::default();
    let datetime = DateTime::from_seconds(0, &cal);
    cal.parse(s, datetime);
}
