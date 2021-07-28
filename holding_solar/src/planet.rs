use holding_kronos::calendar::{traits::ConvertTime, Calendar};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

use holding_color::Color;

use crate::orbit::{self, Orbit};

/// A unique identifier for celestial bodies.
#[derive(Copy, Clone, Deserialize, Serialize, PartialEq, Eq, Debug)]
pub struct PlanetId(pub Uuid);

/// A celestial body.
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct CelestialBody {
    /// The id of the body.
    pub id: PlanetId,

    /// The name of the body.
    pub name: String,

    /// The orbit of the body.
    pub orbit: Option<Orbit>,

    /// The color of the body.
    pub color: Color,

    /// The ids of the planets in orbit of this one.
    pub children: Vec<PlanetId>,

    /// The number of seconds the planet
    /// takes to rotate.
    pub rotational_period: u32,

    /// The temperature in degrees kelvin.
    pub temperature: i32,
}

impl CelestialBody {
    /// Creates a new `CelestialBody`.
    pub fn new(name: String, temperature: i32, rotational_period: u32, color: Color) -> Self {
        Self {
            id: PlanetId(Uuid::new_v4()),
            name,
            color,
            temperature,
            rotational_period,
            orbit: None,
            children: vec![],
        }
    }

    /// Calculates whether a planet is luminous.
    ///
    /// https://qph.fs.quoracdn.net/main-qimg-6bce97c94fb60cc1fe025591653de493
    pub fn is_luminous(&self) -> bool {
        self.temperature > 3500
    }

    /// Adds a new moon to this planet.
    pub fn with_moon(&mut self, moon: &mut CelestialBody, period: u32) -> &mut Self {
        let orbit = Orbit::from_period(moon, self.id, period, 0);
        moon.orbit = Some(orbit);
        self.children.push(moon.id);
        self
    }

    /// Sets the parent of this planet.
    pub fn with_parent(&mut self, parent: &mut CelestialBody, period: u32) -> &mut Self {
        let orbit = Orbit::from_period(self, parent.id, period, 0);
        self.orbit = Some(orbit);
        parent.children.push(self.id);
        self
    }

    /// Validates an orbit against a calendar,
    /// ensuring the rotational and orbital
    /// periods are correct.
    pub fn validate_calendar(&self, calendar: &Calendar) -> Result<bool, ValidationError> {
        let planet_period = self.rotational_period;
        let calendar_period = calendar.days_to_seconds(1);
        let x = if planet_period != calendar_period {
            Err(ValidationError::InconsistentRotationalPeriod(
                planet_period,
                calendar_period,
            ))
        } else {
            Ok(true)
        };

        if let Some(y) = self.orbit.as_ref().map(|o| o.validate_calendar(calendar)) {
            x.and(y.map_err(Into::into))
        } else {
            x
        }
    }
}

#[derive(Error, Debug, Copy, Clone)]
pub enum ValidationError {
    #[error("the rotational period is inconsistent. planet: {0}, calendar: {1}")]
    InconsistentRotationalPeriod(u32, u32),
    #[error("invalid orbit: {0}")]
    OrbitValidationError(#[from] orbit::ValidationError),
}

/// Keeps track of `CelestialBody`s.
pub trait PlanetStore {
    /// Get a `CelestialBody` from the `PlanetStore`.
    fn get_planet(&self, id: PlanetId) -> Option<&CelestialBody>;

    /// Get a mutable reference to a `CelestialBody` from the `PlanetStore`.
    fn get_planet_mut(&mut self, id: PlanetId) -> Option<&mut CelestialBody>;

    /// Create a new `CelestialBody` in the `PlanetStore`.
    fn create_planet(
        &mut self,
        name: String,
        temperature: i32,
        rotational_period: u32,
        color: Color,
    ) -> &CelestialBody;

    /// Adds an orbit.
    ///
    /// todo(arlyon): Allow this to fail if
    /// - either planet doesn't exist
    /// - the child is orbiting something else
    fn add_orbit(&mut self, parent_id: PlanetId, child_id: PlanetId, period: u32) {
        let mut child = self.get_planet_mut(child_id).unwrap();
        let orbit = Orbit::from_period(child, parent_id, period, 0);
        child.orbit = Some(orbit);
        let parent = self.get_planet_mut(parent_id).unwrap();
        parent.children.push(child_id);
    }
}
