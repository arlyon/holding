use std::convert::TryFrom;

use num_enum::{IntoPrimitive, TryFromPrimitive};
use strum_macros::Display;

#[derive(IntoPrimitive, Clone, Debug, TryFromPrimitive, Eq, PartialEq, Copy, Display)]
#[repr(u8)]
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
    pub fn from_time(curr_hour: u32, max_hour: u32) -> Self {
        Self::try_from(((curr_hour as f64 / max_hour as f64) * 8.0).floor() as u8).unwrap()
    }

    pub fn is_day(&self) -> bool {
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

#[cfg(test)]
mod test {
    use crate::datetime::time_of_day::TimeOfDay;

    #[test]
    pub fn test_get_time() {
        assert_eq!(TimeOfDay::from_time(0, 8), TimeOfDay::LateNight);
        assert_eq!(TimeOfDay::from_time(1, 8), TimeOfDay::Dawn);
        assert_eq!(TimeOfDay::from_time(2, 8), TimeOfDay::Sunrise);
        assert_eq!(TimeOfDay::from_time(3, 8), TimeOfDay::Morning);
        assert_eq!(TimeOfDay::from_time(4, 8), TimeOfDay::Afternoon);
        assert_eq!(TimeOfDay::from_time(5, 8), TimeOfDay::Sunset);
        assert_eq!(TimeOfDay::from_time(6, 8), TimeOfDay::Dusk);
        assert_eq!(TimeOfDay::from_time(7, 8), TimeOfDay::Night);
    }
}
