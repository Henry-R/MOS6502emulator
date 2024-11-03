use crate::computer_state::memory::Memory;
use crate::computer_state::operations::opcode_from_operation;
use crate::computer_state::registers::*;
use crate::computer_state::status_register::StatusRegister;

pub(crate) mod status_register;
pub(crate) mod operations;
mod registers;
mod memory;

// Important constants
pub const MEMORY_SIZE: usize = 2usize.pow(16);
// Important memory locations
pub const PAGE_SIZE: usize = 0xFF;
pub const ZERO_PAGE: usize = 0x0000;
pub const STACK_PAGE: usize = 0x0100;
pub const NON_MASKABLE_INTERRUPT_HANDLER: u16 = 0xFFFA;
pub const POWER_ON_RESET_LOCATION: u16 = 0xFFFC;
pub const INTERRUPT_REQUEST_HANDLER: u16 = 0xFFFE;

pub struct ComputerState {
    // MEMORY
    // Each page is 256 bytes
    // First page is reserved for the Zero-Page ($0000-$00FF)
    // Second page is reserved for system stack ($0100-$01FF)
    // Last 6 bytes are reserved for interrupts ($FFFA-$FFFF)
    pub mem: Memory,

    pub acc: Accumulator,
    pub sta: StatusRegister,
    // Stack can be a subset of memory
}

impl ComputerState {
    pub fn new() -> ComputerState {
        ComputerState {
            mem: Memory::new(),
            acc: Accumulator::new(0),
            sta: StatusRegister::new(),
        }
    }

    // REGISTERS
    pub fn get_x(&self) -> usize {
        self.mem.x
    }

    pub fn get_y(&self) -> usize {
        self.mem.y
    }

    pub fn set_x(&mut self, value: usize) {
        self.mem.x = value
    }

    pub fn set_y(&mut self, value: usize) {
        self.mem.y = value
    }


    // EXECUTION
    /// Executes the instruction at the program counter
    pub fn execute_next(&mut self) {
        // Fetch
        let opcode = self.mem.fetch_next_byte();
        // Decode
        let operation = operations::decode(opcode);
        // Execute instruction
        operation(self);
    }

    // MEMORY ACCESS
    /// Inserts the given value at the position pointed to by the PC; increments the PC
    pub fn insert_at_pc(&mut self, value: u8) {
        self.mem.set_byte_at_addr(self.mem.pc.get(), value);
        self.mem.pc.add_unsigned(1);
    }

    /// Inserts the given instruction at the position pointed to by the PC; increments the PC
    pub fn insert_operation_at_pc(&mut self, op: operations::MosOp) {
        let opcode = opcode_from_operation(op);
        self.insert_at_pc(opcode);
    }

    pub fn set_up_state(&mut self, bytes: Vec<u8>) {
        let old_pc = self.mem.pc.get();
        for byte in bytes {
            self.insert_at_pc(byte);
        }
        self.mem.pc.set(old_pc);
    }


    // REGISTER INSTRUCTIONS
    pub const fn get_carry(&self) -> u8 {
        if self.sta.contains(StatusRegister::C) { 1 } else { 0 }
    }
}
