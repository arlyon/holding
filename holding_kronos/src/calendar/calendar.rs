#[cfg(feature = "parse")]
use lazy_static::lazy_static;
#[cfg(feature = "parse")]
use regex::Regex;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "parse")]
use thiserror::Error;

use crate::{
    calendar::{
        traits::{ConvertDate, ConvertTime, DayCycle, YearCycle},
        Day, Month, Week, Year,
    },
    datetime::traits::ShowDate,
    datetime::traits::ShowTime,
    datetime::InvalidDateTimeError,
    datetime::{
        traits::{ModifyDate, ModifyDateTime},
        DateTime, InvalidDateError, InvalidTimeError, InvalidWaitError, Time, TimeFormat,
        WaitTarget,
    },
};

use super::{traits::WeekCycle, week::WeekDay};

#[cfg(feature = "parse")]
lazy_static! {
    static ref DATE: Regex = Regex::new(r"^(?P<y>\d+)-(?P<m>\d+)-(?P<d>\d+)$").expect("compiles");
    static ref REL: Regex = Regex::new(r"(?P<val>\d+)(?P<suff>(mo|[ywdhms]))").expect("compiles");
    static ref TIME: Regex = Regex::new(r"^(?P<val>\d+)(?P<suff>(am|pm))$").expect("compiles");
}

/// A calendar provides a frame of reference for the manipulation
/// of `DateTime`. It defines what a day, week, or month is, and
/// since those can change.
#[derive(Clone, Debug, Eq, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Calendar {
    year: Year,
    week: Week,
    day: Day,
}

impl Calendar {
    /// Creates a new Calendar given a number of months, week days, and eras.
    pub fn new(year: Year, week: Week, day: Day) -> Self {
        Self { year, week, day }
    }

    /// Validates a date against this calendar.
    pub fn validate_date<'a, T: ShowDate>(&self, date: &'a T) -> Result<&'a T, InvalidDateError> {
        let day = date.day();
        let month = date.month();

        if month == 0 || month > self.months_in_year() {
            Err(InvalidDateError::MonthOutOfBounds(month))
        } else if day == 0 || day > self.months()[month as usize - 1].days {
            Err(InvalidDateError::DayOutOfBounds(day))
        } else {
            Ok(date)
        }
    }

    /// Validates a time against this calendar.
    pub fn validate_time<'a, T: ShowTime>(&self, time: &'a T) -> Result<&'a T, InvalidTimeError> {
        let hour = time.hour();
        let minute = time.minute();
        let second = time.second();

        if hour >= self.hours_in_day() {
            Err(InvalidTimeError::HourOutOfBounds(hour))
        } else if minute >= self.minutes_in_hour() {
            Err(InvalidTimeError::MinuteOutOfBounds(minute))
        } else if second >= self.seconds_in_minute() {
            Err(InvalidTimeError::SecondOutOfBounds(second))
        } else {
            Ok(time)
        }
    }

    /// Gets the list of months in this calendar.
    pub fn months(&self) -> &[Month] {
        self.year.as_slice()
    }

    /// Gets the list of week days in this calendar.
    pub fn week_days(&self) -> &[WeekDay] {
        self.week.as_slice()
    }

    /// Parses human times such as 1d8h43m into datetime objects
    /// according to the rules of the calendar.
    ///
    /// 1101-02-12 - some specific date
    /// 8am, 2pm - some specific time
    /// 1y32mo6d3s - relative from some time
    /// long rest - exactly 8 hours
    /// short rest - exactly 4 hours
    #[cfg(feature = "parse")]
    pub fn parse<'a, T>(
        &'a self,
        date_string: &str,
        relative_to: T,
    ) -> Result<DateTime<'a>, ParseDateTimeError>
    where
        T: Into<Option<DateTime<'a>>>,
    {
        let relative_to: Option<DateTime> = relative_to.into();
        if let Some(captures) = DATE.captures(date_string) {
            return DateTime::from_date(
                captures
                    .name("y")
                    .expect("This is in the regex")
                    .as_str()
                    .parse()
                    .expect("This is a valid i64"),
                captures
                    .name("m")
                    .expect("This is in the regex")
                    .as_str()
                    .parse()
                    .expect("This is a valid i64"),
                captures
                    .name("d")
                    .expect("This is in the regex")
                    .as_str()
                    .parse()
                    .expect("This is a valid i64"),
                self,
            )
            .map_err(Into::into);
        }

        let mut relative = relative_to.ok_or(ParseDateTimeError::NoRelativeReferencePoint)?;

        if date_string.eq("long rest") {
            return Ok(relative.add_hours(8));
        }

        if date_string.eq("short rest") {
            return Ok(relative.add_hours(4));
        }

        if date_string.eq("midday") {
            return relative.wait_until(WaitTarget::Midday).map_err(Into::into);
        }

        if date_string.eq("midnight") {
            return relative
                .wait_until(WaitTarget::Midnight)
                .map_err(Into::into);
        }

        if REL.is_match(date_string) {
            for captures in REL.captures_iter(date_string) {
                let value: u32 = captures
                    .name("val")
                    .expect("This is in the regex")
                    .as_str()
                    .parse()
                    .expect("This is a valid u32");

                let suffix = captures
                    .name("suff")
                    .expect("This is in the regex")
                    .as_str();

                relative = match suffix {
                    "y" => relative.add_years(value),
                    "mo" => relative.add_months(value),
                    "w" => relative.add_weeks(value),
                    "d" => relative.add_days(value),
                    "h" => relative.add_hours(value),
                    "m" => relative.add_minutes(value),
                    "s" => relative.add_seconds(value),
                    _ => relative,
                };
            }
            return Ok(relative);
        }

        if let Some(captures) = TIME.captures(date_string) {
            let hour = captures
                .name("val")
                .expect("This is in the regex")
                .as_str()
                .parse()
                .expect("This is a valid u32");
            let format = match captures
                .name("suff")
                .expect("This is in the regex")
                .as_str()
            {
                "am" => Some(TimeFormat::AM),
                "pm" => Some(TimeFormat::PM),
                _ => None,
            }
            .expect("The regex is incorrect");

            let time = Time::from_hms(hour, 0, 0, self, format)?;

            return relative
                .wait_until(WaitTarget::Time(time))
                .map_err(Into::into);
        }

        Err(ParseDateTimeError::InvalidFormat)
    }
}

impl YearCycle for Calendar {
    fn days_in_year(&self) -> u32 {
        self.year.days_in_year()
    }

    fn months_in_year(&self) -> u32 {
        self.year.months_in_year()
    }
}

impl WeekCycle for Calendar {
    fn days_in_week(&self) -> u32 {
        self.week.days_in_week()
    }
}

impl DayCycle for Calendar {
    fn hours_in_day(&self) -> u32 {
        self.day.hours_in_day()
    }

    fn minutes_in_hour(&self) -> u32 {
        self.day.minutes_in_hour()
    }

    fn seconds_in_minute(&self) -> u32 {
        self.day.seconds_in_minute()
    }
}

impl ConvertDate for Calendar {
    fn months_to_seconds(&self, month: u32) -> u32 {
        self.days_to_seconds(
            self.year
                .iter()
                .take(month as usize)
                .map(|m| m.days)
                .sum::<u32>(),
        )
    }

    fn years_to_seconds(&self, years: u32) -> u32 {
        self.days_to_seconds(self.days_in_year()) * years
    }

    fn weeks_to_seconds(&self, weeks: u32) -> u32 {
        self.days_to_seconds(weeks * self.days_in_week())
    }

    fn days_to_months(&self, days: u32) -> (u32, u32) {
        let mut rem_days = days;
        let mut months = self.months().iter().cycle().enumerate();
        loop {
            let (index, month) = months.next().expect("This loops forever");
            let month_days = month.days;
            if month_days <= rem_days {
                rem_days -= month_days;
            } else {
                return (index as u32, rem_days);
            }
        }
    }
}

#[cfg(feature = "parse")]
#[derive(Error, Debug, Copy, Clone)]
pub enum ParseDateTimeError {
    #[error("invalid date: {0}")]
    InvalidDate(#[from] InvalidDateError),
    #[error("invalid time: {0}")]
    InvalidTime(#[from] InvalidTimeError),
    #[error("invalid wait: {0}")]
    InvalidWait(#[from] InvalidWaitError),
    #[error("invalid format")]
    InvalidFormat,
    #[error("relative time given with no reference point")]
    NoRelativeReferencePoint,
}

impl From<InvalidDateTimeError> for ParseDateTimeError {
    fn from(val: InvalidDateTimeError) -> Self {
        match val {
            InvalidDateTimeError::InvalidDate(d) => ParseDateTimeError::InvalidDate(d),
            InvalidDateTimeError::InvalidTime(t) => ParseDateTimeError::InvalidTime(t),
        }
    }
}
