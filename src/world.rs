use anyhow::{anyhow, Result};
use holding_color::Color;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use holding_kronos::{Calendar, DateTime};
use holding_solar::{CelestialBody, PlanetId, PlanetStore};

#[derive(Serialize, Deserialize)]
pub struct World {
    pub name: String,
    pub calendar: Calendar,
    pub time: DateTime,

    /// If Some, it means we have jumped to
    /// a different location in time.
    pub canonical_time: Option<DateTime>,
    pub home_planet: PlanetId,
    pub bodies: Vec<CelestialBody>,
    pub records: Vec<Record>,
}

#[derive(Serialize, Deserialize)]
pub struct Record {
    id: Uuid,
    date: DateTime,
    note: String,
}

impl Record {
    pub fn new(date: DateTime, note: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            date,
            note,
        }
    }

    pub fn note(&self) -> &str {
        &self.note
    }
}

impl PlanetStore for World {
    fn get_planet(&self, id: PlanetId) -> Option<&CelestialBody> {
        self.bodies.iter().find(|p| p.id == id)
    }

    fn get_planet_mut(&mut self, id: PlanetId) -> Option<&mut CelestialBody> {
        self.bodies.iter_mut().find(|p| p.id == id)
    }

    fn create_planet(
        &mut self,
        name: String,
        temperature: i32,
        rotational_period: usize,
        color: Color,
    ) -> &CelestialBody {
        let planet = CelestialBody::new(name, temperature, rotational_period, color);
        let index = self.bodies.len();
        self.bodies.push(planet);
        self.bodies.get(index).expect("We just pushed a planet.")
    }
}

impl Default for World {
    fn default() -> Self {
        let mut world = Self::new(
            "World".to_string(),
            Default::default(),
            CelestialBody::new("Earth".to_string(), 290, 86400, Color::Green),
        );

        let home = world.home_planet;

        let moon = world
            .create_planet("Moon".to_string(), 240, 2419200, Color::White)
            .id;
        world.add_orbit(home, moon, 1);

        let sun = world
            .create_planet("Sun".to_string(), 5800, 2419200, Color::Yellow)
            .id;
        world.add_orbit(sun, home, 365);

        world
    }
}

impl World {
    pub fn new(name: String, calendar: Calendar, home_planet: CelestialBody) -> Self {
        let home_planet_id = home_planet.id;
        Self {
            name,
            time: DateTime::new(0, 1),
            canonical_time: None,
            calendar,
            home_planet: home_planet_id,
            bodies: vec![home_planet],
            records: vec![],
        }
    }

    pub fn jumped(&self) -> bool {
        self.canonical_time.is_some()
    }

    pub fn add_record(&mut self, note: String) -> &Record {
        let x = Record::new(self.time, note);
        self.records.push(x);
        self.time = self.time.with_calendar(&self.calendar).add_seconds(1);
        self.records.last().unwrap()
    }

    pub fn update_time(&mut self, expr: &str) -> Result<()> {
        let new_time = self.calendar.parse(expr, Some(self.time))?;

        if new_time < self.time {
            return Err(anyhow!("Can't go back in time!"));
        }

        self.time = new_time;
        Ok(())
    }

    pub fn jump_time(&mut self, expr: &str) -> Result<()> {
        if self.canonical_time.is_none() {
            self.canonical_time = Some(self.time);
        }

        self.time = self.calendar.parse(expr, Some(self.time))?;
        Ok(())
    }

    pub fn return_time(&mut self) -> Result<()> {
        if let Some(time) = self.canonical_time {
            self.time = time;
            self.canonical_time = None;
            Ok(())
        } else {
            Err(anyhow!("You are already in the canonical time."))
        }
    }

    /// Validates the world.
    pub fn validate(&self) -> Result<bool> {
        self.get_planet(self.home_planet)
            .map(|p| p.validate_calendar(&self.calendar).map_err(Into::into))
            .unwrap_or_else(|| Err(anyhow!("Home planet doest not exist.")))
    }
}
