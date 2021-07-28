//! holding_solar: Simple celestial simulation.

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
    clippy::panic,
    clippy::as_conversions
)]

mod orbit;
mod planet;

pub use orbit::{Orbit, Phase};
pub use planet::{CelestialBody, PlanetId, PlanetStore};
