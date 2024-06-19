#[cfg(test)]
mod configuration_tests {
    use anyhow::Result;
    use libmmu::architecture::generic::{
        Machine, MachineType, MemoryRegion, MemoryRegionType, MemorySpace,
    };
    use libmmu::architecture::riscv::{RiscVCPU, RiscVMMU, RiscVMMUMode};
    use libmmu::utils::TryFromPath;
    use std::path::{Path, PathBuf};

    #[test]
    fn create_riscv_configuration_from_code() -> Result<()> {
        let machine_type = MachineType::RiscV;
        let mmu = RiscVMMU::new(RiscVMMUMode::SV32);
        let cpu = RiscVCPU::new();
        let dumpfile = PathBuf::from("tests/fixtures/riscv-sv32.dump");

        let memspaces = MemorySpace::new()
            .add(MemoryRegion {
                _type: MemoryRegionType::RAM,
                start_address: 0,
                end_address: 1,
            })?
            .add(MemoryRegion {
                _type: MemoryRegionType::ROM,
                start_address: 2,
                end_address: 3,
            })?
            .to_owned();

        let machine =
            Machine::<RiscVCPU, RiscVMMU>::new(machine_type, cpu, mmu, memspaces, dumpfile);

        assert_eq!(machine._type, MachineType::RiscV);
        assert_eq!(machine.cpu.registers.len(), 0);
        assert_eq!(machine.mmu.mode, RiscVMMUMode::SV32);
        assert_eq!(machine.memory_regions.regions.len(), 2);
        assert_eq!(
            machine.dumpfile,
            PathBuf::from("tests/fixtures/riscv-sv32.dump")
        );

        Ok(())
    }

    // #[test]
    // fn create_riscv_configuration_from_toml() {
    //     let path = Path::new("tests/fixtures/riscv.toml");
    //     let machine = Machine::<RiscVCPU, RiscVMMU>::from_path(path);

    //     println!("{:?}", machine);

    //     assert!(machine.is_ok());
    // }
}
