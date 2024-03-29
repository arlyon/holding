use std::path::PathBuf;

use clap::Clap;

use bodies::Planetarium;
use calendar::Time;
use characters::Characters;
use completion::Completion;
use dice::Dice;
use history::History;
use new::New;
use now::Now;
use record::AddRecord;
#[cfg(feature = "sound")]
use sound::Sound;
use weather::Weather;

mod bodies;
mod calendar;
mod characters;
mod completion;
mod dice;
mod history;
mod host;
mod new;
mod now;
mod record;
#[cfg(feature = "sound")]
mod sound;
mod weather;

/// Manage a DND world from the command line.
#[derive(Clap)]
#[clap(bin_name="holding", version = env!("CARGO_PKG_VERSION"))]
pub struct Opts {
    /// The path to the world (defaulting to here).
    #[clap(short)]
    pub path: Option<PathBuf>,

    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Clap)]
pub enum Command {
    Dice(Dice),
    #[clap(subcommand)]
    Characters(Characters),
    Now(Now),
    New(New),
    #[clap(subcommand)]
    Time(Time),
    Record(AddRecord),
    History(History),
    Weather(Weather),
    #[clap(subcommand)]
    Planetarium(Planetarium),
    #[cfg(feature = "sound")]
    Sound(Sound),
    Completion(Completion),
    // Host(Host),
    // Join(Join),
}
