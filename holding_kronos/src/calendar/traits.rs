//! Useful traits for representing arbitrary calendars.

/// Allows for inspecting information about a year cycle.
pub trait YearCycle {
    /// Gets the days in a year for this calendar.
    fn days_in_year(&self) -> u32;

    /// Gets the number of months in a year for this calendar.
    fn months_in_year(&self) -> u32;
}

/// Allows for inspecting information about a week cycle.
pub trait WeekCycle {
    /// Gets the days in a week for this calendar.
    fn days_in_week(&self) -> u32;
}

/// Allows for inspecting information about a day cycle.
pub trait DayCycle {
    /// Gets the hours in the day for this calendar.
    fn hours_in_day(&self) -> u32;

    /// Gets the hours in the day for this calendar.
    fn minutes_in_hour(&self) -> u32;

    /// Gets the hours in the day for this calendar.
    fn seconds_in_minute(&self) -> u32;

    /// Gets the total number of seconds in a day,
    fn seconds_in_day(&self) -> u32 {
        self.seconds_in_minute() * self.minutes_in_day()
    }

    /// Gets the total number of seconds in an hour.
    fn seconds_in_hour(&self) -> u32 {
        self.seconds_in_minute() * self.minutes_in_hour()
    }

    /// Gets the total number of minutes in a day.
    fn minutes_in_day(&self) -> u32 {
        self.minutes_in_hour() * self.hours_in_day()
    }
}

impl<T> ConvertTime for T
where
    T: DayCycle,
{
    fn days_to_seconds(&self, days: u32) -> u32 {
        days * self.seconds_in_day()
    }

    fn hours_to_seconds(&self, hours: u32) -> u32 {
        hours * self.seconds_in_hour()
    }

    fn minutes_to_seconds(&self, minutes: u32) -> u32 {
        minutes * self.seconds_in_minute()
    }
}

/// Allows for conversion between various units of time.
///
/// This trait is unstable and may change.
pub trait ConvertTime {
    /// Gets the number of seconds corresponding to a number of days.
    fn days_to_seconds(&self, days: u32) -> u32;
    /// Gets the number of seconds corresponding to a number of hours.
    fn hours_to_seconds(&self, hours: u32) -> u32;
    /// Gets the number of seconds corresponding to a number of minutes.
    fn minutes_to_seconds(&self, minutes: u32) -> u32;
}

/// Allows for version between various units of time.
///
/// This trait is unstable and may change.
pub trait ConvertDate {
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
    fn months_to_seconds(&self, months: u32) -> u32;

    /// Gets the number of seconds corresponding to a number of years.
    fn years_to_seconds(&self, years: u32) -> u32;

    /// Gets the number of seconds corresponsing to a number of years.
    fn weeks_to_seconds(&self, weeks: u32) -> u32;

    /// Gets the number of months from a number of days with remainder.
    fn days_to_months(&self, days: u32) -> (u32, u32);
}
