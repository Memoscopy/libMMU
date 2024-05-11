use std::thread::sleep;

// NOTE: most of the types here are wrong currently (for testing purposes)
struct CPU {
    opcode_to_mmu_regs: String,
    opcode_to_gregs: String,
    architecture: String,
    bits: String,
    endianness: String,
    processor_features: String,
    register_values: String,
}

// temporary struct for testing
struct Config {
    architecture: String,
    bits: String,
    endianness: String,
    processor_features: String,
    register_values: String,
}

impl CPU {
    fn new(mut self, config: Config) {
        self.architecture = config.architecture;
        self.bits = config.bits;
        self.endianness = config.endianness;
        self.processor_features = config.processor_features;
        self.register_values = config.register_values;
    }

    fn extract_bits_little(entry: u32, pos: u32, n: u32) -> u32 {
        (entry >> pos) & ((1 << n) - 1)
    }

    fn extract_bits_big(entry: u32, pos: u32, n: u32) -> u32 {
        (entry >> (32 - pos - n)) & ((1 << n) - 1)
    }

    fn parse_opcode(self, buff: String, page_addr: String, offset: u32) {
        todo!()
    }

    fn parse_opcode_parallel(self, addresses: Vec<String>, frame_size: u32, pidx: u32) {
        todo!()
    }

    fn find_registers_values_dataflow(self, opcodes: String, zero_registers: Vec<u32>) {
        todo!()
    }
}
