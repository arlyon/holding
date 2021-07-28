//! orbits: Define planetary systems with complex orbits.
//!
//! This model uses a simple tree hierarchy and only considers
//! orbits in relation to their parents, ignoring siblings.

use std::convert::TryFrom;
use std::f64::consts::PI;

use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};
use strum_macros::Display;
use thiserror::Error;

use holding_kronos::calendar::{traits::ConvertDate, Calendar};
use holding_kronos::datetime::DateTime;

use crate::{CelestialBody, PlanetId, PlanetStore};

/// Describes the orbit of a given planet.
///
/// For the sake of simplicity, orbits are rounded
/// to the nearest day relative to the parent object.
/// This makes calendar calculation much much simpler,
/// as you do not need to manage leap seconds / days.
#[derive(Clone, Deserialize, Serialize, Debug, Copy)]
pub struct Orbit {
    /// The id of the parent planet.
    pub parent: PlanetId,

    /// The body that is orbiting.
    pub body: PlanetId,

    /// The starting offset for the orbit in days.
    pub shift: u32,

    /// The eccentricity of the orbit, or how elliptic it is.
    pub eccentricity: f64,

    /// The amount of time a single orbit takes in seconds.
    ///
    /// For simplicity, valid planets _must_ have a period
    /// that is a multiple of the number of seconds in a day.
    pub period: u32,
}

impl Orbit {
    /// Create a new orbit with a given perion.
    pub fn from_period(target: &CelestialBody, parent: PlanetId, period: u32, shift: u32) -> Self {
        Orbit {
            parent,
            body: target.id,
            period: period * target.rotational_period,
            shift: shift * target.rotational_period % period,
            eccentricity: 0.0,
        }
    }

    /// Creates a new orbit at a given semimajor axis.
    /// Note that for now, since the period is a fixed number
    /// of days relative to the parent, this will be rounded.
    /// This is for simpler interop with the calendar. Nobody
    /// wants to RP leap seconds!
    pub fn from_radius(
        target: &CelestialBody,
        parent: PlanetId,
        semimajor_axis: f64,
        shift: u32,
    ) -> Self {
        let period = semimajor_axis.powf(3.0).sqrt() as u32;
        Orbit::from_period(target, parent, period, shift)
    }

    /// Get the phase of the orbit.
    ///
    /// Only valid if the body being orbited is in turn
    /// orbiting something else (that gives off light).
    pub fn get_phase(&self, lookup: &dyn PlanetStore, date_time: DateTime) -> Option<Phase> {
        // luminous bodies don't have a visible phase.
        if lookup.get_planet(self.body)?.is_luminous() {
            return None;
        }

        let parent = lookup.get_planet(self.parent)?;
        let parent_orbit = parent.orbit.as_ref()?;
        let sun = lookup.get_planet(parent_orbit.parent)?;

        // unilluminated bodies don't have a visible phase.
        if !sun.is_luminous() {
            return None;
        }

        // angle of moon relative to parent
        let theta_moon = self.get_orbit_radians(date_time.seconds_modulo(self.period));

        // angle of parent relative to sun
        let theta_parent =
            parent_orbit.get_orbit_radians(date_time.seconds_modulo(parent_orbit.period));

        let mut theta = theta_moon - theta_parent;
        if theta.is_sign_negative() {
            theta += 2.0 * PI
        };

        let index = (theta * 4.0 / PI) as u8;

        // we multiply 4/pi to put it in the range [0,8)
        Some(Phase::try_from(index).expect("This should be in range"))
    }

    /// Given some day, gets the radians relative to the periapsis.
    pub fn get_orbit_radians(&self, seconds: u32) -> f64 {
        f64::from(seconds + self.shift) / f64::from(self.period) % 1.0 * 2.0 * PI
    }

    /// Calculates the distance between a body and its parent.
    pub fn get_distance(&self, seconds: u32) -> f64 {
        let radians = self.get_orbit_radians(seconds);
        self.semimajor_axis() * (1.0 - self.eccentricity.powf(2.0))
            / (1.0 + self.eccentricity * radians.cos())
    }

    /// Gets the semimajor axis of the orbit ie. the furthest
    /// distance of orbit. This is dependent on eccentricity.
    pub fn semimajor_axis(&self) -> f64 {
        f64::from(self.period).powf(2.0).powf(-3.0)
    }

    /// Validates an orbit against a calendar,
    /// ensuring the period is correct.
    pub fn validate_calendar(&self, calendar: &Calendar) -> Result<bool, ValidationError> {
        let calendar_period = calendar.years_to_seconds(1);

        if self.period != calendar_period {
            Err(ValidationError::InconsistentPeriod(
                self.period,
                calendar_period,
            ))
        } else {
            Ok(true)
        }
    }
}

#[derive(Error, Debug, Copy, Clone)]
pub enum ValidationError {
    #[error("the orbial period is inconsistent. planet: {0}, calendar: {1}")]
    InconsistentPeriod(u32, u32),
}

/// A phase is exhibited by 'grandchild' objects in orbit,
/// as the light from a planet's parent hits its children.
#[derive(IntoPrimitive, Clone, Debug, TryFromPrimitive, Copy, Display)]
#[repr(u8)]
#[allow(missing_docs)]
pub enum Phase {
    #[strum(serialize = "a brilliant gleaming disk in the dark")]
    Full,
    #[strum(serialize = "in waning gibbous, beginning to retreat into darkness")]
    WaningGibbous,
    #[strum(serialize = "in the half-shadow of the third quarter")]
    ThirdQuarter,
    #[strum(serialize = "in waning crescent, nearly covered in darkness")]
    WaningCrescent,
    #[strum(serialize = "a silky hole in the starry sky")]
    New,
    #[strum(serialize = "in waxing crescent, light creeping out")]
    WaxingCrescent,
    #[strum(serialize = "in the half-light of the first quarter")]
    FirstQuarter,
    #[strum(serialize = "in waxing gibbous, nearly fully lit")]
    WaxingGibbous,
}

impl Phase {
    /// Maps the moon phases to unicode images.
    pub fn unicode(&self) -> &str {
        match self {
            Self::Full => "🌕",
            Self::WaningGibbous => "🌖",
            Self::ThirdQuarter => "🌗",
            Self::WaningCrescent => "🌘",
            Self::New => "🌑",
            Self::WaxingCrescent => "🌒",
            Self::FirstQuarter => "🌓",
            Self::WaxingGibbous => "🌔",
        }
    }
}
