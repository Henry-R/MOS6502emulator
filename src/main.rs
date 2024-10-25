


struct Emulator {
    // Registers
    A: i8,      // Accumulator
    P: i8,      // Status register
    PC: i16,    // Program counter
    S: i8,      // Stack pointer
    X: i8,      // Index register
    Y: i8      // Index register
}

impl Emulator {
    fn decode(opcode: u8) {
        // Defines the type of instruction
        let opcode = (opcode | 0b11100000) >> 5;
        // Defines the addressing mode (intermediate, zero page, ...)
        let address_mode = (opcode | 0b00011100) >> 2;
        // Specifies the exact version of the instruction
        let opcode_mode = opcode | 0b00000011;


    }
}

fn main() {
    println!("Hello, world!");
}
