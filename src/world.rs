use crate::character::{Character, CharacterReference, Location, LocationId, LocationReference};
use crate::character::{CharacterStore, LocationStore};
use anyhow::{anyhow, Result};
use holding_color::Color;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};

use holding_color::colored::*;
use holding_kronos::{
    calendar::Calendar,
    datetime::{traits::ModifyDateTime, DateTime, RawDateTime},
};
use holding_solar::{CelestialBody, PlanetId, PlanetStore};
use uuid::Uuid;

use crate::{character::CharacterId, record::RawRecord};

lazy_static! {
    static ref CHAR: Regex = Regex::new(r"\B@(?P<name>\w+)\b").expect("compiles");
    static ref LOC: Regex = Regex::new(r"\B#(?P<name>\w+)\b").expect("compiles");
}

/// The data structure for the world.
///
/// To avoid this being self-referential, the
/// datetimes are stored raw (without a ref to
/// the calendar).
///
/// todo(arlyon): Devise a more elegant data-structure.
#[derive(Clone, Serialize, Deserialize)]
pub struct World {
    pub name: String,
    pub calendar: Calendar,
    pub time: RawDateTime,

    /// If Some, it means we have jumped to
    /// a different location in time.
    pub canonical_time: Option<RawDateTime>,
    pub home_planet: PlanetId,
    pub bodies: Vec<CelestialBody>,
    pub records: Vec<RawRecord>,

    pub characters: Vec<Character>,
    pub locations: Vec<Location>,
}

impl World {
    pub fn new(name: String, calendar: Calendar, home_planet: CelestialBody) -> Self {
        let home_planet_id = home_planet.id;
        Self {
            name,
            time: DateTime::from_seconds(0, &calendar).into(),
            canonical_time: None,
            calendar,
            home_planet: home_planet_id,
            bodies: vec![home_planet],
            ..Default::default()
        }
    }

    pub fn jumped(&self) -> bool {
        self.canonical_time.is_some()
    }

    pub fn records_pretty(&self) -> String {
        let records = self.records.iter().map(|r| (r.date, r.pretty()));

        self.canonical_time
            .filter(|t| !self.time.eq(t))
            .map(|t| {
                vec![
                    (self.time, "Now".purple().to_string()),
                    (t, "Canonical Time".bright_purple().to_string()),
                ]
            })
            .unwrap_or_else(|| vec![(self.time, "Now".purple().to_string())])
            .into_iter()
            .chain(records)
            .sorted_by_key(|(x, _)| *x)
            .map(|(date, text)| format!("- {} | {}", date.to_string().bright_black(), text))
            .join("\n")
    }

    /// Add a record to the world. The string accepts any
    /// observation, with some special syntax.
    ///
    /// @character - refers to a character
    /// #location - refers to a location
    pub fn add_record(&mut self, note: String) -> &RawRecord {
        let chars: Vec<CharacterReference> = CHAR
            .captures_iter(&note)
            .filter_map(|caps| {
                caps.name("name").map(|cap| {
                    let character = self.lookup_character(cap.as_str()).expect("match");
                    CharacterReference {
                        id: character.id,
                        string: cap.as_str().to_owned(),
                        start: cap.start(),
                        end: cap.end(),
                    }
                })
            })
            .collect();

        let locs: Vec<LocationReference> = LOC
            .captures_iter(&note)
            .filter_map(|caps| {
                caps.name("name").map(|cap| {
                    let loc = self
                        .lookup_location(cap.as_str())
                        .unwrap_or_else(|| self.create_location(cap.as_str()));
                    LocationReference {
                        id: loc.id,
                        string: cap.as_str().to_owned(),
                        start: cap.start(),
                        end: cap.end(),
                    }
                })
            })
            .collect();

        // println!("{:?}", chars);
        // println!("{:?}", locs);

        let x = RawRecord::new(self.time, note, chars, locs);
        self.records.push(x);
        self.time = self
            .time
            .into_datetime(&self.calendar)
            .add_seconds(1)
            .into();
        self.records.last().unwrap()
    }

    pub fn records_between(&mut self, d1: RawDateTime, d2: RawDateTime) -> Vec<RawRecord> {
        self.records
            .iter()
            .sorted_by_key(|r| r.date)
            .skip_while(|r| r.date < d1)
            .take_while(|r| r.date < d2)
            .map(ToOwned::to_owned)
            .collect()
    }

    pub fn update_time(&mut self, expr: &str) -> Result<()> {
        let cal_time = self.time.into_datetime(&self.calendar);
        let new_time = self.calendar.parse(expr, Some(cal_time))?;

        if new_time < cal_time {
            return Err(anyhow!("Can't go back in time!"));
        }

        self.time = new_time.into();
        Ok(())
    }

    pub fn jump_time(&mut self, expr: &str) -> Result<()> {
        if self.canonical_time.is_none() {
            self.canonical_time = Some(self.time);
        }

        self.time = self
            .calendar
            .parse(expr, Some(self.time.into_datetime(&self.calendar)))?
            .into();
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
        rotational_period: u32,
        color: Color,
    ) -> &CelestialBody {
        let planet = CelestialBody::new(name, temperature, rotational_period, color);
        let index = self.bodies.len();
        self.bodies.push(planet);
        self.bodies.get(index).expect("We just pushed a planet")
    }
}

impl CharacterStore for World {
    fn get_character(&self, id: CharacterId) -> Option<Character> {
        self.characters
            .iter()
            .find(|c| c.id == id)
            .map(ToOwned::to_owned)
    }

    fn lookup_character(&self, search: &str) -> Option<Character> {
        self.characters
            .iter()
            .find(|c| c.identifier == search)
            .map(ToOwned::to_owned)
    }

    fn create_character(&self) -> Character {
        unimplemented!()
    }

    fn list_characters(&self) -> Vec<Character> {
        self.characters.clone()
    }
}

impl LocationStore for World {
    fn get_location(&self, id: LocationId) -> Option<Location> {
        self.locations
            .iter()
            .find(|c| c.id == id)
            .map(ToOwned::to_owned)
    }

    fn lookup_location(&self, search: &str) -> Option<Location> {
        self.locations
            .iter()
            .find(|l| l.identifier == search)
            .map(ToOwned::to_owned)
    }

    fn create_location(&mut self, identifier: &str) -> Location {
        let loc = Location {
            id: LocationId(Uuid::new_v4()),
            identifier: identifier.to_string(),
            planet: PlanetId("68f83dbc-1894-4d2f-b3a2-ec9fe59f8071".parse().unwrap()),
        };
        self.locations.push(loc.clone());
        loc
    }

    fn list_locations(&self) -> Vec<Location> {
        self.locations.clone()
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
