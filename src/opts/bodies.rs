use std::path::Path;

use anyhow::Result;
use clap::Clap;
use itertools::Itertools;

use crate::persistence::load_world;

/// Reveal information about celestial bodies.
#[derive(Clap)]
pub enum Planetarium {
    /// Prints all the existing celestial objects in this world.
    List,

    /// Adds a new celestial object to this world.
    Add,

    /// Eradicates a celestial body from this world.
    Delete,
}

impl Planetarium {
    pub fn run(&self, path: &Path) -> Result<()> {
        match self {
            Planetarium::List => {
                let world = load_world(path)?;
                println!("Known bodies:");
                println!(
                    "{}",
                    world
                        .bodies
                        .iter()
                        .map(|b| format!("- {}", b.name))
                        .join("\n")
                );

                Ok(())
            }
            Planetarium::Add => todo!(),
            Planetarium::Delete => todo!(),
        }
    }
}
