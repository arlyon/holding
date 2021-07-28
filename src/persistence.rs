use std::{
    ffi::OsStr,
    fs::{create_dir_all, File},
    path::Path,
};

use anyhow::{anyhow, Context, Result};
use thiserror::Error;

use crate::world::World;

/// Loads or creates a world at a given path.
/// Fails if the path exists and the world
/// could not be read.
pub fn load_world(path: &Path) -> Result<World> {
    if !path.exists() {
        return Err(anyhow!("Invalid path."));
    };

    let mut path = path.to_path_buf();
    if path.file_name() != Some(OsStr::new("world.yaml")) {
        path.push("world.yaml");
    }

    let f = File::open(&path).context("Couldn't find world.")?;
    let world: World = serde_yaml::from_reader(f).context("world file is corrupted.")?;

    world.validate().context("Could not validate the world.")?;

    Ok(world)
}

pub fn save_world(path: &Path, world: &World) -> Result<()> {
    if !path.exists() {
        return Err(anyhow!("Invalid path."));
    };

    let mut path = path.to_path_buf();
    if path.file_name() != Some(OsStr::new("world.yaml")) {
        path.push("world.yaml");
    }

    let f = File::create(&path).context("Couldn't find world.")?;
    serde_yaml::to_writer(f, world).context("world file is corrupted.")?;
    Ok(())
}

#[derive(Error, Debug)]
enum WorldCreationError {
    #[error("There is already a folder at this path.")]
    PathExists,
}

pub fn create_world(path: &Path, name: String, force: bool) -> Result<World> {
    if path.exists() && path.read_dir()?.next().is_some() && !force {
        return Err(WorldCreationError::PathExists.into());
    };

    create_dir_all(path)?;

    let mut path = path.to_path_buf();
    if path.file_name() != Some(OsStr::new("world.yaml")) {
        path.push("world.yaml");
    }

    let f = File::create(&path)?;
    let world = World {
        name,
        ..Default::default()
    };
    serde_yaml::to_writer(f, &world)?;

    Ok(world)
}
