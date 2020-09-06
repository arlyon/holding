use std::path::PathBuf;

use anyhow::Result;
use clap::Clap;

use crate::persistence::{load_world, save_world};

/// Records a piece of information about the world.
#[derive(Clap)]
pub struct AddRecord {
    /// The information that is to be saved.
    note: String,
}

impl AddRecord {
    pub fn run(&self, path: &PathBuf) -> Result<()> {
        let mut world = load_world(&path)?;
        let time = world.time.clone();
        let cal = world.calendar.clone();
        let note = world.add_record(self.note.clone());
        println!("Noted at {}:\n{}", time.with_calendar(&cal), note.note());
        save_world(&path, &world)?;

        Ok(())
    }
}
