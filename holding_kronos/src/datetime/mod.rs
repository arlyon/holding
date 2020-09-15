//! calendar: General-purpose flexible dates and times in arbitrary calendars.
//!
//! For simplicity, it does not support leap-anything.

use std::fmt::Debug;

use thiserror::Error;

mod date;
mod datetime;
mod time;
mod time_of_day;
pub mod traits;

pub use date::{Date, InvalidDateError, RawDate};
pub use datetime::{DateTime, InvalidDateTimeError, RawDateTime};
pub use time::{InvalidTimeError, RawTime, Time, TimeFormat};
pub use time_of_day::TimeOfDay;

/// Allows you to wait until a given time.
#[derive(Copy, Clone)]
pub enum WaitTarget<'a> {
    /// Wait until the next occurrence of `Time`.
    Time(Time<'a>),
    /// Convenience to wait until midnight regardless of calendar.
    Midnight,
    /// Convenience to wait until midday regardless of calendar.
    Midday,
}

/// Possible error states that can occur when trying to wait.
#[derive(Error, Debug, Copy, Clone)]
#[allow(missing_docs)]
pub enum InvalidWaitError {
    #[error("the time is out of bounds")]
    InvalidTime(#[from] InvalidTimeError),

    #[error("the date is out of bounds")]
    InvalidDate(#[from] InvalidDateError),

    #[error("the wait target {0} is before current {1}")]
    BackwardsWait(RawTime, RawTime),
}
