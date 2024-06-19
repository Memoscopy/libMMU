#[cfg(test)]
mod configuration_tests {
    use std::path::PathBuf;

    use anyhow::Result;

    use libmmu::architecture::generic::{
        Machine, MachineType, MemoryRegion, MemoryRegionType, MemorySpace,
    };
    use libmmu::architecture::riscv::MMUMode;
    use libmmu::utils::TryFromPath;

    #[test]
    fn create_riscv_sv32_configuration_from_code() -> Result<()> {
        let machine_type = MachineType::RiscV(MMUMode::SV32);
        let dumpfile = PathBuf::from("tests/fixtures/riscv-sv32.dump");
        let outfolder = PathBuf::from("tests/output");

        let memspaces = MemorySpace::new()
            .add(MemoryRegion {
                region_type: MemoryRegionType::RAM,
                start_address: 0,
                end_address: 1,
            })?
            .add(MemoryRegion {
                region_type: MemoryRegionType::ROM,
                start_address: 2,
                end_address: 3,
            })?
            .to_owned();

        let machine = Machine {
            machine_type: machine_type,
            memory_regions: memspaces,
            dumpfile,
            outfolder,
        };

        assert_eq!(machine.machine_type, MachineType::RiscV(MMUMode::SV32));
        assert_eq!(machine.memory_regions.regions.len(), 2);
        assert_eq!(
            machine.memory_regions.regions[0].region_type,
            MemoryRegionType::RAM
        );
        assert_eq!(machine.memory_regions.regions[0].start_address, 0x12345678);
        assert_eq!(machine.memory_regions.regions[0].end_address, 0x87654321);
        assert_eq!(
            machine.memory_regions.regions[1].region_type,
            MemoryRegionType::ROM
        );
        assert_eq!(machine.memory_regions.regions[1].start_address, 0);
        assert_eq!(machine.memory_regions.regions[1].end_address, 73727);
        assert_eq!(
            machine.dumpfile,
            PathBuf::from("tests/fixtures/riscv-sv32.dump")
        );
        assert_eq!(machine.outfolder, PathBuf::from("tests/output"));

        Ok(())
    }

    #[test]
    fn create_riscv_sv32_configuration_from_toml() -> Result<()> {
        let path = PathBuf::from("tests/fixtures/riscv-sample.toml");
        let machine = Machine::from_path(path.as_path())?;

        assert_eq!(machine.machine_type, MachineType::RiscV(MMUMode::SV32));
        assert_eq!(machine.memory_regions.regions.len(), 2);
        assert_eq!(
            machine.memory_regions.regions[0].region_type,
            MemoryRegionType::RAM
        );
        assert_eq!(machine.memory_regions.regions[0].start_address, 0x12345678);
        assert_eq!(machine.memory_regions.regions[0].end_address, 0x87654321);
        assert_eq!(
            machine.memory_regions.regions[1].region_type,
            MemoryRegionType::ROM
        );
        assert_eq!(machine.memory_regions.regions[1].start_address, 0);
        assert_eq!(machine.memory_regions.regions[1].end_address, 73727);
        assert_eq!(
            machine.dumpfile,
            PathBuf::from("tests/fixtures/riscv-sv32.dump")
        );
        assert_eq!(machine.outfolder, PathBuf::from("tests/output"));

        Ok(())
    }
}
