use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{hash, path::PathBuf};

// MemoryRegion
// CPU
// CPURegister
// PageTableEntry
// PageTable
// RadixTree
// VirtualAddressSpace
// PhysicalAddressSpace
// MMU
// Machine

/// Enumerates types of memory regions.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MemoryRegionType {
    RAM,
    ROM,
}

impl Default for MemoryRegionType {
    fn default() -> Self {
        MemoryRegionType::RAM
    }
}

/// Represents a memory region with a start and end address.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct MemoryRegion {
    pub _type: MemoryRegionType,
    pub start_address: u64,
    pub end_address: u64,
}

impl MemoryRegion {
    pub fn new(_type: MemoryRegionType, start_address: u64, end_address: u64) -> Result<Self> {
        // Check that addresses are valid memory addresses
        if start_address >= end_address {
            return Err(anyhow::anyhow!(
                "Invalid memory region, start address is greater than or equal to end address"
            ));
        }

        if start_address >= u64::MAX || end_address >= u64::MAX {
            return Err(anyhow::anyhow!(
                "Invalid memory address, address is greater than u64::MAX"
            ));
        }

        Ok(Self {
            _type,
            start_address,
            end_address,
        })
    }

    /// Returns the size of the memory region.
    pub fn size(&self) -> u64 {
        self.end_address - self.start_address
    }

    /// Returns true if the memory region contains the address.
    /// A memory region contains an address if the address is greater than or equal to the start address and less than the end address.
    pub fn contains(&self, address: u64) -> bool {
        self.start_address <= address && address < self.end_address
    }

    /// Returns true if the two memory regions are overlapping.
    /// Two memory regions are overlapping if the start address of one region is less than the end address of the other region.
    pub fn is_overlapping(&self, other: &MemoryRegion) -> bool {
        self.contains(other.start_address) || other.contains(self.start_address)
    }

    /// Returns true if the two memory regions are adjacent.
    /// Two memory regions are adjacent if the end address of one region is equal to the start address of the other region.
    pub fn is_adjacent(&self, other: &MemoryRegion) -> bool {
        self.start_address == other.end_address || other.start_address == self.end_address
    }
}

/// Represents a memory space with regions.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MemorySpace {
    pub regions: Vec<MemoryRegion>,
}

impl MemorySpace {
    pub fn new(regions: Vec<MemoryRegion>) -> Self {
        Self { regions }
    }

    pub fn add(&mut self, region: MemoryRegion) -> Result<&mut Self> {
        // Check if the memory region is overlapping with another region
        if self.regions.iter().any(|r| r.is_overlapping(&region)) {
            return Err(anyhow::anyhow!(
                "Memory region is overlapping with another region"
            ));
        }

        self.regions.push(region);
        Ok(self)
    }
}

/// Represents a CPU register with a value.
/// Depending on the architecture, the *validity* changes.
pub trait CPURegister {
    type Value: hash::Hash + Eq + Copy + Default;

    fn is_valid(&self) -> Result<Self::Value>;
}

/// Represents a page table entry with an address and flags.
/// It holds the mapping between a virtual address of a page and the address of a physical frame.
/// There is also auxiliary information about the page such as a present bit, a dirty or modified bit,
/// address space or process ID information, amongst others.
pub trait PageTableEntry {
    type Address: hash::Hash + Eq + Copy + Default;
    type Flags: hash::Hash + Eq + Copy + Default;

    fn is_dirty(&self) -> bool;
    fn is_accessed(&self) -> bool;
    fn is_global(&self) -> bool;
    fn is_readable(&self) -> bool;
    fn is_writable(&self) -> bool;
    fn is_executable(&self) -> bool;
}

/// Represents a page table with entries.
/// It is a data structure used in a virtual memory system to manage the mapping between virtual addresses and physical addresses.
/// It is used to translate virtual addresses to physical addresses and to manage the memory permissions of the pages.
/// It is also used to store additional information about the pages, such as the status of the page, the address space or process ID, amongst others.
pub trait PageTable {
    type Entries: hash::Hash + Eq + Copy + Default + PageTableEntry;

    // fn apply_on_entries(function: FnMut(PageTableEntry) -> Vec<?> ) -> ? // FIXME: to be defined, but is it necessary?
}

pub trait CPU {}

pub trait MMU {}

/// Enumerates types of supported machines.
/// This enum is used to specify the type of machine that is being parsed.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MachineType {
    RiscV,
}

/// Represents a machine with a type, MMU, CPU, memory regions, and an associated dump file.
/// It is used to store the machine's configuration, memory regions, and the dump file that is being used.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Machine<T: CPU, U: MMU> {
    pub _type: MachineType,
    pub mmu: T,
    pub cpu: U,
    pub memory_regions: Vec<MemoryRegion>,
    pub dumpfile: PathBuf,
}

impl<T: CPU, U: MMU> Machine<T, U> {
    pub fn new(
        _type: MachineType,
        mmu: T,
        cpu: U,
        memory_regions: Vec<MemoryRegion>,
        dumpfile: PathBuf,
    ) -> Self {
        Self {
            _type,
            mmu,
            cpu,
            memory_regions,
            dumpfile,
        }
    }
}
