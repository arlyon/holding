use std::fmt::{self, Display, Formatter};

use ordinal::Ordinal;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{Calendar, Era};

mod time_of_day;

pub use time_of_day::TimeOfDay;

/// DateTime is a position in time relative to an
/// era in a given calendar.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Ord, PartialOrd, PartialEq, Eq)]
pub struct DateTime {
    era: usize,
    seconds: i64,
}

#[derive(Error, Debug, Copy, Clone)]
pub enum DateTimeError {
    #[error("month {0} is out of bounds")]
    MonthOutOfBounds(u32),
    #[error("day {0} is out of bounds")]
    DayOutOfBounds(u32),
}

impl DateTime {
    /// Creates a new DateTime object.
    pub fn new(seconds: i64, era: usize) -> Self {
        Self { seconds, era }
    }

    /// Creates a `DateTime` from a date in a given calendar.
    pub fn from_date(
        calendar: &Calendar,
        year: i64,
        month: u32,
        day: u32,
    ) -> Result<Self, DateTimeError> {
        let (era_index, era) = calendar.get_era(year);
        let year = match era.start_year() {
            Some(era_start) => year - era_start,
            None => match era.end_year() {
                Some(era_end) => year - era_end,
                None => year,
            },
        };

        let total_months = calendar.months().len();
        let month = if month as usize <= total_months {
            (month as usize + total_months - 1) % total_months
        } else {
            return Err(DateTimeError::MonthOutOfBounds(month));
        };

        let total_days = calendar.months()[month].days();
        let day = if day <= total_days {
            (day + total_days - 1) % total_days
        } else {
            return Err(DateTimeError::DayOutOfBounds(day));
        };

        // we subtract one here because days, months, and years are all 1-indexed
        let seconds = calendar.years_to_seconds(year - 1)
            + calendar.months_to_seconds(month as u32) as i64
            + calendar.days_to_seconds(day) as i64;

        Ok(Self {
            seconds,
            era: era_index,
        })
    }

    /// Augments this `DateTime` with a calendar, allowing it
    /// to be updated in relation to it.
    pub fn with_calendar<'a>(self, calendar: &'a Calendar) -> CalendarDateTime<'a> {
        CalendarDateTime {
            era: self.era,
            seconds: self.seconds,
            calendar: calendar,
        }
    }
}

/// Allows you to do calendar-aware manipulation
/// of the `DateTime`.
#[derive(Copy, Clone)]
pub struct CalendarDateTime<'a> {
    era: usize,
    seconds: i64,
    calendar: &'a Calendar,
}

#[derive(Error, Debug, Copy, Clone)]
pub enum WaitError {
    #[error("the time is out of bounds")]
    InvalidTime,
}

impl CalendarDateTime<'_> {
    /// Get the current era.
    pub fn era(&self) -> Option<&Era> {
        self.calendar.eras().get(self.era)
    }

    /// Get the current year.
    pub fn year(&self) -> i64 {
        self.calendar.year(self.era, self.seconds)
    }

    /// Get the current day.
    pub fn day(&self) -> u32 {
        self.calendar.day(self.seconds)
    }

    /// Get the current hour.
    pub fn hour(&self) -> u32 {
        self.calendar.hour(self.seconds)
    }

    /// Get the current minute.
    pub fn minute(&self) -> u32 {
        self.calendar.minute(self.seconds)
    }

    /// Get the current second.
    pub fn second(&self) -> u32 {
        self.calendar.second(self.seconds)
    }

    /// Get the number of seconds this date represents modulo another.
    pub fn modulo(&self, other: u32) -> u32 {
        let era = &self.calendar.eras()[self.era];
        let seconds = if let Some(start) = era.start_year() {
            self.calendar.years_to_seconds(start) + self.seconds
        } else {
            self.seconds
        };

        (seconds % other as i64) as u32
    }

    /// Get the ordinal of the current week day.
    pub fn week_day(&self) -> usize {
        self.calendar.week_day(self.era, self.seconds)
    }

    /// Get the name of the current week day.
    pub fn week_day_name(&self) -> &str {
        self.calendar.week_day_name(self.era, self.seconds)
    }

    /// Get the ordinal of the day of the month.
    pub fn month_day(&self) -> u32 {
        self.calendar.month_day(self.seconds)
    }

    /// Get the ordinal of the current month.
    pub fn month(&self) -> u32 {
        self.calendar.month(self.seconds)
    }

    /// Get the name of the current month.
    pub fn month_name(&self) -> &str {
        self.calendar.month_name(self.seconds)
    }

    /// Progress time forward to a given target.
    pub fn wait_until(self, target: WaitTarget) -> Result<DateTime, WaitError> {
        match target {
            WaitTarget::Time(t) => {
                let curr = self.calendar.time(self.seconds);
                let target = t.seconds(self.calendar);
                let day_length = self.calendar.days_to_seconds(1);
                if day_length + curr > target {
                    let difference =
                        (day_length - curr + target) % self.calendar.days_to_seconds(1);
                    Ok(self.add_seconds(difference))
                } else {
                    Err(WaitError::InvalidTime)
                }
            }
            WaitTarget::Midnight => self.wait_until(WaitTarget::Time(Default::default())),
            WaitTarget::Midday => self.wait_until(WaitTarget::Time(Time {
                hour: self.calendar.hours_in_day() / 2,
                ..Default::default()
            })),
        }
    }

    /// Get a new DateTime object with years applied.
    pub fn add_years(self, years: u32) -> DateTime {
        self.add_seconds(self.calendar.years_to_seconds(years as i64) as u32)
    }

    /// Get a new DateTime object with months applied.
    pub fn add_months(self, months: u32) -> DateTime {
        let days = &self
            .calendar
            .months()
            .iter()
            .cycle()
            .skip(self.calendar.month(self.seconds) as usize - 1)
            .take(months as usize)
            .map(|m| m.days())
            .sum();

        self.add_days(*days)
    }

    /// Get a new DateTime object with weeks applied.
    pub fn add_weeks(self, weeks: u32) -> DateTime {
        self.add_days(weeks * self.calendar.days_in_week())
    }

    /// Get a new DateTime object with days applied.
    pub fn add_days(self, days: u32) -> DateTime {
        self.add_seconds(self.calendar.days_to_seconds(days))
    }

    /// Get a new DateTime object with hours applied.
    pub fn add_hours(self, hours: u32) -> DateTime {
        self.add_seconds(self.calendar.hours_to_seconds(hours))
    }

    /// Get a new DateTime object with minutes applied.
    pub fn add_minutes(self, minutes: u32) -> DateTime {
        self.add_seconds(self.calendar.minutes_to_seconds(minutes))
    }

    /// Get a new DateTime object with seconds applied.
    pub fn add_seconds(self, seconds: u32) -> DateTime {
        DateTime {
            era: self.era,
            seconds: self.seconds + seconds as i64,
        }
    }

    /// Get the `TimeOfDay`
    pub fn time_of_day(self) -> TimeOfDay {
        TimeOfDay::from_time(self.hour(), self.calendar.hours_in_day())
    }
}

/// Allows you to wait until a given time.
#[derive(Copy, Clone)]
pub enum WaitTarget {
    Time(Time),
    Midnight,
    Midday,
}

#[derive(Default, Copy, Clone)]
pub struct Time {
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
    pub format: TimeFormat,
}

impl Time {
    /// Converts a `Time` with a relative `TimeFormat` into an `Exact` one.
    pub fn exact(self, calendar: &Calendar) -> Self {
        match self.format {
            TimeFormat::Exact => self,
            TimeFormat::AM => Self {
                format: TimeFormat::Exact,
                ..self
            },
            TimeFormat::PM => Self {
                hour: calendar.hours_in_day() / 2 + self.hour,
                format: TimeFormat::Exact,
                ..self
            },
        }
    }

    /// Get number of seconds from midnight on a given calendar.
    pub fn seconds(&self, calendar: &Calendar) -> u32 {
        let x = self.exact(calendar);
        calendar.hours_to_seconds(x.hour) + calendar.minutes_to_seconds(x.minute) + x.second
    }
}

#[derive(Copy, Clone)]
pub enum TimeFormat {
    /// Military Time
    Exact,
    /// AM
    AM,
    /// PM
    PM,
}

impl Default for TimeFormat {
    fn default() -> Self {
        Self::Exact
    }
}

impl Display for CalendarDateTime<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{:0>2}:{:0>2} on {} the {} of {} year {}",
            self.hour(),
            self.minute(),
            self.week_day_name(),
            Ordinal(self.month_day()),
            self.month_name(),
            self.year(),
        )
    }
}
