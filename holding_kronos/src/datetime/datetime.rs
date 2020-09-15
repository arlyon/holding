#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use std::fmt::{Display, Formatter, Result as FmtResult};

use thiserror::Error;

use crate::{
    calendar::{traits::ConvertTime, Calendar},
    datetime::{
        date::{Date, InvalidDateError},
        time::{InvalidTimeError, Time},
        traits::{ModifyDate, ModifyDateTime, ModifyTime, ShowDate, ShowTime},
        InvalidWaitError, TimeFormat, WaitTarget,
    },
};

use super::{date::RawDate, time::RawTime};

#[derive(Copy, Clone, Eq, PartialEq, Debug, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
/// A datetime without the calendar (for serialization.)
pub struct RawDateTime {
    /// The date component.
    pub date: RawDate,
    /// The time component.
    pub time: RawTime,
}

impl RawDateTime {
    /// Convert this `RawDate` into a `Date` object.
    pub fn into_datetime(self, cal: &Calendar) -> DateTime {
        DateTime {
            date: self.date.into_date(cal),
            time: self.time.into_time(cal),
        }
    }
}

/// Allows you to do calendar-aware manipulation
/// of the `DateTime`.
#[derive(Copy, Clone, Eq, PartialEq, Debug, PartialOrd)]
pub struct DateTime<'a> {
    /// The date component.
    pub date: Date<'a>,

    /// The time component.
    pub time: Time<'a>,
}

impl<'a> DateTime<'a> {
    /// Create a new `DateTime` from a year-month-day tuple.
    ///
    /// This will return an `InvalidDateTimeError` if any of the supplied
    /// components are outside the bounds of the provided Calendar.
    ///
    /// # Arguments
    ///
    /// * `year` - The number of years relative to 0.
    /// * `month` - The number of months relative to the beginning of that year.
    /// * `day` - The number of days relative to the beginning of that month.
    /// * `calendar` - The calendar this `DateTime` is relative to.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use holding_kronos::calendar::Calendar;
    /// use holding_kronos::datetime::DateTime;
    ///
    /// let cal = Calendar::default();
    /// let time_1 = DateTime::from_date(2020, 1, 4, &cal);
    /// ```
    pub fn from_date(
        year: i64,
        month: u32,
        day: u32,
        calendar: &'a Calendar,
    ) -> Result<Self, InvalidDateTimeError> {
        Ok(Self {
            date: Date::from_ymd(year, month, day, calendar).map_err(InvalidDateTimeError::from)?,
            time: Time::from_seconds(0, calendar),
        })
    }

    /// Create a new `DateTime` representing the number of seconds.
    ///
    /// # Arguments
    ///
    /// * `seconds` - The number of seconds relative to midnight.
    /// * `calendar` - The calendar this `Time` is relative to.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use holding_kronos::calendar::Calendar;
    /// use holding_kronos::datetime::{DateTime, traits::{ShowTime, ModifyDateTime}};
    ///
    /// let cal = Calendar::default();
    /// let time_1 = DateTime::from_seconds(3600, &cal);
    /// let time_2 = time_1.add_hours(1);
    ///
    /// # assert_eq!(time_1.hour(), 1);
    /// # assert_eq!(time_2.hour(), 2);
    /// ```
    ///
    /// todo(arlyon): Get working with negative values.
    pub fn from_seconds(seconds: i64, calendar: &'a Calendar) -> Self {
        Self {
            date: Date::from_seconds(seconds, calendar),
            time: Time::from_seconds(seconds as u32, calendar),
        }
    }

    /// Gets the underlying calendar for this DateTime object.
    pub fn calendar(&self) -> &'a Calendar {
        self.date.calendar
    }

    /// Gets the seconds this `DateTime` represents modulo some other.
    pub fn seconds_modulo(&self, other: u32) -> u32 {
        self.seconds() % other
    }

    /// Progress time forward to a given target.
    pub fn wait_until(self, target: WaitTarget) -> Result<DateTime<'a>, InvalidWaitError> {
        match target {
            WaitTarget::Time(time) => {
                let curr = self.seconds();
                let target = time.seconds();
                let day_length = self.calendar().days_to_seconds(1);
                if day_length + curr > target {
                    let difference = (day_length - curr + target) % day_length;
                    Ok(ModifyDateTime::add_seconds(&self, difference))
                } else {
                    Err(InvalidWaitError::BackwardsWait(
                        time.into(),
                        self.time.into(),
                    ))
                }
            }
            WaitTarget::Midnight => self.wait_until(WaitTarget::Time(
                Time::from_hms(0, 0, 0, self.calendar(), TimeFormat::AM)
                    .map_err(InvalidWaitError::from)?,
            )),
            WaitTarget::Midday => self.wait_until(WaitTarget::Time(
                Time::from_hms(0, 0, 0, self.calendar(), TimeFormat::PM)
                    .map_err(InvalidWaitError::from)?,
            )),
        }
    }
}

impl<'a> ModifyTime for DateTime<'a> {
    type Item = DateTime<'a>;

    fn add_hours(&self, hours: u32) -> (Self::Item, u32) {
        let (time, days) = self.time.add_hours(hours);
        (
            DateTime {
                date: self.date,
                time,
            },
            days,
        )
    }

    fn add_minutes(&self, minutes: u32) -> (Self::Item, u32) {
        let (time, days) = self.time.add_minutes(minutes);
        (
            DateTime {
                date: self.date,
                time,
            },
            days,
        )
    }

    fn add_seconds(&self, seconds: u32) -> (Self::Item, u32) {
        let (time, days) = self.time.add_seconds(seconds);
        (
            DateTime {
                date: self.date,
                time,
            },
            days,
        )
    }
}

impl<'a> ModifyDate for DateTime<'a> {
    type Item = DateTime<'a>;

    fn add_years(self, years: u32) -> Self::Item {
        DateTime {
            date: self.date.add_years(years),
            time: self.time,
        }
    }

    fn add_months(self, months: u32) -> Self::Item {
        DateTime {
            date: self.date.add_months(months),
            time: self.time,
        }
    }

    fn add_weeks(self, weeks: u32) -> Self::Item {
        DateTime {
            date: self.date.add_weeks(weeks),
            time: self.time,
        }
    }

    fn add_days(self, days: u32) -> Self::Item {
        DateTime {
            date: self.date.add_days(days),
            time: self.time,
        }
    }
}

// todo(arlyon) can this be impl automatically?
impl<'a> ModifyDateTime for DateTime<'a> {
    type Item = DateTime<'a>;
    fn add_seconds(&self, seconds: u32) -> Self::Item {
        let (time, days) = ModifyTime::add_seconds(self, seconds);
        time.add_days(days)
    }

    fn add_minutes(&self, minutes: u32) -> Self::Item {
        let (time, days) = ModifyTime::add_minutes(self, minutes);
        time.add_days(days)
    }

    fn add_hours(&self, hours: u32) -> Self::Item {
        let (time, days) = ModifyTime::add_hours(self, hours);
        time.add_days(days)
    }
}

impl ShowDate for DateTime<'_> {
    fn year(&self) -> i64 {
        self.date.year()
    }

    fn month(&self) -> u32 {
        self.date.month()
    }

    fn day(&self) -> u32 {
        self.date.day()
    }

    fn week(&self) -> u32 {
        self.date.week()
    }

    fn month_name(&self) -> &str {
        self.date.month_name()
    }

    fn week_day(&self) -> u32 {
        self.date.week_day()
    }

    fn week_day_name(&self) -> &str {
        self.date.week_day_name()
    }

    fn days(&self) -> u32 {
        self.date.days()
    }
}

impl ShowTime for DateTime<'_> {
    fn hour(&self) -> u32 {
        self.time.hour()
    }

    fn minute(&self) -> u32 {
        self.time.minute()
    }

    fn second(&self) -> u32 {
        self.time.second()
    }

    fn seconds(&self) -> u32 {
        self.time.seconds()
    }

    fn time_of_day(self) -> super::TimeOfDay {
        self.time.time_of_day()
    }
}

impl Display for RawDateTime {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}T{}Z", self.date, self.time)
    }
}

impl Display for DateTime<'_> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let x: RawDateTime = (*self).into();
        x.fmt(f)
    }
}

impl Into<RawDateTime> for DateTime<'_> {
    fn into(self) -> RawDateTime {
        RawDateTime {
            date: self.date.into(),
            time: self.time.into(),
        }
    }
}

/// Possible invalid datetime states.
#[derive(Error, Debug, Copy, Clone)]
#[allow(missing_docs)]
pub enum InvalidDateTimeError {
    #[error("date component is invalid: {0}")]
    InvalidDate(#[from] InvalidDateError),
    #[error("time component is invalid: {0}")]
    InvalidTime(#[from] InvalidTimeError),
}

#[cfg(test)]
mod test {
    use std::{error::Error, result};

    use proptest::prelude::*;
    use test_case::test_case;

    use crate::{
        calendar::traits::DayCycle,
        calendar::{traits::YearCycle, Calendar},
        datetime::{
            traits::{ModifyDate, ShowDate, ShowTime},
            DateTime, RawDate, RawTime,
        },
    };

    type Result = result::Result<(), Box<dyn Error>>;

    #[test_case(0, 1, 1, 1, 0, 0, 0)]
    #[test_case(3600, 1, 1, 1, 1, 0, 0)]
    #[test_case(60, 1, 1, 1, 0, 1, 0)]
    #[test_case(86400, 1, 1, 2, 0, 0, 0)]
    pub fn datetime_from_seconds(
        seconds: i64,
        year: i64,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        second: u32,
    ) -> Result {
        let cal = Calendar::default();
        let datetime = DateTime::from_seconds(seconds, &cal);
        assert_eq!(datetime.date.date, RawDate::new(year, month, day)?);
        assert_eq!(
            datetime.time.time,
            RawTime {
                hour,
                minute,
                second
            }
        );

        Ok(())
    }

    proptest! {
        #[test]
        fn days(days in 0u32..10000) {
            let cal = Calendar::default();
            let date = DateTime::from_seconds(i64::from(days) * 86400, &cal);
            assert_eq!(date.days(), days % date.calendar().days_in_year());
        }

        #[test]
        fn extracts_times(hours in 0u32..23, minutes in 0u32..59, seconds in 0u32..59) {
            let cal = Calendar::default();
            let date = DateTime::from_seconds((hours * cal.seconds_in_hour() + minutes * cal.seconds_in_minute() + seconds).into(), &cal);

            assert_eq!(date.hour(), hours);
            assert_eq!(date.minute(), minutes);
            assert_eq!(date.second(), seconds);
        }
    }

    #[test_case(0, "Monday", 1)]
    #[test_case(1, "Tuesday", 2)]
    #[test_case(2, "Wednesday", 3)]
    #[test_case(7, "Monday", 1)]
    pub fn week_day_name(day: i64, day_name: &str, week_day: u32) -> Result {
        let cal = Calendar::default();
        let date = DateTime::from_seconds(day * 86400, &cal);

        assert_eq!(date.week_day(), week_day);
        assert_eq!(date.week_day_name(), day_name);

        Ok(())
    }

    #[test_case(86400 * 1,  1, 2, "January")]
    #[test_case(86400 * 40, 2, 10, "February")]
    #[test_case(86400 * 95, 4, 6, "April")]
    pub fn extracts_month_days(seconds: i64, month: u32, day: u32, month_name: &str) -> Result {
        let cal = Calendar::default();
        let date = DateTime::from_seconds(seconds, &cal);

        assert_eq!(date.month_name(), month_name);
        assert_eq!(date.month(), month);
        assert_eq!(date.day(), day);
        assert_eq!(date.hour(), 0);
        assert_eq!(date.minute(), 0);
        assert_eq!(date.second(), 0);

        Ok(())
    }
}
