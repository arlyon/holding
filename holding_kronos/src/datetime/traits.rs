//! Useful traits for viewing / modifying dates and times.

use super::TimeOfDay;

/// Allows modifying time-like objects.
pub trait ModifyTime {
    /// The resulting type.
    type Item;

    /// Add a number of hours to a time object.
    ///
    /// Returns the resulting object and a remainder
    /// for the number of days.
    fn add_hours(&self, hours: u32) -> (Self::Item, u32);

    /// Add a number of minutes to a time object.
    ///
    /// Returns the resulting object and a remainder
    /// for the number of days.
    fn add_minutes(&self, minutes: u32) -> (Self::Item, u32);

    /// Add a number of seconds to a time object.
    ///
    /// Returns the resulting object and a remainder
    /// for the number of days.
    fn add_seconds(&self, seconds: u32) -> (Self::Item, u32);
}

/// Allows modifying date-like objects.
pub trait ModifyDate {
    /// The resulting type.
    type Item;

    /// Add a number of years to a date object.
    fn add_years(self, years: u32) -> Self::Item;

    /// Add a number of months to a date object.
    fn add_months(self, months: u32) -> Self::Item;

    /// Add a number of weeks to a date object.
    fn add_weeks(self, weeks: u32) -> Self::Item;

    /// Add a number of days to a date object.
    fn add_days(self, days: u32) -> Self::Item;
}

/// The `ModifyDateTime` trait provides a number of
/// methods to modify time in datetime-like objects
/// while respecting rollover between time and date.
///
/// ```rust
/// # use std::error::Error;
/// #
/// # use holding_kronos::datetime::{
/// #     traits::{ModifyDate, ModifyDateTime, ShowDate, ShowTime},
/// #     DateTime,
/// # };
/// # use holding_kronos::calendar::Calendar;
/// #
/// # fn main() -> Result<(), Box<dyn Error>> {
///       let cal = Calendar::default();
///       let x = DateTime::from_date(2020, 09, 09, &cal)?;
///       let x = x.add_hours(13);
///       let x = x.add_hours(15);
/// #     assert_eq!(x.hour(), 4);
/// #     assert_eq!(x.day(), 10);
/// #     Ok(())
/// # }
/// ```
pub trait ModifyDateTime {
    /// The item that the modification outputs.
    type Item;

    /// Get a new `Self::Item` with a number of hours added.
    fn add_hours(&self, hours: u32) -> Self::Item;

    /// Get a new `Self::Item` with a number of minutes added.
    fn add_minutes(&self, minutes: u32) -> Self::Item;

    /// Get a new `Self::Item` with a number of seconds added.
    fn add_seconds(&self, seconds: u32) -> Self::Item;
}

/// The `ShowTime` trait provides a number of
/// methods to get common information about times.
pub trait ShowTime {
    /// Gets the hour component.
    fn hour(&self) -> u32;

    /// Gets the minutes component.
    fn minute(&self) -> u32;

    /// Gets the seconds component.
    fn second(&self) -> u32;

    /// The total number of seconds this time represents.
    fn seconds(&self) -> u32;

    /// Get the `TimeOfDay` that corresponds to this time.
    fn time_of_day(self) -> TimeOfDay;
}

/// The `ShowDate` trait provides a number of
/// methods to get common information about dates.
pub trait ShowDate {
    /// Gets the year that the datetime is
    /// refering to in this calendar.
    ///
    /// The year component is 1-indexed.
    fn year(&self) -> i64;

    /// Extracts the month component from a time.
    ///
    /// The month component is 1-indexed.
    fn month(&self) -> u32;

    /// Gets the name of the month for a given date.
    fn month_name(&self) -> &str;

    /// Extracts the week component from a date.
    ///
    /// The week component is 1-indexed.
    fn week(&self) -> u32;

    /// Gets the day of the week for a given date.
    ///
    /// This requires the era because the number of days
    /// in a year is not alwyas 0 modulo the days in the week.
    ///
    /// This is 1-indexed.
    fn week_day(&self) -> u32;

    /// Gets the name of the week for a given time.
    fn week_day_name(&self) -> &str;

    /// Extracts the day component from a date.
    ///
    /// The day component is 1-indexed.
    fn day(&self) -> u32;

    /// Gets the number of days that have passed
    /// in the current year.
    ///
    /// Example: 03-01-2020 -> 2 days have passed
    ///
    /// The day component is 1-indexed.
    fn days(&self) -> u32;
}
