//! calendar: A general-purpose flexible fantasy calendar.
//!
//! For simplicity, it does not support leap-anything.

mod calendar;
mod day;
mod era;
mod test;
pub mod traits;
mod week;
mod year;

pub use calendar::Calendar;
pub use day::Day;
pub use era::{Era, Eras};
pub use week::Week;
pub use year::{Month, Year};
