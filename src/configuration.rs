use anyhow::Result;
use serde::de::DeserializeOwned;
use std::fs;
use std::path::Path;

use crate::utils::TryFromPath;

use super::architecture::generic::{Machine, CPU, MMU};

/// Represents a configuration file for a machine.
/// It holds information about the machine's architecture, memory regions, and other relevant information.
/// The configuration file must be written in the TOML format.
impl<T, U> TryFromPath for Machine<T, U>
where
    T: CPU + DeserializeOwned,
    U: MMU + DeserializeOwned,
{
    fn from_path(path: &Path) -> Result<Self> {
        let configuration = fs::read_to_string(path)?;
        let machine: Machine<T, U> = toml::from_str(&configuration)?;

        Ok(machine)
    }
}
