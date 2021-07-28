use anyhow::Result;
use clap::Clap;

use crate::{
    character::CharacterStore,
    persistence::{load_world, save_world},
};

/// See and change the characters in this world.
#[derive(Clap)]
pub enum Locations {
    /// Show all characters alphabetically.
    List,

    /// Shows characters that are around in the current time.
    Now,

    /// Find a character by its name or identifier.
    Find(Search),

    /// Shows characters that are nearby.
    Nearby,
}

#[derive(Clap)]
pub struct Search {
    expr: String,
}

impl Locations {
    pub fn run(&self, path: &Path) -> Result<()> {
        let mut world = load_world(&path)?;

        match self {
            Locations::List => {
                let chars = world.list_locations();
                println!("{:?}", chars);
            }
            Locations::Now => todo!(),
            Locations::Find(Search { expr }) => todo!(),
            Locations::Nearby => todo!(),
        }

        save_world(&path, &world)?;
        Ok(())
    }
}
