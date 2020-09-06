#![cfg(test)]

use anyhow::Result;
use proptest::prelude::*;
use test_case::test_case;

use crate::{
    calendar::Calendar,
    datetime::{DateTime, WaitTarget},
};

#[test_case("10-09-12", 10, 9, 12)]
#[test_case("1-1-1", 1, 1, 1)]
#[test_case("0001-10-12", 1, 10, 12)]
pub fn parses_a_date(string: &str, year: i64, month: u32, day: u32) -> Result<()> {
    let cal = Calendar::default();
    let date = cal.parse(string, None)?.with_calendar(&cal);

    assert_eq!(date.year(), year);
    assert_eq!(date.month(), month);
    assert_eq!(date.month_day(), day);
    assert_eq!(date.hour(), 0);

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
#[test_case("5m20s", 1, 1, 1, 0, 5, 20 ; "combination")]
pub fn parses_a_relative(
    string: &str,
    year: i64,
    month: u32,
    month_day: u32,
    hour: u32,
    minute: u32,
    second: u32,
) -> Result<()> {
    let cal = Calendar::default();
    let date = cal
        .parse(string, Some(DateTime::new(0, 1)))?
        .with_calendar(&cal);

    assert_eq!(date.year(), year);
    assert_eq!(date.month(), month);
    assert_eq!(date.month_day(), month_day);
    assert_eq!(date.hour(), hour);
    assert_eq!(date.minute(), minute);
    assert_eq!(date.second(), second);

    Ok(())
}

#[test_case("10-09-wrong" ; "invalid day")]
#[test_case("1-ouch-100" ; "invalid month")]
#[test_case("1-1-100" ; "out of bounds day")]
#[test_case("1-20-01" ; "out of bounds month")]
pub fn parses_date_graceful_fail(string: &str) -> Result<()> {
    let cal = Calendar::default();
    let date = cal.parse(string, None);

    assert_eq!(date.is_err(), true);

    Ok(())
}

#[test_case(0, "Monday", 1)]
#[test_case(1, "Tuesday", 2)]
#[test_case(2, "Wednesday", 3)]
#[test_case(7, "Monday", 1)]
pub fn week_day_name(day: u32, day_name: &str, week_day: usize) -> Result<()> {
    let cal = Calendar::default();
    let date = DateTime::new(0, 1).with_calendar(&cal).add_hours(day * 24);

    assert_eq!(date.with_calendar(&cal).week_day(), week_day);
    assert_eq!(date.with_calendar(&cal).week_day_name(), day_name);

    Ok(())
}

#[test]
pub fn parses_a_duration() -> Result<()> {
    let cal = Calendar::default();
    let date = cal.parse("3h", Some(DateTime::new(0, 1)))?;

    assert_eq!(date.with_calendar(&cal).hour(), 3);
    assert_eq!(
        date.with_calendar(&cal).era().map(|e| e.name.clone()),
        Some("Common Era".to_string())
    );

    Ok(())
}

#[test]
pub fn extracts_times() -> Result<()> {
    let cal = Calendar::default();
    let date = DateTime::new(90, 1);

    assert_eq!(date.with_calendar(&cal).minute(), 1);
    assert_eq!(date.with_calendar(&cal).second(), 30);

    Ok(())
}

#[test]
pub fn extracts_hours() -> Result<()> {
    let cal = Calendar::default();
    let date = DateTime::new(7290, 1);

    assert_eq!(date.with_calendar(&cal).hour(), 2);
    assert_eq!(date.with_calendar(&cal).minute(), 1);
    assert_eq!(date.with_calendar(&cal).second(), 30);

    Ok(())
}

#[test_case("short rest", 4 ; "short rest")]
#[test_case("long rest", 8 ; "long rest")]
#[test_case("midday", 12 ; "midday")]
#[test_case("midnight", 0 ; "midnight")]
pub fn parses_a_relative_time(string: &str, hour: u32) -> Result<()> {
    let cal = Calendar::default();
    let date = cal.parse(string, Some(DateTime::new(0, 1)))?;

    assert_eq!(date.with_calendar(&cal).hour(), hour);

    Ok(())
}

use crate::datetime::Time;

#[test_case(4, 8 ; "forward")]
#[test_case(1, 14 ; "across noon")]
#[test_case(13, 2 ; "across midnight")]
pub fn sets_time_forward(start_time: u32, target_time: u32) -> Result<()> {
    let cal = Calendar::default();
    let date = DateTime::new(0, 1)
        .with_calendar(&cal)
        .add_hours(start_time);

    let t = WaitTarget::Time(Time {
        hour: target_time,
        ..Default::default()
    });

    let date = date.with_calendar(&cal).wait_until(t)?;
    assert_eq!(date.with_calendar(&cal).hour(), target_time);

    Ok(())
}

#[test_case(86400 * 1,  2 ; "january")]
#[test_case(86400 * 40, 41 ; "february")]
#[test_case(86400 * 95, 96 ; "april")]
pub fn extracts_days(seconds: i64, day: u32) -> Result<()> {
    let cal = Calendar::default();
    let date = DateTime::new(seconds, 1);

    assert_eq!(date.with_calendar(&cal).day(), day);
    assert_eq!(date.with_calendar(&cal).hour(), 0);
    assert_eq!(date.with_calendar(&cal).minute(), 0);
    assert_eq!(date.with_calendar(&cal).second(), 0);

    Ok(())
}

#[test_case(86400 * 1,  1, 2 ; "january")]
#[test_case(86400 * 40, 2, 11 ; "february")]
#[test_case(86400 * 95, 4, 5 ; "april")]
pub fn extracts_month_days(seconds: i64, month: u32, day: u32) -> Result<()> {
    let cal = Calendar::default();
    let date = DateTime::new(seconds, 1);

    assert_eq!(date.with_calendar(&cal).month(), month);
    assert_eq!(date.with_calendar(&cal).month_day(), day);
    assert_eq!(date.with_calendar(&cal).hour(), 0);
    assert_eq!(date.with_calendar(&cal).minute(), 0);
    assert_eq!(date.with_calendar(&cal).second(), 0);

    Ok(())
}

#[test_case(1, 86400 * 30 ; "january")]
#[test_case(2, 86400 * (30 + 31) ; "january + february")]
pub fn months_to_seconds(months: u32, seconds: u32) -> Result<()> {
    let cal = Calendar::default();
    assert_eq!(cal.months_to_seconds(months), seconds);

    Ok(())
}

#[test_case(86400 * 1,  "January" ; "january")]
#[test_case(86400 * 40, "February" ; "february")]
#[test_case(86400 * 95, "April" ; "april")]
pub fn calculates_months(seconds: i64, month: &str) -> Result<()> {
    let cal = Calendar::default();
    let date = DateTime::new(seconds, 1);

    assert_eq!(date.with_calendar(&cal).month_name(), month);

    Ok(())
}

proptest! {
    #[test]
    fn parses_exact_dates(s in "[0-9]{4}-([1-9]|10|11|12){1}-([1-9]|10|11|12){1}") {
        let cal = Calendar::default();
        cal.parse(&s, Some(DateTime::new(0, 1)));
    }

    #[test]
    fn parses_relative_dates(s in "(([0-9]{1,3})([ywdhms]|mo))+") {
        let cal = Calendar::default();
        cal.parse(&s, Some(DateTime::new(0, 1))).unwrap();
    }

    #[test]
    fn parses_valid_times(s in "([0-9]|10|11){1}(am|pm){1}") {
        let cal = Calendar::default();
        cal.parse(&s, Some(DateTime::new(0, 1))).unwrap();
    }

    #[test]
    fn parses_times(s in "([0-9]{1,3})(am|pm){1}") {
        let cal = Calendar::default();
        cal.parse(&s, Some(DateTime::new(0, 1)));
    }
}
