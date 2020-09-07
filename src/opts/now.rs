use std::path::PathBuf;

use ordinal::Ordinal;

use anyhow::Result;
use clap::Clap;

use crate::persistence::load_world;
use holding_color::colored::*;
use holding_solar::PlanetStore;

/// Observe your surroundings.
#[derive(Clap)]
pub struct Now;

impl Now {
    pub fn run(&self, path: &PathBuf) -> Result<()> {
        let world = load_world(&path)?;
        let time_with_cal = world.time.with_calendar(&world.calendar);
        let time_of_day = time_with_cal.time_of_day();
        let is_day = time_of_day.is_day();
        println!(
            "It is {:0>2}:{:0>2}, {} on {} the {} day of {} in the year {}\n",
            time_with_cal.hour(),
            time_with_cal.minute(),
            time_of_day,
            time_with_cal.week_day_name(),
            Ordinal(time_with_cal.month_day()),
            time_with_cal.month_name(),
            time_with_cal.year()
        );

        if world.jumped() {
            println!(
                "{}\n",
                "⧗ A sting in your temporal lobe indicates that this is not your native timeline..."
                    .bright_black()
            )
        }

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
                let phase = child
                    .orbit
                    .and_then(|o| o.get_phase(&world, world.time.with_calendar(&world.calendar)));
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
