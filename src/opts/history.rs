use crate::persistence::load_world;
use std::path::Path;

use anyhow::Result;
use clap::Clap;

/// Inspect the very flow of time.
#[derive(Clap)]
pub struct History {}

impl History {
    pub fn run(&self, path: &Path) -> Result<()> {
        let world = load_world(path)?;
        println!("{}", world.records_pretty());
        Ok(())
    }
}
