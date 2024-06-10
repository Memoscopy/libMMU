use serde::{Serialize, Deserialize};

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
/// 
#[derive(Debug, Serialize, Deserialize)]
pub struct MachineConfig {
    pub mmu_mode: Option<String>,
    pub memspaces: Vec<MemorySpace>,
    pub dumpfile: Option<String>,
    pub outfile: Option<String>,
}

/// Defines a memory space with a type and address range.
#[derive(Serialize, Deserialize, Debug)]
pub struct MemorySpace {
    pub space_type: SpaceType,
    pub start_address: u64,
    pub end_address: u64,
}

/// Enumerates types of memory spaces.
#[derive(Serialize, Deserialize, Debug)]
pub enum SpaceType {
    RAM,
    ROM,
}

/// Enumerates RISC-V MMU modes.
#[derive(Debug, Serialize, Deserialize)]
pub enum RiscVMMUMode {
    SV39,
    SV48,
}
