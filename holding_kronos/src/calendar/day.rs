#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::traits::DayCycle;

/// Represents a day.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Day {
    seconds_in_minute: u32,
    minutes_in_hour: u32,
    hours_in_day: u32,
}

impl Day {
    /// Creates a new `Day`.
    pub fn new(seconds_in_minute: u32, minutes_in_hour: u32, hours_in_day: u32) -> Self {
        Self {
            seconds_in_minute,
            minutes_in_hour,
            hours_in_day,
        }
    }
}

impl DayCycle for Day {
    fn hours_in_day(&self) -> u32 {
        self.hours_in_day
    }

    fn minutes_in_hour(&self) -> u32 {
        self.minutes_in_hour
    }

    fn seconds_in_minute(&self) -> u32 {
        self.seconds_in_minute
    }
}

impl Default for Day {
    fn default() -> Self {
        Self::new(60, 60, 24)
    }
}
