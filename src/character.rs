use holding_solar::PlanetId;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct CharacterId(pub Uuid);

impl Default for CharacterId {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(PartialEq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Class {
    Fighter(Fighter),
    Warlock(Warlock),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Race {
    Unknown,
    Human,
    Dwarf,
}

impl Default for Race {
    fn default() -> Self {
        Self::Unknown
    }
}

#[derive(PartialEq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Fighter {
    Champion,
    BattleMaster,
    EldrichKnight,
}

#[derive(PartialEq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Warlock {
    ArchFey,
    Fiend,
    GreatOldOne,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    pub id: CharacterId,
    pub identifier: String,
    location: Option<LocationId>,
    name: String,
    description: Description,

    race: Race,
    health: Health,
    inspiration: bool,
    classes: [(Option<Class>, u8); 4],

    /// Only set if working with experience
    experience: Option<u32>,
}

impl Character {
    pub fn new(
        identifier: String,
        name: Option<String>,
        max_health: Option<u32>,
        race: Option<Race>,
    ) -> Self {
        Character {
            id: CharacterId(Uuid::new_v4()),
            name: name.unwrap_or_else(|| identifier.clone()),
            identifier,
            health: max_health.map(Health::new).unwrap_or_default(),
            race: race.unwrap_or_default(),
            ..Default::default()
        }
    }

    pub fn level(&self) -> u8 {
        self.classes
            .iter()
            .filter(|(c, _)| c.is_some())
            .map(|(_, l)| l)
            .sum()
    }

    pub fn level_up(&mut self, class: Class) -> Result<(), ()> {
        if let Some((_, level)) = self.classes.iter_mut().find(|(c, _)| Some(class).eq(c)) {
            *level += 1;
            if let Some(x) = &mut self.experience {
                *x = 0;
            }
            Ok(())
        } else {
            Err(())
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Health {
    max: u32,
    status: HealthStatus,
}

#[derive(PartialEq, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum HealthStatus {
    Alive(Hitpoints),
    Unconscious(DeathSaves),
    Dead,
}

impl Default for HealthStatus {
    fn default() -> Self {
        Self::Alive(Default::default())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct DeathSaves {
    successes: u8,
    failures: u8,
}

impl DeathSaves {
    pub fn damage(self, critical: bool) -> HealthStatus {
        match self.failures + (if critical { 1 } else { 2 }) {
            x if x < 3 => HealthStatus::Unconscious(Self {
                failures: x,
                successes: self.successes,
            }),
            _ => HealthStatus::Dead,
        }
    }
    pub fn heal(self, heal: u32) -> HealthStatus {
        HealthStatus::Alive(Hitpoints {
            current: heal,
            bonus: 0,
        })
    }
}

#[derive(PartialEq, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Hitpoints {
    current: u32,
    bonus: u32,
}

impl Default for Hitpoints {
    fn default() -> Self {
        Self {
            current: 1,
            bonus: 0,
        }
    }
}

impl Hitpoints {
    pub fn hp(self) -> u32 {
        self.current + self.bonus
    }

    pub fn damage(self, amount: u32) -> HealthStatus {
        let (bonus, amount) = {
            let rem = self.bonus.saturating_sub(amount);
            (rem, amount - (self.bonus - rem))
        };
        let next = Self {
            current: self.current.saturating_sub(amount),
            bonus,
        };

        // todo(arlyon): handle instant death
        if next.hp() == 0 {
            HealthStatus::Unconscious(Default::default())
        } else {
            HealthStatus::Alive(next)
        }
    }

    pub fn heal(self, amount: u32) -> HealthStatus {
        // todo(arlyon): saturate at max
        HealthStatus::Alive(Hitpoints {
            current: self.current + amount,
            bonus: self.bonus,
        })
    }
}

impl Health {
    pub fn new(max: u32) -> Self {
        Health {
            max,
            ..Default::default()
        }
    }

    pub fn status(&self) -> HealthStatus {
        self.status
    }

    pub fn damage(&mut self, damage: u32, critical: bool) -> HealthStatus {
        self.status = match self.status {
            HealthStatus::Alive(hp) => hp.damage(damage), // todo(arlyon): critical hits
            HealthStatus::Unconscious(ds) => ds.damage(critical),
            HealthStatus::Dead => HealthStatus::Dead,
        };

        self.status
    }

    pub fn heal(&mut self, health: u32) -> HealthStatus {
        self.status = match self.status {
            HealthStatus::Alive(hp) => hp.heal(health), // todo(arlyon): critical hits
            HealthStatus::Unconscious(ds) => ds.heal(health),
            HealthStatus::Dead => HealthStatus::Dead,
        };

        self.status
    }

    pub fn dead(self) -> bool {
        matches!(self.status, HealthStatus::Dead)
    }

    pub fn full(self) -> bool {
        match self.status {
            HealthStatus::Alive(hp) => hp.current == self.max,
            _ => false,
        }
    }
}

impl Default for Health {
    fn default() -> Self {
        Self {
            max: 1,
            status: Default::default(),
        }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Description {
    age: Option<u32>,

    /// height in cm
    height: Option<u32>,

    /// wight in kg
    weight: Option<u32>,

    backstory: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
/// Tracks a character reference inside a record string.
pub struct CharacterReference {
    pub id: CharacterId,
    pub string: String,
    pub start: usize,
    pub end: usize,
}

pub trait CharacterStore {
    fn get_character(&self, id: CharacterId) -> Option<Character>;
    fn list_characters(&self) -> Vec<Character>;
    fn lookup_character(&self, search: &str) -> Option<Character>;
    fn create_character(&self) -> Character;
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct LocationId(pub Uuid);

#[derive(Clone, Serialize, Deserialize)]
pub struct Location {
    pub id: LocationId,
    pub identifier: String,
    pub planet: PlanetId,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LocationReference {
    pub id: LocationId,
    pub string: String,
    pub start: usize,
    pub end: usize,
}

pub trait LocationStore {
    fn get_location(&self, id: LocationId) -> Option<Location>;
    fn list_locations(&self) -> Vec<Location>;
    fn lookup_location(&self, search: &str) -> Option<Location>;
    fn create_location(&mut self, ident: &str) -> Location;
}

#[cfg(test)]
mod test {
    use crate::character::{HealthStatus, Hitpoints};
    use test_case::test_case;

    #[test_case(3, 5, 5, 3, 0)]
    #[test_case(10, 2, 5, 7, 0)]
    #[test_case(1, 10, 5, 1, 5)]
    pub fn do_damage(current: u32, bonus: u32, damage: u32, exp_current: u32, exp_bonus: u32) {
        let hp = Hitpoints { current, bonus };
        let next = hp.damage(damage);
        assert_eq!(
            next,
            HealthStatus::Alive(Hitpoints {
                current: exp_current,
                bonus: exp_bonus
            })
        );
    }
}
