use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::hash;

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
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
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
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default)]
pub struct MemoryRegion {
    pub _type: MemoryRegionType,
    pub start_address: u64,
    pub end_address: u64,
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

    // fn apply_on_entries(function: FnMut(PageTableEntry) -> Vec<?> ) -> ? // to be defined
}
