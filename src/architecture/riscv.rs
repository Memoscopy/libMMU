use super::generic::{
    CPURegister as CPURegisterTrait, PageTable, PageTableEntry, PageTableEntryTrait,
};

use anyhow::Result;
use bitfield::bitfield;
use serde::{Deserialize, Serialize};

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

    #[allow(unused_variables)]
    fn is_mmu_equivalent_to(&self, other: &Self) -> bool {
        todo!()
    }
}

impl CPURegister {
    pub fn new(value: u64) -> Self {
        Self { value }
    }
}

bitfield! {
    /// Represents a RISC-V SV32 page table entry flags.
    /// The flags are used to determine the permissions of a page, or some other attributes.
    /// This is baed on the RISC-V Sv39 page table entry flags.
    /// All fields are read-only.
    #[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq, Ord, PartialOrd, Default)]
    pub struct PTE32Flags(u32);

    pub read, _: 1;
    pub write, _: 2;
    pub exec, _: 3;
    pub supervisor, _: 4;
    pub global, _: 5;
    pub accessed, _: 6;
    pub dirty, _: 7;
}

bitfield! {
    /// Represents a RISC-V SV39 and SV48 page table entry flags.
    /// The flags are used to determine the permissions of a page, or some other attributes.
    /// This is baed on the RISC-V Sv39 page table entry flags.
    /// All fields are read-only.
    #[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq, Ord, PartialOrd, Default)]
    pub struct PTE64Flags(u64);

    pub read, _: 1;
    pub write, _: 2;
    pub exec, _: 3;
    pub supervisor, _: 4;
    pub global, _: 5;
    pub accessed, _: 6;
    pub dirty, _: 7;
}

/// Represents a RISC-V SV32 page table entry size.
#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq, Ord, PartialOrd, Default)]
pub enum PTE32Size {
    /// 4 bytes
    #[default]
    PTP32,
    /// 4KB, 1024 * 4 bytes
    PTE4KB,
    /// 4MB, 1024 * 1024 * 4 bytes
    PTE4MB,
}

/// Represents a RISC-V SV39 and SV48 page table entry size.
#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq, Ord, PartialOrd, Default)]
pub enum PTE64Size {
    /// 8 bytes
    #[default]
    PTP64,
    /// 4KB, 512 * 8 bytes
    PTE4KB,
    /// 2MB, 256 * 8 bytes
    PTE2MB,
    /// 1GB, 128 * 8 bytes
    PTE1GB,
    /// 512GB, 64 * 8 bytes
    /// This is only available in Sv48
    PTE512GB,
}

impl PageTableEntry<u32, PTE32Flags, PTE32Size> {
    pub fn new(address: u32, flags: u32, size: PTE32Size) -> Self {
        let flags = PTE32Flags(flags);
        Self {
            address,
            flags,
            size,
        }
    }

    pub fn is_supervisor(&self) -> bool {
        self.flags.supervisor()
    }
}

impl PageTableEntryTrait for PageTableEntry<u32, PTE32Flags, PTE32Size> {
    type Address = u32;
    type Flags = PTE32Flags;
    type Size = PTE32Size;

    fn is_dirty(&self) -> bool {
        self.flags.dirty()
    }

    fn is_accessed(&self) -> bool {
        self.flags.accessed()
    }

    fn is_global(&self) -> bool {
        self.flags.global()
    }

    fn is_readable(&self) -> bool {
        self.flags.read()
    }

    fn is_writable(&self) -> bool {
        self.flags.write()
    }

    fn is_executable(&self) -> bool {
        self.flags.exec()
    }
}

impl PageTableEntry<u64, PTE64Flags, PTE64Size> {
    pub fn new(address: u64, flags: u64, size: PTE64Size) -> Self {
        let flags = PTE64Flags(flags);
        Self {
            address,
            flags,
            size,
        }
    }

    pub fn is_supervisor(&self) -> bool {
        self.flags.supervisor()
    }
}

impl PageTableEntryTrait for PageTableEntry<u64, PTE64Flags, PTE64Size> {
    type Address = u64;
    type Flags = PTE64Flags;
    type Size = PTE64Size;

    fn is_dirty(&self) -> bool {
        self.flags.dirty()
    }

    fn is_accessed(&self) -> bool {
        self.flags.accessed()
    }

    fn is_global(&self) -> bool {
        self.flags.global()
    }

    fn is_readable(&self) -> bool {
        self.flags.read()
    }

    fn is_writable(&self) -> bool {
        self.flags.write()
    }

    fn is_executable(&self) -> bool {
        self.flags.exec()
    }
}

/// Represents a RISC-V SV32 page table.
pub type PageTable32 = PageTable<u32, PTE32Flags, PTE32Size>;
/// Represents a RISC-V SV39 and SV48 page table.
pub type PageTable64 = PageTable<u64, PTE64Flags, PTE64Size>;

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

/// Represents a MMU that uses a radix tree.
pub struct MMURadixTree32 {
    pub nb_radix_levels: u8,
}
