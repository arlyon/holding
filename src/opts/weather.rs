use std::path::Path;

use anyhow::Result;
use clap::Clap;

/// Prints information about the current weather.
#[derive(Clap)]
pub struct Weather {}

impl Weather {
    pub fn run(&self, _path: &Path) -> Result<()> {
        todo!();
    }
}
