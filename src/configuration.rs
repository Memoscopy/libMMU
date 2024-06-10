mod config_struct;

use std::fs;
use std::path::Path;
use std::env;
use anyhow::{Result, Context};
use crate::config_struct::{MachineConfig, MemorySpace, SpaceType, RiscVMMUMode};

/// Loads configuration from a specified file.
/// 
/// Args:
/// - `file_path`: The path to the configuration file.
/// 
/// Returns:
/// - A `Result` containing `MachineConfig` or an error.
/// 
/// Description:
/// - This function reads a configuration file and deserializes it into a `MachineConfig` instance.
/// 
/// Purpose:
/// - To abstract the details of loading and parsing configuration files into a single reusable function.
///
/// Technical Explanation:
/// - The function determines the file format based on the extension and uses the appropriate Serde deserializer.
/// - It handles TOML format.
///
/// Example:
/// ```rust
/// let config = load_config("config.toml").unwrap();
/// println!("{:?}", config);
/// ```
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
/// Validates the contents of the configuration.
/// 
/// Args:
/// - `config`: The configuration to validate.
/// 
/// Returns:
/// - A `Result` indicating whether the validation was successful.
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

///
///fn main() -> Result<()> {
///     //Collect command line arguments
///    let args: Vec<String> = env::args().collect();
///    // Check if a file path is provided
///    if args.len() < 2 {
///        eprintln!("Usage: {} <path_to_config_file>", args[0]);
///        std::process::exit(1);
///    }
///    // The file path is the second argument
///    let config_path = Path::new(&args[1]);
///    // Output the path for debugging purposes
///    println!("Loading configuration from: {:?}", config_path);
///    // Attempt to load the configuration
///    let file_conf = load_config(config_path)?;
///    // Output the loaded configuration for verification
///    println!("Loaded Configuration: {:#?}", file_conf);
///
///    // Manually convert `mmu_mode` to `RiscVMMUMode`
///    let mmu_mode: Option<RiscVMMUMode> = match file_conf.mmu_mode.as_deref() {
///        Some("SV39") => Some(RiscVMMUMode::SV39),
///        Some("SV48") => Some(RiscVMMUMode::SV48),
///        _ => None,
///    };
///
///    // Inline configuration example
///    let inline_conf = MachineConfig {
///        mmu_mode: Some("SV39".to_string()),
///        memspaces: vec![
///            MemorySpace {
///                space_type: SpaceType::RAM,
///                start_address: 0x0000000080000000,
///                end_address: 0x000000017fffffff,
///            },
///            MemorySpace {
///                space_type: SpaceType::ROM,
///                start_address: 0x0000000000000000,
///                end_address: 0x0000000000011fff,
///            },
///        ],
///        dumpfile: Some("dump.raw".to_string()),
///        outfile: Some("output.txt".to_string()),
///    };
///
///    // Output the loaded configuration and the manually converted MMU mode
///    println!("File-based Configuration: {:#?}", file_conf);
///    println!("MMU Mode: {:?}", mmu_mode);
///    println!("Inline Configuration: {:#?}", inline_conf);
///    Ok(())
///}
///