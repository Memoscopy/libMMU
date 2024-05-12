use serde::{Serialize, Deserialize};
use std::fs;
use std::error::Error;
use std::env;

/// Represents the main configuration for a machine.
/// 
/// Fields:
/// - `mmu_mode`: Optional string that specifies the MMU mode, which will be manually converted to a specific type.
/// - `memspaces`: A vector of `MemorySpace` that defines different memory regions.
/// - `dumpfile`: Optional string specifying the path to a dump file.
/// - `outfile`: Optional string specifying the path to an output file.
/// 
/// Description:
/// - This struct is used to parse and hold the configuration data of a machine from various file formats.
/// - It includes memory spaces, MMU mode, and optional paths for dump and output files.
/// 
/// Purpose:
/// - To provide a structured representation of machine configuration that can be easily loaded, manipulated, and accessed.
///
/// Technical Explanation:
/// - The `mmu_mode` is stored as a string to allow flexibility in how it is represented and used in different contexts.
/// - `memspaces` is a vector of `MemorySpace`, which details the type and address range of memory spaces.
#[derive(Debug, Serialize, Deserialize)]
struct MachineConfig {
    mmu_mode: Option<String>,
    memspaces: Vec<MemorySpace>,
    dumpfile: Option<String>,
    outfile: Option<String>,
}

/// Defines a memory space with a type and address range.
#[derive(Serialize, Deserialize, Debug)]
struct MemorySpace {
    space_type: SpaceType,
    start_address: u64,
    end_address: u64,
}

/// Enumerates types of memory spaces.
#[derive(Serialize, Deserialize, Debug)]
enum SpaceType {
    RAM,
    ROM,
}

/// Enumerates RISC-V MMU modes.
#[derive(Debug, Serialize, Deserialize)]
enum RiscVMMUMode {
    SV39,
    SV48,
}

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
/// - It handles JSON, YAML, TOML, and XML formats.
///
/// Example:
/// ```rust
/// let config = load_config("config.json").unwrap();
/// println!("{:?}", config);
/// ```
fn load_config(file_path: &str) -> Result<MachineConfig, Box<dyn Error>> {
    let config_str = fs::read_to_string(file_path)?;
    let config: MachineConfig = match file_path.rsplit('.').next().ok_or("No file extension found")? {
        "json" => serde_json::from_str(&config_str)?,
        "yaml" => serde_yaml::from_str(&config_str)?,
        "toml" => toml::from_str(&config_str)?,
        "xml" => serde_xml_rs::from_str(&config_str)?,
        _ => return Err("Unsupported file format".into()),
    };
    Ok(config)
}

/// Main function demonstrating configuration loading and usage.
/// 
/// Example:
/// - This example loads a configuration from a JSON file and demonstrates how to manually convert the `mmu_mode`.
fn main() -> Result<(), Box<dyn Error>> {
    // Collect command line arguments
    let args: Vec<String> = env::args().collect();
    // Check if a file path is provided
    if args.len() < 2 {
        eprintln!("Usage: {} <path_to_config_file>", args[0]);
        std::process::exit(1);
    }
    // The file path is the second argument
    let config_path = &args[1];
    // Output the path for debugging purposes
    println!("Loading configuration from: {}", config_path);
    // Attempt to load the configuration
    let file_conf = load_config(config_path)?;
    // Output the loaded configuration for verification
    println!("Loaded Configuration: {:#?}", file_conf);

    let mmu_mode: Option<RiscVMMUMode> = match file_conf.mmu_mode {
        Some(ref mode) => match mode.as_str() {
            "SV39" => Some(RiscVMMUMode::SV39),
            "SV48" => Some(RiscVMMUMode::SV48),
            _ => None,
        },
        None => None,
    };
    let inline_conf = MachineConfig {
        mmu_mode: Some("SV39".to_string()),
        memspaces: vec![
            MemorySpace {
                space_type: SpaceType::RAM,
                start_address: 0x0000000080000000,
                end_address: 0x000000017fffffff,
            },
            MemorySpace {
                space_type: SpaceType::ROM,
                start_address: 0x0000000000000000,
                end_address: 0x0000000000011fff,
            },
        ],
        dumpfile: Some("dump.raw".to_string()),
        outfile: Some("output.txt".to_string()),
    };
    // Output the loaded configuration and the manually converted MMU mode.
    println!("File-based Configuration: {:#?}", file_conf);
    println!("MMU Mode: {:?}", mmu_mode);
    println!("Inline Configuration: {:#?}", inline_conf);
    Ok(())
}