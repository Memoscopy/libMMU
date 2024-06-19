<p align="center" width="100%">
    <img src="assets/libmmu.webp" alt="libMMU logo" style="width: 10em; height: auto;">
</p>
<h1 align="center">libMMU</h1>
<p align="center">libMMU is a Rust crate that aims to simplify the process of rebuilding virtual address spaces from a memory dump in an OS-agnostic way.<p>
<div align="center">
    <img alt="Open issues"     src="https://img.shields.io/github/issues/Memoscopy/libMMU?style=for-the-badge&color=%23973B21&labelColor=%230C1510">
    <img alt="Commit activity" src="https://img.shields.io/github/commit-activity/w/Memoscopy/libMMU?style=for-the-badge&color=%23973B21&labelColor=%230C1510">
    <img alt="License"         src="https://img.shields.io/github/license/Memoscopy/libMMU?style=for-the-badge&color=%23973B21&labelColor=%230C1510">
</div>

## Usage

todo

## Installation

todo

## Documentation

todo

## Roadmap

**v0.1.0**

- [ ] Basic generic structures (CPU, MMU modes, Page Table Entries, ...)
- [ ] Basic generic methods (extract bit(s), check flags on PDE/PTE/Pages )
- [ ] Basic architectures constraints, e.g: cannot create a IA32 configuration with 64 bits
- [ ] Loading base configuration from differents formats : TOML, YAML or from an in-line Rust [builder pattern](https://rust-unofficial.github.io/patterns/patterns/creational/builder.html)

**v0.2.0**

- [ ] Implement RISC-V architecture and its associated MMU modes, basic invariants
- [ ] Implement structural signatures
- [ ] Implement validation rules

**v0.3.0**

- [ ] Refactor the code to be asynchronous and threaded

**Planned features**

- Allow users to write their own validation rules with either a custom made grammar DSL or traits implementations
- Add support for Binary Code Analysis with `miasm`

```rust
use anyhow::Result;
use libmmu::architectures::{ RiscV, RiscVMMUMode };
use libmmu::utils::{ MemorySpace, SpaceType, MachineConfig };

fn main() -> Result<()> {
    let dumpfile = ...;
    let outfolder = ...;

    let memspaces = MemorySpace::new()
        .add(SpaceType::RAM, 0x0000000080000000, 0x000000017fffffff)
        .add(SpaceType::ROM, 0x0000000000000000, 0x0000000000011fff);

    let machine = Machine::new(
        MachineType::RiscV(MMUMode:SV32),
        memspaces,
        dumpfile,
        outfolder
    )?;

    machine.resolve_spaces()?;
}
```

## Credits

A huge part of this work is based on the thesis [In the Land of MMUs: Multiarchitecture OS-Agnostic Virtual Memory Forensics](https://www.s3.eurecom.fr/docs/tops22_oliveri.pdf) by Andrea Oliveri and Davide Balzarotti and their POC [mmushell](https://github.com/eurecom-s3/mmushell).
