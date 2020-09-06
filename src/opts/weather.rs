use std::path::PathBuf;

use anyhow::Result;
use clap::Clap;

/// Prints information about the current weather.
#[derive(Clap)]
pub struct Weather {}

impl Weather {
    pub fn run(&self, _path: &PathBuf) -> Result<()> {
        todo!();
    }
}
