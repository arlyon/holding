use std::slice::Iter;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::traits::WeekCycle;

/// Represents a week in the calendar.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Week(Vec<WeekDay>);

/// Represents a week day in the calendar.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WeekDay {
    /// The name of this day.
    pub name: String,
}

impl Week {
    /// Iterate over the days in the week.
    pub fn iter(&self) -> Iter<'_, WeekDay> {
        self.0.iter()
    }

    /// Gets a slice of the week days in this week.
    pub fn as_slice(&self) -> &[WeekDay] {
        &self.0
    }
}

impl WeekCycle for Week {
    fn days_in_week(&self) -> u32 {
        self.0.len() as u32
    }
}

impl Default for Week {
    fn default() -> Self {
        Self(vec![
            WeekDay {
                name: "Monday".to_string(),
            },
            WeekDay {
                name: "Tuesday".to_string(),
            },
            WeekDay {
                name: "Wednesday".to_string(),
            },
            WeekDay {
                name: "Thursday".to_string(),
            },
            WeekDay {
                name: "Friday".to_string(),
            },
            WeekDay {
                name: "Saturday".to_string(),
            },
            WeekDay {
                name: "Sunday".to_string(),
            },
        ])
    }
}
