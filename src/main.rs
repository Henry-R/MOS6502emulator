
struct ComputerState {
    pub A: i8,      // Accumulator
    pub P: i8,      // Status register
    pub PC: i16,    // Program counter
    pub S: i8,      // Stack pointer
    pub X: i8,      // Index register
    pub Y: i8      // Index register
}

// MOS6502 operation. Mutates the computer's state
type MosOp = fn(&mut ComputerState);

fn decode(opcode: u8) {
    // Defines the type of instruction
    let opcode = (opcode | 0b11100000) >> 5;
    // Defines the addressing mode (intermediate, zero page, ...)
    let address_mode = (opcode | 0b00011100) >> 2;
    // Specifies the exact version of the instruction
    let opcode_mode = opcode | 0b00000011;


}

fn main() {
    let instruction_table: [MosOp; 8];

    println!("Hello, world!");
}
