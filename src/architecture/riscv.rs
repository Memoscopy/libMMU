use super::generic::{CPURegister as CPURegisterTrait, PageTableEntry as PageTableEntryTrait};

use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Extracts bits from a 64-bit entry in little-endian order.
fn extract_bits(entry: u64, pos: u64, n: u64) -> u64 {
    (entry >> pos) & ((1 << n) - 1)
}

/// Represents a RISC-V CPU register associated with a value.
#[derive(Debug, Clone, Serialize, Deserialize, Default, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct CPURegister {
    pub value: u64,
}

impl CPURegisterTrait for CPURegister {
    type Value = u64;

    fn is_valid(&self) -> Result<u64> {
        todo!()
    }
}

impl CPURegister {
    pub fn new(value: u64) -> Self {
        Self { value }
    }
}

/// Represents a RISC-V page table entry.
/// It holds the mapping between a virtual address of a page and the address of a physical frame.
/// There is also auxiliary information about the page such as a present bit, a dirty or modified bit,
/// address space or process ID information, amongst others.
#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct PageTableEntry {
    pub address: u64,
    pub flags: u64,
}



impl PageTableEntry {
    pub fn new(address: u64, flags: u64) -> Self {
        Self { address, flags }
    }

    pub fn is_supervisor(&self) -> bool {
        extract_bits(self.flags, 4, 1) == 0
    }
    
    pub fn extract_addr(entry: u64) -> u64 {
        extract_bits(entry, 10, 21) << 12
    }
}

impl PageTableEntryTrait for PageTableEntry {
    type Address = u64;
    type Flags = u64;

    fn is_dirty(&self) -> bool {
        extract_bits(self.flags, 7, 1) != 0
    }

    fn is_accessed(&self) -> bool {
        extract_bits(self.flags, 6, 1) != 0
    }

    fn is_global(&self) -> bool {
        extract_bits(self.flags, 5, 1) != 0 
    }

    fn is_readable(&self) -> bool {
        extract_bits(self.flags, 1, 1) != 0
    }

    fn is_writable(&self) -> bool {
        extract_bits(self.flags, 2, 1) != 0
    }

    fn is_executable(&self) -> bool {
        extract_bits(self.flags, 3, 1) != 0
    }
}

/// Enumerates RISC-V MMU modes.
/// The MMU modes are used to determine the number of bits used for virtual and physical addresses.
/// The modes are named after the number of bits used for the virtual address space.
#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq, Ord, PartialOrd, Default)]
pub enum MMUMode {
    #[default]
    SV32,
    SV39,
    SV48,
}

/// Represents a RISC-V CPU.
#[derive(Debug, Clone, Serialize, Deserialize, Default, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct CPU {
    pub registers: Vec<CPURegister>,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            registers: Vec::new(),
        }
    }
}

/// Represents a RISC-V MMU.
#[derive(Debug, Clone, Serialize, Deserialize, Default, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct MMU {
    pub mode: MMUMode,
}

impl MMU {
    pub fn new(mode: MMUMode) -> Self {
        Self { mode }
    }
}
