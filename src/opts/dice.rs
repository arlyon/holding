use anyhow::Result;
use clap::Clap;
use dnd_dice_roller::dice_set::DiceSet;
use itertools::Itertools;

/// Tempt fate and throw some dice.
#[derive(Clap)]
pub struct Dice {
    sets: Vec<DiceSet>,
}

impl Dice {
    pub fn run(&self) -> Result<()> {
        println!(
            "{}",
            self.sets
                .iter()
                .map(|set| set.roll_dice_set().final_result)
                .join(", ")
        );

        Ok(())
    }
}
