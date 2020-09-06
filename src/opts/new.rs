use anyhow::Result;
use clap::Clap;
use std::path::PathBuf;

use crate::persistence::create_world;

/// Forge a new universe
#[derive(Clap)]
pub struct New {
    /// Force create, overwriting existing worlds.
    #[clap(short, long)]
    force: bool,

    /// A name for your new creation.
    name: String,
}

impl New {
    pub fn run(&self, path: &PathBuf) -> Result<()> {
        create_world(path, self.name.clone(), self.force)?;
        println!("Created world {}", self.name);
        Ok(())
    }
}
