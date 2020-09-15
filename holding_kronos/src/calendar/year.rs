use std::slice::Iter;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::traits::YearCycle;

/// Represents a year in the calendar.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Year(Vec<Month>);

impl Year {
    /// Iterate over the months in the year.
    pub fn iter(&self) -> Iter<'_, Month> {
        self.0.iter()
    }

    /// Get a slice from the months in order.
    pub fn as_slice(&self) -> &[Month] {
        &self.0
    }
}

impl YearCycle for Year {
    fn days_in_year(&self) -> u32 {
        self.0.iter().map(|month| month.days).sum::<u32>()
    }

    fn months_in_year(&self) -> u32 {
        self.0.len() as u32
    }
}

/// A simple default that sortof matches earth (but with exactly 365 days)
impl Default for Year {
    fn default() -> Self {
        Self(vec![
            Month {
                name: "January".to_string(),
                days: 31,
            },
            Month {
                name: "February".to_string(),
                days: 28,
            },
            Month {
                name: "March".to_string(),
                days: 31,
            },
            Month {
                name: "April".to_string(),
                days: 30,
            },
            Month {
                name: "May".to_string(),
                days: 31,
            },
            Month {
                name: "June".to_string(),
                days: 30,
            },
            Month {
                name: "July".to_string(),
                days: 31,
            },
            Month {
                name: "August".to_string(),
                days: 31,
            },
            Month {
                name: "September".to_string(),
                days: 30,
            },
            Month {
                name: "October".to_string(),
                days: 31,
            },
            Month {
                name: "November".to_string(),
                days: 30,
            },
            Month {
                name: "December".to_string(),
                days: 31,
            },
        ])
    }
}

/// A month with an arbitrary number of days.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Month {
    /// The name of the month.
    pub name: String,

    /// The days in the month.
    pub days: u32,
}
