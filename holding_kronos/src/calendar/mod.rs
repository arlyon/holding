//! calendar: A general-purpose flexible fantasy calendar.
//!
//! For simplicity, it does not support leap-anything.

use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::datetime::Time;
use crate::datetime::{DateTime, DateTimeError, TimeFormat, WaitError, WaitTarget};

mod era;
mod test;

pub use era::{Era, Eras};

lazy_static! {
    static ref DATE: Regex = Regex::new(r"^(?P<year>\d+)-(?P<month>\d+)-(?P<day>\d+)$").unwrap();
    static ref RELATIVE: Regex = Regex::new(r"(?P<value>\d+)(?P<suffix>(mo|[ywdhms]))").unwrap();
    static ref TIME: Regex = Regex::new(r"^(?P<value>\d+)(?P<suffix>(am|pm))$").unwrap();
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Month {
    name: String,
    days: u32,
}

impl Month {
    pub fn days(&self) -> u32 {
        self.days
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Week(Vec<String>);

/// Allows manipulation of dates.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Calendar {
    eras: Eras,
    months: Vec<Month>,
    week_days: Week,

    seconds_in_minute: u32,
    minutes_in_hour: u32,
    hours_in_day: u32,
}

impl Calendar {
    /// Creates a new Calendar given a number of months, week days, and eras.
    pub fn new(months: Vec<Month>, week_days: Week, eras: Eras) -> Self {
        Self {
            eras,
            months,
            week_days,
            seconds_in_minute: 60,
            minutes_in_hour: 60,
            hours_in_day: 24,
        }
    }

    /// Gets the hours in the day for this calendar.
    pub fn hours_in_day(&self) -> u32 {
        self.hours_in_day
    }

    /// Gets the days in a year for this calendar.
    pub fn days_in_year(&self) -> u32 {
        self.months.iter().map(|month| month.days).sum()
    }

    /// Gets the days in a year for this calendar.
    pub fn days_in_week(&self) -> u32 {
        self.week_days.0.len() as u32
    }

    /// Gets the number of seconds corresponding to a number of years.
    pub fn years_to_seconds(&self, years: i64) -> i64 {
        self.days_to_seconds(self.days_in_year()) as i64 * years
    }

    /// Given a month, calculates the seconds that pass between
    /// the beginning of the year and the end of that month.
    ///
    /// Note that since months are irregular (have different lengths)
    /// this is always relative to the start of the year.
    ///
    /// Example:
    /// calendar.months_to_seconds(3) = January + February + March
    ///
    /// todo(arlyon) return result if month is out of range
    pub fn months_to_seconds(&self, month: u32) -> u32 {
        self.days_to_seconds(
            self.months
                .iter()
                .take(month as usize)
                .map(|m| m.days)
                .sum::<u32>(),
        )
    }

    /// Gets the number of seconds corresponding to a number of days.
    pub fn days_to_seconds(&self, days: u32) -> u32 {
        days * self.hours_to_seconds(self.hours_in_day)
    }

    /// Gets the number of seconds corresponding to a number of hours.
    pub fn hours_to_seconds(&self, hours: u32) -> u32 {
        hours * self.minutes_to_seconds(self.minutes_in_hour)
    }

    /// Gets the number of seconds corresponding to a number of minutes.
    pub fn minutes_to_seconds(&self, minutes: u32) -> u32 {
        minutes * self.seconds_in_minute
    }

    /// Extracts the year component from a time.
    /// This does not respect the era. In that
    /// case please use `Calendar::year`.
    ///
    /// The year component is 0-indexed.
    pub fn years(&self, seconds: i64) -> i64 {
        let seconds_in_year = self.days_to_seconds(self.days_in_year()) as i64;
        seconds / seconds_in_year
    }

    /// Gets the list of months in this calendar.
    pub fn months(&self) -> &[Month] {
        self.months.as_slice()
    }

    /// Extracts the month component from a time.
    ///
    /// The month component is 1-indexed.
    pub fn month(&self, seconds: i64) -> u32 {
        if seconds.is_negative() {
            todo!()
        } else {
            let seconds = (seconds % self.years_to_seconds(1)) as u32;

            let mut seconds_so_far = 0;
            for (index, month) in self.months.iter().enumerate() {
                seconds_so_far += self.days_to_seconds(month.days);
                if seconds_so_far > seconds {
                    return index as u32 + 1;
                }
            }

            panic!("This shouldn't happen.");
        }
    }

    /// Extracts the days component from a time.
    ///
    /// The day component is 1-indexed.
    pub fn day(&self, seconds: i64) -> u32 {
        (seconds % self.years_to_seconds(1)) as u32
            / (self.seconds_in_minute * self.minutes_in_hour * self.hours_in_day)
            + 1
    }

    /// Extracts the hours component from a time.
    ///
    /// The hour component is 0-indexed.
    pub fn hour(&self, seconds: i64) -> u32 {
        (seconds % self.days_to_seconds(1) as i64) as u32
            / (self.seconds_in_minute * self.minutes_in_hour)
    }

    /// Extracts the minutes component from a time.
    ///
    /// The minutes component is 0-indexed.
    pub fn minute(&self, seconds: i64) -> u32 {
        (seconds as u32 % self.hours_to_seconds(1)) / self.seconds_in_minute
    }

    /// Extracts the seconds component from a time.
    ///
    /// The seconds component is 0-indexed.
    pub fn second(&self, seconds: i64) -> u32 {
        seconds as u32 % self.seconds_in_minute
    }

    /// Gets a slice into the Eras in this calendar.
    pub fn eras(&self) -> &[Era] {
        self.eras.0.as_slice()
    }

    /// Gets an era by year.
    pub fn get_era(&self, year: i64) -> (usize, &Era) {
        for (index, era) in self.eras().iter().enumerate() {
            match era.end_year() {
                Some(x) if year <= x => {
                    return (index, era);
                }
                None => return (index, era),
                _ => continue,
            }
        }

        panic!("This shouldn't happen.");
    }

    /// Gets the day of the week for a given DateTime.
    ///
    /// This requires the era because the number of days
    /// in a year is not alwyas 0 modulo the days in the week.
    ///
    /// This is 1-indexed.
    pub fn week_day(&self, era: usize, seconds: i64) -> usize {
        let day = match self.eras.0.get(era) {
            None
            | Some(Era {
                start_year: None,
                end_year: None,
                ..
            }) => {
                let day =
                    (seconds / self.days_to_seconds(1) as i64) as usize % self.week_days.0.len();
                // in this case, there is no start or end date,
                // and we should consider the year from 0
                day
            }
            Some(Era {
                start_year: None,
                end_year: Some(e),
                ..
            }) => {
                // todo(arlyon)
                panic!("{} ending {}", era, e);
            }
            Some(Era {
                start_year: Some(start),
                ..
            }) => {
                let days_passed = *start as u32 * self.days_in_year() + self.day(seconds) - 1;
                days_passed as usize % self.week_days.0.len()
            }
        };

        day + 1
    }

    /// Gets the name of the week for a given time.
    pub fn week_day_name(&self, era: usize, seconds: i64) -> &str {
        &self.week_days.0[self.week_day(era, seconds) - 1]
    }

    /// Gets the day of the month for a given DateTime.
    ///
    /// The month-day component is 1-indexed.
    pub fn month_day(&self, seconds: i64) -> u32 {
        if seconds.is_negative() {
            todo!("{}", seconds)
        } else {
            // find the month we are in
            let month = self.month(seconds);
            let months_passed = month - 1;
            let seconds_so_far = self.months_to_seconds(months_passed) as i64;
            self.day(seconds - seconds_so_far)
        }
    }

    /// Gets the name of the month for a given DateTime.
    pub fn month_name(&self, seconds: i64) -> &str {
        &self.months[self.month(seconds) as usize - 1].name
    }

    /// Get the time component (ie. modulo days).
    ///
    /// This is nonnegative.
    pub fn time(&self, seconds: i64) -> u32 {
        seconds.rem_euclid(self.days_to_seconds(1) as i64) as u32
    }

    /// Gets the year at an offset to a given era.
    ///
    /// The year component is 1-indexed.
    pub fn year(&self, era: usize, seconds: i64) -> i64 {
        let year = match self.eras.0.get(era) {
            None
            | Some(Era {
                start_year: None,
                end_year: None,
                ..
            }) => {
                // in this case, there is no start or end date,
                // and we should consider the year from 0
                self.years(seconds)
            }
            Some(Era {
                start_year: None,
                end_year: Some(_),
                ..
            }) => {
                if seconds.is_positive() {
                    self.year(era + 1, seconds)
                } else {
                    self.years(seconds)
                }
                // this is 'before time', and dates should count backwards
            }
            Some(Era {
                start_year: Some(start),
                end_year: end,
                ..
            }) => {
                let year = start + self.years(seconds);
                if let Some(end) = end {
                    if year > *end {
                        todo!()
                    }
                };
                year
            }
        };

        year + 1
    }

    /// Parses human times such as 1d8h43m into datetime objects
    /// according to the rules of the calendar.
    ///
    /// 1101-02-12 - some specific date
    /// 8am, 2pm - some specific time
    /// 1y32mo6d3s - relative from some time
    /// long rest - exactly 8 hours
    /// short rest - exactly 4 hours
    pub fn parse<'a>(
        &'a self,
        date_string: &str,
        relative_to: Option<DateTime>,
    ) -> Result<DateTime, DateTimeParseError> {
        if let Some(captures) = DATE.captures(date_string) {
            return DateTime::from_date(
                self,
                captures
                    .name("year")
                    .expect("This is in the regex.")
                    .as_str()
                    .parse()
                    .expect("This is a valid i64"),
                captures
                    .name("month")
                    .expect("This is in the regex.")
                    .as_str()
                    .parse()
                    .expect("This is a valid i64"),
                captures
                    .name("day")
                    .expect("This is in the regex.")
                    .as_str()
                    .parse()
                    .expect("This is a valid i64"),
            )
            .map_err(Into::into);
        }

        let mut relative = relative_to.ok_or(DateTimeParseError::NoRelativeReferencePoint)?;

        if date_string.eq("long rest") {
            return Ok(relative.with_calendar(self).add_hours(8));
        }

        if date_string.eq("short rest") {
            return Ok(relative.with_calendar(self).add_hours(4));
        }

        if date_string.eq("midday") {
            return relative
                .with_calendar(self)
                .wait_until(WaitTarget::Midday)
                .map_err(Into::into);
        }

        if date_string.eq("midnight") {
            return relative
                .with_calendar(self)
                .wait_until(WaitTarget::Midnight)
                .map_err(Into::into);
        }

        if RELATIVE.is_match(date_string) {
            for captures in RELATIVE.captures_iter(date_string) {
                let value: u32 = captures
                    .name("value")
                    .expect("This is in the regex.")
                    .as_str()
                    .parse()
                    .expect("This is a valid u32");

                let suffix = captures
                    .name("suffix")
                    .expect("This is in the regex.")
                    .as_str();

                relative = match suffix {
                    "y" => relative.with_calendar(self).add_years(value),
                    "mo" => relative.with_calendar(self).add_months(value),
                    "w" => relative.with_calendar(self).add_weeks(value),
                    "d" => relative.with_calendar(self).add_days(value),
                    "h" => relative.with_calendar(self).add_hours(value),
                    "m" => relative.with_calendar(self).add_minutes(value),
                    "s" => relative.with_calendar(self).add_seconds(value),
                    _ => relative,
                };
            }
            return Ok(relative);
        }

        if let Some(captures) = TIME.captures(date_string) {
            let hour = captures
                .name("value")
                .expect("This is in the regex.")
                .as_str()
                .parse()
                .expect("This is a valid u32");
            let format = match captures
                .name("suffix")
                .expect("This is in the regex.")
                .as_str()
            {
                "am" => TimeFormat::AM,
                "pm" => TimeFormat::PM,
                _ => panic!("This shouldn't happen."),
            };

            return relative
                .with_calendar(self)
                .wait_until(WaitTarget::Time(Time {
                    hour,
                    format,
                    ..Default::default()
                }))
                .map_err(Into::into);
        }

        Err(DateTimeParseError::InvalidFormat)
    }
}

impl Default for Calendar {
    fn default() -> Self {
        Self::new(
            vec![
                Month {
                    name: "January".to_string(),
                    days: 30,
                },
                Month {
                    name: "February".to_string(),
                    days: 31,
                },
                Month {
                    name: "March".to_string(),
                    days: 30,
                },
                Month {
                    name: "April".to_string(),
                    days: 31,
                },
                Month {
                    name: "May".to_string(),
                    days: 30,
                },
                Month {
                    name: "June".to_string(),
                    days: 31,
                },
                Month {
                    name: "July".to_string(),
                    days: 30,
                },
                Month {
                    name: "August".to_string(),
                    days: 30,
                },
                Month {
                    name: "September".to_string(),
                    days: 31,
                },
                Month {
                    name: "October".to_string(),
                    days: 30,
                },
                Month {
                    name: "November".to_string(),
                    days: 31,
                },
                Month {
                    name: "December".to_string(),
                    days: 30,
                },
            ],
            Week(vec![
                "Monday".to_string(),
                "Tuesday".to_string(),
                "Wednesday".to_string(),
                "Thursday".to_string(),
                "Friday".to_string(),
                "Saturday".to_string(),
                "Sunday".to_string(),
            ]),
            Eras(vec![
                Era::new("Before Common Era".to_string(), None, Some(-1)),
                Era::new("Common Era".to_string(), Some(0), None),
            ]),
        )
    }
}

#[derive(Error, Debug, Copy, Clone)]
pub enum DateTimeParseError {
    #[error("invalid date: {0}")]
    InvalidDate(DateTimeError),
    #[error("invalid wait: {0}")]
    InvalidWait(WaitError),
    #[error("invalid format")]
    InvalidFormat,
    #[error("relative time given with no reference point")]
    NoRelativeReferencePoint,
}

impl From<WaitError> for DateTimeParseError {
    fn from(e: WaitError) -> Self {
        Self::InvalidWait(e)
    }
}

impl From<DateTimeError> for DateTimeParseError {
    fn from(e: DateTimeError) -> Self {
        Self::InvalidDate(e)
    }
}
