use std::path::Path;

use anyhow::Result;
use clap::Clap;
use holding_color::colored::Colorize;
use itertools::Itertools;

use crate::persistence::{load_world, save_world};

/// Manipulate the very flow of time itself.
#[derive(Clap)]
pub enum Time {
    /// Steps forward in the flow of time.
    Step(TimeSwitch),

    /// Temporarily opens a rift to a new location in time, preserving your place.
    Jump(TimeSwitch),

    /// Returns to the 'canonical' time line.
    Return,
}

#[derive(Clap)]
pub struct TimeSwitch {
    expr: String,
}

impl Time {
    pub fn run(&self, path: &Path) -> Result<()> {
        let mut world = load_world(path)?;

        match self {
            Time::Step(TimeSwitch { expr }) => {
                let prev = world.time;
                world.update_time(expr)?;
                println!("The time is now {}", world.time);

                let events = world.records_between(prev, world.time);
                if !events.is_empty() {
                    println!("Things have happened:");
                    println!(
                        "{}",
                        events
                            .iter()
                            .map(|r| format!(
                                "- {} | {}",
                                r.date.to_string().bright_black(),
                                r.pretty()
                            ))
                            .join("\n")
                    );
                }
            }
            Time::Jump(TimeSwitch { expr }) => {
                world.jump_time(expr)?;
                println!("You open a rift and step through.");
                println!("The time is now {}", world.time);
            }
            Time::Return => {
                world.return_time()?;
                println!("You open a rift and step through.");
                println!("You have returned to {}.", world.time)
            }
        }

        save_world(path, &world)?;
        Ok(())
    }
}
