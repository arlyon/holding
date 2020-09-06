use std::io;

use anyhow::Result;
use clap::{Clap, IntoApp};
use clap_generate::{generate, generators::Bash};

use crate::Opts;

/// Generate bash completions
#[derive(Clap)]
pub struct Completion {}

impl Completion {
    pub fn run(&self) -> Result<()> {
        let mut app = Opts::into_app();
        generate::<Bash, _>(&mut app, "holding", &mut io::stdout());
        Ok(())
    }
}
