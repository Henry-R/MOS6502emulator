use crate::computer_state::memory::Memory;
use crate::computer_state::registers::*;
use crate::computer_state::status_register::StatusRegister;

pub(crate) mod status_register;
pub(crate) mod operations;
mod registers;
mod memory;

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

    // DISPLAY FUNCTIONS
    pub fn get_state_str(&self) -> String {
        // Registers
        let program_counter = self.mem.pc.get();
        //let stack_pointer = stringify!(self.mem.pc.get());
        let accumulator = self.acc.get();
        let index_x = self.mem.x;
        let index_y = self.mem.y;

        // Processor Status
        let carry_flag =     self.sta.flag_value(StatusRegister::C);
        let zero_flag =      self.sta.flag_value(StatusRegister::Z);
        let interrupt_flag = self.sta.flag_value(StatusRegister::I);
        let decimal_flag =   self.sta.flag_value(StatusRegister::D);
        let break_flag =     self.sta.flag_value(StatusRegister::B);
        let overflow_flag =  self.sta.flag_value(StatusRegister::V);
        let negative_flag =  self.sta.flag_value(StatusRegister::N);

        let pc = self.mem.pc.get();
        let opcode = self.mem.fetch_byte_from_addr(pc);
        let instruction = operations::decode_operation_name(opcode);

        format!(
            "CURRENT INSTRUCTION:
                instruction: {instruction}
                opcode:      {opcode}
            REGISTERS:
            Program Counter: {program_counter}
            Accumulator:     {accumulator}
            Index X:         {index_x}
            Index Y:         {index_y}
            Status Flags:
                Carry:       {carry_flag}
                Zero:        {zero_flag}
                Interrupt:   {interrupt_flag}
                Decimal:     {decimal_flag}
                Break:       {break_flag}
                Overflow:    {overflow_flag}
                Negative:    {negative_flag}")
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
    pub fn set_up_state(&mut self, bytes: &[u8]) {
        let old_pc = self.mem.pc.get();
        for byte in bytes {
            self.mem.insert_at_pc(*byte);
        }
        self.mem.pc.set(old_pc);
    }


    // REGISTER INSTRUCTIONS
    pub const fn get_carry(&self) -> u8 {
        if self.sta.contains(StatusRegister::C) { 1 } else { 0 }
    }
}
