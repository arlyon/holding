use std::path::Path;

use anyhow::Result;
use clap::Clap;

use crate::{
    character::CharacterStore,
    persistence::{load_world, save_world},
};

/// See and change the characters in this world.
#[derive(Clap)]
pub enum Characters {
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

impl Characters {
    pub fn run(&self, path: &Path) -> Result<()> {
        let world = load_world(path)?;

        match self {
            Characters::List => {
                let chars = world.list_characters();
                println!("{:?}", chars);
            }
            Characters::Now => todo!(),
            Characters::Find(_) => todo!(),
            Characters::Nearby => todo!(),
        }

        save_world(path, &world)?;
        Ok(())
    }
}
