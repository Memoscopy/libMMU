use super::generic::{CPURegister, PageTableEntry};

use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Represents a RISC-V CPU register associated with a value.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, Hash, Eq, PartialEq)]
pub struct RiscVCPURegister {
    pub value: u64,
}

impl CPURegister for RiscVCPURegister {
    type Value = u64;

    fn is_valid(&self) -> Result<u64> {
        // FIXME: Implement validation logic
        Ok(self.value)
    }
}

impl RiscVCPURegister {
    pub fn new(value: u64) -> Self {
        Self { value }
    }
}

/// Represents a RISC-V page table entry.
/// It holds the mapping between a virtual address of a page and the address of a physical frame.
/// There is also auxiliary information about the page such as a present bit, a dirty or modified bit,
/// address space or process ID information, amongst others.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct RiscVPageTableEntry {
    pub address: u64,
    pub flags: u64,
}

impl RiscVPageTableEntry {
    pub fn new(address: u64, flags: u64) -> Self {
        Self { address, flags }
    }

    pub fn is_supervisor(&self) -> bool {
        self.flags & 0x1 == 0
    }
}

impl PageTableEntry for RiscVPageTableEntry {
    type Address = u64;
    type Flags = u64;

    // FIXME: Implement the following methods
    fn is_dirty(&self) -> bool {
        true
    }

    fn is_accessed(&self) -> bool {
        true
    }

    fn is_global(&self) -> bool {
        true
    }

    fn is_readable(&self) -> bool {
        true
    }

    fn is_writable(&self) -> bool {
        true
    }

    fn is_executable(&self) -> bool {
        true
    }
}

/// Enumerates RISC-V MMU modes.
/// The MMU modes are used to determine the number of bits used for virtual and physical addresses.
/// The modes are named after the number of bits used for the virtual address space.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub enum RiscVMMUMode {
    SV32,
    SV39,
    SV48,
}
