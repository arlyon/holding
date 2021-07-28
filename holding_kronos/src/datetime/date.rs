use std::{convert::TryInto, fmt::Display};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    calendar::{
        traits::{ConvertDate, ConvertTime, WeekCycle, YearCycle},
        Calendar,
    },
    datetime::traits::{ModifyDate, ShowDate},
    util::div_rem,
};

/// A raw date object (useful for serialization).
#[derive(Debug, Clone, Copy, Ord, PartialOrd, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RawDate {
    /// The year component
    pub year: i64,
    /// The month component
    pub month: u32,
    /// The day component
    pub day: u32,
}

impl RawDate {
    /// Create a new `RawDate` from a year-month-day tuple.
    ///
    /// This will return an `InvalidDateError` if the month or day are 0.
    ///
    /// # Arguments
    ///
    /// * `year`     - The number of years relative to 0.
    /// * `month`    - The number of months relative to the beginning of that year.
    ///                This is 1-indexed.
    /// * `day`      - The number of days relative to the beginning of that month.
    ///                This is 1-indexed.
    pub fn new(year: i64, month: u32, day: u32) -> Result<Self, InvalidDateError> {
        if month == 0 {
            Err(InvalidDateError::MonthOutOfBounds(month))
        } else if day == 0 {
            Err(InvalidDateError::DayOutOfBounds(day))
        } else {
            Ok(Self {
                year,
                month: month - 1,
                day: day - 1,
            })
        }
    }

    /// Convert this `RawDate` into a `Date` object.
    pub fn into_date(self, cal: &Calendar) -> Date {
        Date {
            date: self,
            calendar: cal,
        }
    }
}

/// Print the raw date following the convention that the first month
/// is called 'month 1'.
impl Display for RawDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{:0>4}-{:0>2}-{:0>2}",
            self.year,
            self.month + 1,
            self.day + 1
        )
    }
}

/// Represents dates in arbitrary calendars.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Date<'a> {
    /// The date component.
    pub date: RawDate,

    /// The calendar this date is relative to.
    pub calendar: &'a Calendar,
}

impl<'a> Date<'a> {
    /// Create a new `Date` from a year-month-day tuple.
    ///
    /// This will return an `InvalidDateError` if any of the supplied
    /// components are outside the bounds of the provided Calendar.
    ///
    /// # Arguments
    ///
    /// * `year`     - The number of years relative to 0.
    /// * `month`    - The number of months relative to the beginning of that year.
    ///                This is 1-indexed.
    /// * `day`      - The number of days relative to the beginning of that month.
    ///                This is 1-indexed.
    /// * `calendar` - The calendar this `DateTime` is relative to.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use holding_kronos::calendar::Calendar;
    /// use holding_kronos::datetime::Date;
    ///
    /// let cal = Calendar::default();
    /// let date = Date::from_ymd(2020, 1, 4, &cal);
    /// ```
    pub fn from_ymd(
        year: i64,
        month: u32,
        day: u32,
        calendar: &'a Calendar,
    ) -> Result<Self, InvalidDateError> {
        let date = Date {
            date: RawDate::new(year, month, day)?,
            calendar,
        };

        calendar.validate_date(&date)?;
        Ok(date)
    }

    /// Create a new `Date` representing the number of seconds
    /// relative to the year 0001-01-01.
    ///
    /// # Arguments
    ///
    /// * `seconds` - The number of seconds relative to midnight.
    /// * `calendar` - The calendar this `Time` is relative to.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use holding_kronos::{
    /// #    calendar::Calendar,
    /// #    datetime::Date,
    /// # };
    /// #
    /// let cal = Calendar::default();
    /// let date = Date::from_seconds(60, &cal);
    /// ```
    pub fn from_seconds(seconds: i64, calendar: &'a Calendar) -> Self {
        let total_days = seconds / i64::from(calendar.days_to_seconds(1));
        let (years, year_days) = div_rem(total_days, i64::from(calendar.days_in_year()));
        let (month, day) = calendar.days_to_months(year_days as u32);
        Date::from_ymd(years + 1, month + 1, day + 1, calendar)
            .expect("If this is out of bounds it is a programmer error")
    }

    // given a number of days into the year, gets the month component
    // and the year component.
    fn calculate_month_and_day(&self, days: u32) -> (u32, u32) {
        let mut months = self.calendar.months().iter().cycle().enumerate();
        let mut delta_days = days;
        loop {
            let (delta_months, month) = months.next().expect("This is on a cycle");
            let month_days = month.days;
            if month_days > delta_days {
                return (delta_months as u32 + 1, delta_days + 1);
            } else {
                delta_days -= month_days;
            }
        }
    }
}

impl PartialOrd for Date<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.calendar == other.calendar {
            self.date.partial_cmp(&other.date)
        } else {
            None
        }
    }
}

impl<'a> ShowDate for Date<'a> {
    fn year(&self) -> i64 {
        self.date.year
    }

    fn month(&self) -> u32 {
        self.date.month + 1
    }

    fn day(&self) -> u32 {
        self.date.day + 1
    }

    fn days(&self) -> u32 {
        self.calendar
            .months()
            .iter()
            .take(self.date.month as usize)
            .map(|m| m.days)
            .sum::<u32>()
            + self.date.day
    }

    fn week_day(&self) -> u32 {
        let week_length: i64 = self
            .calendar
            .week_days()
            .len()
            .try_into()
            .expect("This always fits inside an i64");
        let year_length = i64::from(self.calendar.days_in_year());
        let days_so_far = year_length * (self.date.year - 1) + i64::from(self.days());
        let week_day: u32 = (days_so_far % week_length)
            .try_into()
            .expect("This should be positive");

        week_day + 1
    }

    fn week_day_name(&self) -> &str {
        &self.calendar.week_days()[self.week_day() as usize - 1].name
    }

    fn month_name(&self) -> &str {
        &self.calendar.months()[self.month() as usize - 1].name
    }

    fn week(&self) -> u32 {
        let days = self.year().abs() as u64 * u64::from(self.calendar.days_in_year())
            + u64::from(self.days());

        (days % u64::from(self.calendar.days_in_week())) as u32
    }
}

impl<'a> ModifyDate for Date<'a> {
    type Item = Date<'a>;

    fn add_years(self, years: u32) -> Self::Item {
        Date {
            calendar: self.calendar,
            date: RawDate {
                year: self.date.year + i64::from(years),
                ..self.date
            },
        }
    }

    fn add_months(self, months: u32) -> Self::Item {
        let (year, month) = div_rem(months, self.calendar.months().len() as u32);
        Date {
            date: RawDate {
                year: self.date.year + i64::from(year),
                month: self.date.month + month,
                day: self.date.day,
            },
            calendar: self.calendar,
        }
    }

    fn add_weeks(self, weeks: u32) -> Self::Item {
        self.add_days(weeks * self.calendar.days_in_week())
    }

    fn add_days(self, days: u32) -> Self::Item {
        if days == 0 {
            return self;
        }

        // get the number of years and days into the year
        let (delta_year, delta_days) = div_rem(self.days() + days, self.calendar.days_in_year());
        let year = self.year() + i64::from(delta_year);

        // get month and day component
        let (month, day) = self.calculate_month_and_day(delta_days);

        Date {
            date: RawDate::new(year, month, day).expect("Programmer error"),
            calendar: self.calendar,
        }
    }
}

impl From<Date<'_>> for RawDate {
    fn from(val: Date<'_>) -> Self {
        val.date
    }
}

impl Display for Date<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.date.fmt(f)
    }
}

/// Possible invalid states for dates.
#[derive(Error, Debug, Copy, Clone)]
#[allow(missing_docs)]
pub enum InvalidDateError {
    #[error("month {0} is out of bounds")]
    MonthOutOfBounds(u32),
    #[error("day {0} is out of bounds")]
    DayOutOfBounds(u32),
}

#[cfg(test)]
mod test {
    use proptest::prelude::*;
    use test_case::test_case;

    use crate::{
        calendar::traits::YearCycle,
        calendar::Calendar,
        datetime::{
            traits::{ModifyDate, ShowDate},
            Date,
        },
    };

    #[test_case(1, 1, 1, 2)]
    #[test_case(5, 1, 1, 6)]
    #[test_case(50, 1, 2, 20)]
    #[test_case(400, 2, 2, 5)]
    pub fn add_day(add: u32, year: i64, month: u32, day: u32) {
        let cal = Calendar::default();
        let date = Date::from_seconds(0, &cal).add_days(add);
        assert_eq!(
            date.date,
            Date::from_ymd(year, month, day, &cal).unwrap().date
        );
    }

    proptest! {
        #[test]
        fn days(days in 0u32..10000) {
            let cal = Calendar::default();
            let date = Date::from_seconds(i64::from(days) * 86400, &cal);
            assert_eq!(date.days(), days % date.calendar.days_in_year());
        }
    }

    // #[test_case(0, "Monday", 1)]
    // #[test_case(1, "Tuesday", 2)]
    // #[test_case(2, "Wednesday", 3)]
    // #[test_case(7, "Monday", 1)]
    // pub fn test_add_week() {}

    #[test_case(1, 1, 2, 1)]
    #[test_case(5, 1, 6, 1)]
    #[test_case(14, 2, 3, 1)]
    pub fn add_month(add: u32, year: i64, month: u32, day: u32) {
        let cal = Calendar::default();
        let date = Date::from_seconds(0, &cal).add_months(add);
        assert_eq!(
            date.date,
            Date::from_ymd(year, month, day, &cal).unwrap().date
        );
    }

    #[test_case(1, 2, 1, 1)]
    #[test_case(5, 6, 1, 1)]
    #[test_case(120, 121, 1, 1)]
    pub fn add_year(add: u32, year: i64, month: u32, day: u32) {
        let cal = Calendar::default();
        let date = Date::from_seconds(0, &cal).add_years(add);
        assert_eq!(
            date.date,
            Date::from_ymd(year, month, day, &cal).unwrap().date
        );
    }

    #[test_case(86400, 1, 1, 2)]
    #[test_case(0, 1, 1, 1)]
    pub fn from_seconds(seconds: i64, year: i64, month: u32, day: u32) {
        let cal = Calendar::default();
        let date = Date::from_seconds(seconds, &cal);
        assert_eq!(
            date.date,
            Date::from_ymd(year, month, day, &cal).unwrap().date
        );
    }
}
