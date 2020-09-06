#![warn(
    clippy::todo,
    clippy::unwrap_used,
    clippy::unused_self,
    clippy::unimplemented,
    clippy::trivially_copy_pass_by_ref,
    clippy::panic
)]

use anyhow::Result;
use clap::Clap;
use human_panic::setup_panic;

use opts::Command;

mod opts;
mod persistence;
mod world;

pub use opts::Opts;

fn main() -> Result<()> {
    setup_panic!();

    let opts: Opts = Opts::parse();
    let path = opts.path.unwrap_or(".".into());

    match opts.command {
        Command::Dice(d) => d.run()?,
        #[cfg(feature = "sound")]
        Command::Sound(s) => s.run()?,
        Command::Now(n) => n.run(&path)?,
        Command::Time(c) => c.run(&path)?,
        Command::Planetarium(b) => b.run(&path)?,
        Command::Record(r) => r.run(&path)?,
        Command::New(n) => n.run(&path)?,
        Command::Weather(w) => w.run(&path)?,
        Command::Completion(c) => c.run()?,
    };

    Ok(())
}
