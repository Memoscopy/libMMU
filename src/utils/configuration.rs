use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

pub fn load_config(file_path: &Path) -> Result<MachineConfig> {
    let config_str = fs::read_to_string(file_path)
        .with_context(|| format!("Failed to read configuration file: {:?}", file_path))?;
    let mut config: MachineConfig = match file_path.extension().and_then(|s| s.to_str()) {
        Some("toml") => toml::from_str(&config_str)
            .with_context(|| format!("Failed to parse TOML configuration file: {:?}", file_path))?,
        _ => return Err(anyhow::anyhow!("Unsupported file format: {:?}", file_path)),
    };
    validate_config(&config)?;
    Ok(config)
}

pub fn validate_config(config: &MachineConfig) -> Result<()> {
    if let Some(ref mode) = config.mmu_mode {
        match mode.as_str() {
            "SV39" | "SV48" => Ok(()),
            _ => Err(anyhow::anyhow!("Invalid MMU mode: {}", mode)),
        }
    } else {
        Ok(())
    }
}
