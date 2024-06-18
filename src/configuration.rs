use anyhow::Result;
use std::fs;
use std::path::Path;

use crate::utils::TryFromPath;

use super::architecture::generic::Machine;

/// Represents a configuration file for a machine.
/// It holds information about the machine's architecture, memory regions, and other relevant information.
/// The configuration file must be written in the TOML format.
impl TryFromPath for Machine {
    fn from_path(path: &Path) -> Result<Self> {
        let configuration = fs::read_to_string(path)?;
        let machine: Machine = toml::from_str(&configuration)?;

        Ok(machine)
    }
}
