//! holding_kronos: The fantasy datetime crate.
//!
//! As it is built for macro-level management, this library
//! uses seconds as the fundamental unit of time.
//!
//! It also assumes a basic gregorian calendar with a
//! year split up into months, and days organised into
//! weeks.

#![deny(
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unreachable_code,
    missing_copy_implementations,
    unused_import_braces,
    unused_qualifications
)]
#![warn(
    clippy::todo,
    clippy::unwrap_used,
    clippy::unused_self,
    clippy::unimplemented,
    clippy::trivially_copy_pass_by_ref,
    clippy::panic
)]

mod calendar;
mod datetime;

pub use calendar::{Calendar, Era};
pub use datetime::{CalendarDateTime, DateTime};
