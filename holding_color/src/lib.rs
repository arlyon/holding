//! holding_color: An abstraction over colours.

#![deny(
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unreachable_code,
    missing_copy_implementations,
    unused_import_braces,
    unused_qualifications
)]
#![warn(
    clippy::todo,
    clippy::unwrap_used,
    clippy::unused_self,
    clippy::unimplemented,
    clippy::trivially_copy_pass_by_ref,
    clippy::panic
)]

use colored::Color as Colored;
use serde::{Deserialize, Serialize};

pub use colored;

/// Basic colors that things can be in the `holding` world.
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[allow(missing_docs)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

impl From<Color> for Colored {
    fn from(c: Color) -> Self {
        match c {
            Color::Black => Colored::Black,
            Color::Red => Colored::Red,
            Color::Green => Colored::Green,
            Color::Yellow => Colored::Yellow,
            Color::Blue => Colored::Blue,
            Color::Magenta => Colored::Magenta,
            Color::Cyan => Colored::Cyan,
            Color::White => Colored::White,
        }
    }
}
