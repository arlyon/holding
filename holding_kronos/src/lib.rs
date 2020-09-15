//! holding_kronos: The fantasy datetime crate.
//!
//! As it is built for macro-level management, this library
//! uses seconds as the fundamental unit of time.
//!
//! It also assumes a basic gregorian calendar with a
//! year split up into months, and days organised into
//! weeks.

#![forbid(unsafe_code)]
#![deny(
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
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

pub mod calendar;
pub mod datetime;
mod util;
