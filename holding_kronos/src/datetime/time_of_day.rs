use std::convert::TryFrom;

use num_enum::{IntoPrimitive, TryFromPrimitive};
use strum::Display;

use crate::{
    calendar::traits::DayCycle,
    datetime::{traits::ShowTime, DateTime},
};

#[derive(IntoPrimitive, TryFromPrimitive, Clone, Debug, Eq, PartialEq, Copy, Display)]
#[repr(u8)]
#[allow(missing_docs)]
pub enum TimeOfDay {
    #[strum(serialize = "late in the night")]
    LateNight,
    #[strum(serialize = "at dawn")]
    Dawn,
    #[strum(serialize = "just after sunrise")]
    Sunrise,
    #[strum(serialize = "in the morning")]
    Morning,
    #[strum(serialize = "in the afternoon")]
    Afternoon,
    #[strum(serialize = "just before sunset")]
    Sunset,
    #[strum(serialize = "in the evening")]
    Dusk,
    #[strum(serialize = "at night")]
    Night,
}

impl TimeOfDay {
    /// Gets the time of day for a given hour.
    ///
    /// Returns Some(TimeOfDay) when curr_hour < max_hour, else None.
    pub fn from_time(curr_hour: u32, max_hour: u32) -> Option<Self> {
        let index = (f64::from(curr_hour) * 8.0 / f64::from(max_hour)) as u8;
        Self::try_from(index).ok()
    }

    /// Checks if a given `TimeOfDay` is during day or night.
    pub fn is_day(self) -> bool {
        match self {
            TimeOfDay::LateNight => false,
            TimeOfDay::Dawn => false,
            TimeOfDay::Sunrise => true,
            TimeOfDay::Morning => true,
            TimeOfDay::Afternoon => true,
            TimeOfDay::Sunset => true,
            TimeOfDay::Dusk => false,
            TimeOfDay::Night => false,
        }
    }
}

impl<'a> From<DateTime<'a>> for TimeOfDay {
    fn from(dt: DateTime<'a>) -> Self {
        Self::from_time(dt.hour(), dt.calendar().hours_in_day())
            .expect("If this is not in range it is programmer error")
    }
}

#[cfg(test)]
mod test {
    use crate::datetime::time_of_day::TimeOfDay;
    use test_case::test_case;

    #[test_case(0, 8, TimeOfDay::LateNight)]
    #[test_case(1, 8, TimeOfDay::Dawn)]
    #[test_case(2, 8, TimeOfDay::Sunrise)]
    #[test_case(3, 8, TimeOfDay::Morning)]
    #[test_case(4, 8, TimeOfDay::Afternoon)]
    #[test_case(5, 8, TimeOfDay::Sunset)]
    #[test_case(6, 8, TimeOfDay::Dusk)]
    #[test_case(7, 8, TimeOfDay::Night)]
    pub fn get_time(curr_hour: u32, max_hour: u32, time_of_day: TimeOfDay) {
        assert_eq!(TimeOfDay::from_time(curr_hour, max_hour), Some(time_of_day));
    }
}
