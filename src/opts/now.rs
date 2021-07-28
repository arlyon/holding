use std::path::Path;

use ordinal::Ordinal;

use anyhow::Result;
use clap::Clap;
use holding_kronos::datetime::traits::{ShowDate, ShowTime};

use crate::persistence::load_world;
use holding_color::colored::*;
use holding_solar::PlanetStore;

/// Observe your surroundings.
#[derive(Clap)]
pub struct Now;

impl Now {
    pub fn run(&self, path: &Path) -> Result<()> {
        let world = load_world(path)?;
        let time = world.time.into_datetime(&world.calendar);

        println!(
            "It is {:0>2}:{:0>2}, {} on {} the {} day of {} in the year {}\n",
            time.hour(),
            time.minute(),
            time.time_of_day(),
            time.week_day_name(),
            Ordinal(time.day()),
            time.month_name(),
            time.year()
        );

        if world.jumped() {
            println!(
                "{}\n",
                "⧗ A sting in your temporal lobe indicates that this is not your native timeline..."
                    .bright_black()
            )
        }

        let is_day = time.time_of_day().is_day();
        if let Some(home) = world.get_planet(world.home_planet) {
            let night_status = if is_day { "the sky" } else { "the night sky" };
            println!(
                "You look up at {} from {} and you see",
                night_status,
                home.name.color(Color::from(home.color)),
            );

            if is_day {
                if let Some(orbit) = &home.orbit {
                    if let Some(p) = world.get_planet(orbit.parent) {
                        let status = if p.is_luminous() {
                            "shining brightly"
                        } else {
                            "hanging ominously"
                        };
                        let name = p.name.color(Color::from(p.color)).bold();
                        println!(
                            "- The planet is oribiting around {}, {} in the sky.",
                            name, status
                        );
                    }
                }
            }

            for child in home.children.iter().filter_map(|c| world.get_planet(*c)) {
                let name = child.name.color(Color::from(child.color)).bold();
                let phase = child.orbit.and_then(|o| o.get_phase(&world, time));
                if let Some(phase) = phase {
                    println!("- {} The moon {} is {}.", phase.unicode(), name, phase);
                } else {
                    println!("- The moon {} is floating in the sky.", name);
                }
            }

            if home.orbit.is_none() && home.children.is_empty() {
                println!("Space is a cold and empty place.")
            }
        }

        Ok(())
    }
}
