use std::fmt::Display;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    calendar::{
        traits::{ConvertTime, DayCycle},
        Calendar,
    },
    datetime::{
        traits::{ModifyTime, ShowTime},
        TimeOfDay,
    },
    util::div_rem,
};

/// A raw time object (useful for serialization).
#[derive(Debug, Clone, Copy, Ord, PartialOrd, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RawTime {
    /// The hour component.
    pub hour: u32,
    /// The minute component.
    pub minute: u32,
    /// The second compoent.
    pub second: u32,
}

impl RawTime {
    /// Convert this `RawTime` into a `Time` object.
    pub fn into_time(self, cal: &Calendar) -> Time {
        Time {
            time: self,
            calendar: cal,
        }
    }
}

/// Represents times.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Time<'a> {
    /// The raw time object.
    pub time: RawTime,
    /// The calendar this `Time` uses.
    pub calendar: &'a Calendar,
}

impl<'a> ModifyTime for Time<'a> {
    type Item = Time<'a>;

    fn add_seconds(&self, seconds: u32) -> (Self::Item, u32) {
        let (days, seconds) = div_rem(self.seconds() + seconds, self.calendar.days_to_seconds(1));
        (Time::from_seconds(seconds, self.calendar), days)
    }

    fn add_minutes(&self, minutes: u32) -> (Self::Item, u32) {
        self.add_seconds(minutes * self.calendar.seconds_in_minute())
    }

    fn add_hours(&self, hours: u32) -> (Self::Item, u32) {
        self.add_minutes(hours * self.calendar.minutes_in_hour())
    }
}

impl<'a> ShowTime for Time<'a> {
    fn hour(&self) -> u32 {
        self.time.hour
    }

    fn minute(&self) -> u32 {
        self.time.minute
    }

    fn second(&self) -> u32 {
        self.time.second
    }

    fn seconds(&self) -> u32 {
        self.calendar.hours_to_seconds(self.time.hour)
            + self.calendar.minutes_to_seconds(self.time.minute)
            + self.time.second
    }

    /// Get the `TimeOfDay`
    fn time_of_day(self) -> TimeOfDay {
        TimeOfDay::from_time(self.time.hour, self.calendar.hours_in_day())
            .expect("If this fails it is programmer error")
    }
}

impl<'a> PartialOrd for Time<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.calendar == other.calendar {
            self.time.partial_cmp(&other.time)
        } else {
            None
        }
    }
}

impl Display for RawTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:0>2}:{:0>2}:{:0>2}",
            self.hour, self.minute, self.second
        )
    }
}

impl Display for Time<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.time.fmt(f)
    }
}

impl<'a> Time<'a> {
    /// Create a new `Time` from an hour-minute-second tuple.
    ///
    /// This will return an `InvalidTimeError` if any of the supplied
    /// components are outside the bounds of the provided Calendar.
    ///
    /// # Arguments
    ///
    /// * `hour` - The number of hours relative to midnight.
    /// * `minute` - The number of minutes relative to midnight.
    /// * `second` - The number of seconds relative to midnight.
    /// * `calendar` - The calendar this `Time` is relative to.
    /// * `format` - The format of the hms. This is for convenience.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use holding_kronos::datetime::{
    /// #     traits::{ModifyTime, ShowTime},
    /// #     Time, TimeFormat,
    /// # };
    /// # use holding_kronos::calendar::Calendar;
    /// #
    /// # fn main() -> Result<(), Box<dyn Error>> {
    ///       let cal = Calendar::default();
    ///       let time_1 = Time::from_hms(0, 0, 0, &cal, TimeFormat::Exact)?;
    ///       let time_2 = Time::from_hms(1, 0, 0, &cal, TimeFormat::Exact)?;
    ///
    ///       assert_ne!(time_1, time_2);
    ///
    ///       let (time_3, _) = time_1.add_hours(1);
    ///
    ///       assert_eq!(time_2, time_3);
    /// #     Ok(())
    /// # }
    /// ```
    ///
    /// todo(arlyon): Out of bounds errors.
    pub fn from_hms(
        hour: u32,
        minute: u32,
        second: u32,
        calendar: &'a Calendar,
        format: TimeFormat,
    ) -> Result<Self, InvalidTimeError> {
        let offset = match format {
            TimeFormat::Exact => 0,
            TimeFormat::AM => 0,
            TimeFormat::PM => calendar.hours_in_day() / 2,
        };

        let hour = hour + offset;

        if hour >= calendar.hours_in_day() {
            Err(InvalidTimeError::HourOutOfBounds(hour))
        } else if minute >= calendar.minutes_in_hour() {
            Err(InvalidTimeError::MinuteOutOfBounds(minute))
        } else if second >= calendar.seconds_in_minute() {
            Err(InvalidTimeError::SecondOutOfBounds(second))
        } else {
            Ok(Self {
                time: RawTime {
                    hour,
                    minute,
                    second,
                },
                calendar,
            })
        }
    }

    /// Create a new `Time` representing the number of seconds.
    ///
    /// Note that this will perform a modulo operator if the seconds
    /// provided exceed that of a day in the provided calendar.
    ///
    /// # Arguments
    ///
    /// * `seconds` - The number of seconds relative to midnight.
    /// * `calendar` - The calendar this `Time` is relative to.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use holding_kronos::datetime::{
    /// #     traits::{ModifyTime, ShowTime},
    /// #     Time,
    /// # };
    /// # use holding_kronos::calendar::Calendar;
    /// #
    /// # fn main() -> Result<(), Box<dyn Error>> {
    ///       let cal = Calendar::default();
    ///       let time_1 = Time::from_seconds(0, &cal);
    ///       let time_2 = Time::from_seconds(86400, &cal);
    ///
    ///       assert_eq!(time_1, time_2);
    ///
    ///       let (time_3, days) = time_1.add_hours(24);
    ///
    ///       assert_eq!(time_1, time_3);
    ///       assert_eq!(days, 1);
    /// #     Ok(())
    /// # }
    /// ```
    pub fn from_seconds(seconds: u32, calendar: &'a Calendar) -> Self {
        let (hour, seconds) = div_rem(seconds, calendar.seconds_in_hour());
        let (minute, second) = div_rem(seconds, calendar.seconds_in_minute());

        Self::from_hms(
            hour % calendar.hours_in_day(),
            minute,
            second,
            calendar,
            TimeFormat::Exact,
        )
        .expect("If this fails it is programmer error")
    }
}

impl Into<RawTime> for Time<'_> {
    fn into(self) -> RawTime {
        self.time
    }
}

/// Allow to specify a time relative to another.
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

/// Possible invalid time states.
#[derive(Error, Debug, Copy, Clone)]
#[allow(missing_docs)]
pub enum InvalidTimeError {
    #[error("hour {0} is out of bounds")]
    HourOutOfBounds(u32),
    #[error("minute {0} is out of bounds")]
    MinuteOutOfBounds(u32),
    #[error("second {0} is out of bounds")]
    SecondOutOfBounds(u32),
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    use crate::{calendar::Calendar, datetime::traits::ModifyTime};

    use super::{Time, TimeFormat};

    #[test_case(1, 1, 0, 0, 0)]
    pub fn add_hour(add: u32, hour: u32, minute: u32, second: u32, days_expected: u32) {
        let cal = Calendar::default();
        let (time, days) = Time::from_seconds(0, &cal).add_hours(add);

        assert_eq!(
            time.time,
            Time::from_hms(hour, minute, second, &cal, TimeFormat::Exact)
                .unwrap()
                .time
        );
        assert_eq!(days_expected, days);
    }
}
