use std::{
    ffi::OsStr,
    fs::{create_dir_all, File},
    path::PathBuf,
};

use anyhow::{anyhow, Context, Result};
use thiserror::Error;

use crate::world::World;

/// Loads or creates a world at a given path.
/// Fails if the path exists and the world
/// could not be read.
pub fn load_world(path: &PathBuf) -> Result<World> {
    if !path.exists() {
        return Err(anyhow!("Invalid path."));
    };

    let mut path = path.clone();
    if path.file_name() != Some(OsStr::new("world.yaml")) {
        path.push("world.yaml");
    }

    let f = File::open(&path).context("Couldn't find world.")?;
    let world_data: World = serde_yaml::from_reader(f).context("world file is corrupted.")?;
    world_data
        .validate()
        .context("Could not validate the world.")?;
    Ok(world_data)
}

pub fn save_world(path: &PathBuf, data: &World) -> Result<()> {
    if !path.exists() {
        return Err(anyhow!("Invalid path."));
    };

    let mut path = path.clone();
    if path.file_name() != Some(OsStr::new("world.yaml")) {
        path.push("world.yaml");
    }

    let f = File::create(&path).context("Couldn't find world.")?;
    serde_yaml::to_writer(f, data).context("world file is corrupted.")?;
    Ok(())
}

#[derive(Error, Debug)]
enum WorldCreationError {
    #[error("There is already a folder at this path.")]
    PathExists,
}

pub fn create_world(path: &PathBuf, name: String, force: bool) -> Result<World> {
    if path.exists() && path.read_dir()?.next().is_some() && !force {
        Err(WorldCreationError::PathExists)?;
    };

    create_dir_all(path)?;

    let mut path = path.clone();
    if path.file_name() != Some(OsStr::new("world.yaml")) {
        path.push("world.yaml");
    }

    let f = File::create(&path)?;
    let mut world = World::default();
    world.name = name;
    serde_yaml::to_writer(f, &world)?;
    Ok(world)
}
