//! era: Manage distinct eras of history.

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Eras(pub Vec<Era>);

/// An era is a contiguous block of time in a calendar from which time can be referenced.
/// Multiple eras can exist (and overlap) simultaneously, making it a very flexible tool
/// for spacing out chunks of time such as reigning monarchs, or warring gods.
///
/// An era that is unbounded on the left (ie. has no start) is considered to have always
/// existed, while an era that is unbounded on the right right has not ended.
#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Era {
    /// The name of the Era.
    pub name: String,

    /// When the era started. None implies it has always existed.
    pub start_year: Option<i64>,

    /// When the era ended. None implies it hasn't ended.
    pub end_year: Option<i64>,
}

impl Era {
    /// Creates a new `Era`.
    pub fn new(name: String, start_year: Option<i64>, end_year: Option<i64>) -> Self {
        Self {
            name,
            start_year,
            end_year,
        }
    }

    /// Gets the name of the `Era`.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Gets the start year of the Era.
    pub fn start_year(&self) -> Option<i64> {
        self.start_year
    }

    /// Gets the end year of the Era.
    pub fn end_year(&self) -> Option<i64> {
        self.end_year
    }
}
