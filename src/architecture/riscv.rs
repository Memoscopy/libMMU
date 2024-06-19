use super::generic::{CPURegister as CPURegisterTrait, PageTableEntry as PageTableEntryTrait};

use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Represents a RISC-V CPU register associated with a value.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, Hash, Eq, PartialEq)]
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
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct PageTableEntry {
    pub address: u64,
    pub flags: u64,
}

impl PageTableEntry {
    pub fn new(address: u64, flags: u64) -> Self {
        Self { address, flags }
    }

    pub fn is_supervisor(&self) -> bool {
        todo!()
    }
}

impl PageTableEntryTrait for PageTableEntry {
    type Address = u64;
    type Flags = u64;

    // FIXME: Implement the following methods
    fn is_dirty(&self) -> bool {
        todo!()
    }

    fn is_accessed(&self) -> bool {
        todo!()
    }

    fn is_global(&self) -> bool {
        todo!()
    }

    fn is_readable(&self) -> bool {
        todo!()
    }

    fn is_writable(&self) -> bool {
        todo!()
    }

    fn is_executable(&self) -> bool {
        todo!()
    }
}

/// Enumerates RISC-V MMU modes.
/// The MMU modes are used to determine the number of bits used for virtual and physical addresses.
/// The modes are named after the number of bits used for the virtual address space.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, Eq, PartialEq, Default)]
pub enum MMUMode {
    #[default]
    SV32,
    SV39,
    SV48,
}

/// Represents a RISC-V CPU.
#[derive(Debug, Clone, Serialize, Deserialize, Default, Hash, Eq, PartialEq)]
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
#[derive(Debug, Clone, Serialize, Deserialize, Default, Hash, Eq, PartialEq)]
pub struct MMU {
    pub mode: MMUMode,
}

impl MMU {
    pub fn new(mode: MMUMode) -> Self {
        Self { mode }
    }
}
