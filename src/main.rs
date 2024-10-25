
struct ComputerState {
    // REGISTERS
    // (A) Accumulator
    pub acc: i8,
    // (P) Status register
    pub sta: i8,
    // (PC) Program counter
    pub pc: i16,
    // (S) Stack pointer
    pub stk: i8,
    // (X) Index register
    pub x: i8,
    // (Y) Index register
    pub y: i8
}

// MOS6502 operation. Mutates the computer's state
type MosOp = fn(&mut ComputerState);

fn decode(opcode: u8) -> MosOp{
    let instruction_table: [[MosOp; 16]; 16];

    // Defines the type of instruction
    let opcode = (opcode | 0b11100000) >> 5;
    // Defines the addressing mode (intermediate, zero page, ...)
    let address_mode = (opcode | 0b00011100) >> 2;
    // Specifies the exact version of the instruction
    let opcode_mode = opcode | 0b00000011;

    MosOp {}
}

fn main() {


    println!("Hello, world!");
}
