use crate::computer_state::operations::opcode_from_operation;
use crate::computer_state::registers::*;
use crate::computer_state::status_register::StatusRegister;

pub(crate) mod status_register;
pub(crate) mod operations;
mod registers;

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
    mem: [u8; MEMORY_SIZE],

    pub acc: Accumulator,
    pub sta: StatusRegister,
    //pub stk: Stack<'a>,
    pub pc: ProgramCounter,
    pub x: u8,
    pub y: u8,
}

impl ComputerState {
    pub fn new() -> ComputerState {
        let raw_mem = [0u8; MEMORY_SIZE];

        ComputerState {
            mem: raw_mem,
            acc: Accumulator::new(0),
            sta: StatusRegister::new(),
            //stk: Stack::new(&mut raw_mem),
            pc: ProgramCounter::new(0),
            x: 0,
            y: 0
        }
    }

    // EXECUTION
    /// Executes the instruction at the program counter
    pub fn execute_next(&mut self) {
        // Fetch
        let opcode = self.fetch_next_byte();
        // Decode
        let operation = operations::decode(opcode);
        // Execute instruction
        operation(self);
    }

    // MEMORY ACCESS
    /// Inserts the given value at the position pointed to by the PC; increments the PC
    pub fn insert_at_pc(&mut self, value: u8) {
        self.set_byte_at_addr(self.pc.get(), value);
        self.pc.add_unsigned(1);
    }

    /// Inserts the given instruction at the position pointed to by the PC; increments the PC
    pub fn insert_operation_at_pc(&mut self, op: operations::MosOp) {
        let opcode = opcode_from_operation(op);
        self.insert_at_pc(opcode);
    }

    pub fn set_up_state(&mut self, bytes: Vec<u8>) {
        let old_pc = self.pc.get();
        for byte in bytes {
            self.insert_at_pc(byte);
        }
        self.pc.set(old_pc);
    }



    // REGISTER INSTRUCTIONS
    pub const fn get_carry(&self) -> u8 {
        if self.sta.contains(StatusRegister::C) { 1 } else { 0 }
    }

    // SET INSTRUCTIONS
    pub fn set_byte_at_addr(&mut self, addr: usize, value: u8) {
        self.mem[addr] = value;
    }

    pub fn set_nibble_at_addr(&mut self, addr: usize, value: u16) {
        let hi_byte = ((value & 0xFF00) >> 8) as u8;
        let lo_byte =  (value & 0x00FF) as u8;
        self.set_byte_at_addr(addr, lo_byte);
        self.set_byte_at_addr(addr + 1, hi_byte);
    }

    // FETCH INSTRUCTIONS
    // These instructions help the emulator fetch memory according to addressing modes
    /// Returns the byte of data at the given address
    pub fn fetch_byte_from_addr(&self, addr: usize) -> u8 {
        self.mem[addr]
    }

    /// Returns 16-bits of data at the given address in little endian byte-order
    pub fn fetch_nibble_from_addr(&self, addr: usize) -> u16 {
        let lo_byte = u16::from(self.fetch_byte_from_addr(addr));
        let hi_byte = u16::from(self.fetch_byte_from_addr(addr + 1));
        (hi_byte << 8) + lo_byte
    }

    /// Returns the 8-bit address at the given address
    pub fn fetch_zp_addr_from_addr(&self, addr: usize) -> usize {
        usize::from(self.fetch_byte_from_addr(addr))
    }

    /// Returns the 16-bit address the given address in little endian byte-order
    pub fn fetch_ab_addr_from_addr(&self, addr: usize) -> usize {
        usize::from(self.fetch_nibble_from_addr(addr))
    }

    /// Fetches the byte at the PC, and increments the PC by 1
    fn fetch_next_byte(&mut self) -> u8 {
        let result = self.fetch_byte_from_addr(self.pc.get());
        self.pc.add_unsigned(1);
        result
    }

    /// Fetches the nibble at the PC, and increments the PC by 2
    fn fetch_next_nibble(&mut self) -> u16 {
        let result = self.fetch_nibble_from_addr(self.pc.get());
        self.pc.add_unsigned(2);
        result
    }

    /// Fetches the 8-bit address at the PC, and increments the PC by 1
    fn fetch_next_zp_addr(&mut self) -> usize {
        let result = self.fetch_zp_addr_from_addr(self.pc.get());
        self.pc.add_unsigned(1);
        result
    }

    /// Fetches the 16-bit address at the PC, and increments the PC by 2
    fn fetch_next_ab_addr(&mut self) -> usize {
        let result = self.fetch_ab_addr_from_addr(self.pc.get());
        self.pc.add_unsigned(2);
        result
    }

    /// Fetches the operand as a zero-page address
    fn fetch_zero_page_address(&mut self) -> usize {
        self.fetch_next_byte() as usize
    }

    /// Fetches the operand as a zero_page address and adds the X index to that address
    /// If this addition overflows, it will wrap around
    fn fetch_zero_page_x_address(&mut self) -> usize {
        self.fetch_zero_page_address().wrapping_add(usize::from(self.x))
    }

    /// Fetches the operand as a zero_page address and adds the Y index to that address
    /// If this addition overflows, it will wrap around
    fn fetch_zero_page_y_address(&mut self) -> usize {
        self.fetch_zero_page_address().wrapping_add(usize::from(self.y))
    }

    /// Fetches the operand as an address of an absolute address mode instruction
    fn fetch_absolute_address(&mut self) -> usize {
        self.fetch_next_ab_addr()
    }

    /// Fetches the operand as an absolute address and adds the X index to that address
    /// If this addition overflows, it will wrap around
    fn fetch_absolute_x_address(&mut self) -> usize {
        self.fetch_absolute_address() + usize::from(self.x)
    }

    /// Fetches the operand as an absolute address and adds the Y index to that address
    /// If this addition overflows, it will wrap around
    fn fetch_absolute_y_address(&mut self) -> usize {
        self.fetch_absolute_address() + usize::from(self.y)
    }

    /// TODO finish this documentation when I have more sleep
    fn fetch_indirect_x_address(&mut self) -> usize {
        let indirect_addr = self.fetch_zero_page_x_address();
        self.fetch_ab_addr_from_addr(indirect_addr)
    }

    fn fetch_indirect_y_address(&mut self) -> usize {
        let indirect_addr = self.fetch_next_zp_addr();
        let y = usize::from(self.y);

        self.fetch_ab_addr_from_addr(indirect_addr) + y
    }

    /// Moves the PC up by one and fetches that constant from memory
    /// Wrapper around fetch_next_byte to make its use clearer
    fn fetch_intermediate(&mut self) -> u8 {
        self.fetch_next_byte()
    }

    /// Fetches the byte of memory located at the zero-page address
    fn fetch_zero_page(&mut self) -> u8 {
        self.mem[self.fetch_zero_page_address()]
    }

    /// Fetches the byte of memory located at the zero-page address and adds the X index register to it
    /// The result of this addition wraps
    fn fetch_zero_page_x(&mut self) -> u8 {
        self.mem[self.fetch_zero_page_x_address()]
    }

    /// Fetches the byte of memory located at the zero-page address and adds the Y index register to it
    /// The result of this addition wraps
    /// Exactly the same as fetch_zero_page_x(), but for the Y index register. Used by fewer operations
    fn fetch_zero_page_y(&mut self) -> u8 {
        self.mem[self.fetch_zero_page_y_address()]
    }

    fn fetch_relative(&mut self) -> i8 {
        let raw_offset = self.fetch_next_byte();
        ((0xFF ^ raw_offset) + 1) as i8
    }

    /// Fetches the memory at the target location of an absolute address mode instruction
    fn fetch_absolute(&mut self) -> u8 { self.mem[self.fetch_absolute_address()] }

    /// Fetches the X index register to the absolute address, then fetches the memory from that
    /// address with the offset
    fn fetch_absolute_x(&mut self) -> u8 { self.mem[self.fetch_absolute_x_address()] }

    /// Fetches the Y index register to the absolute address, then fetches the memory from that
    /// address with the offset
    fn fetch_absolute_y(&mut self) -> u8 {
        self.mem[self.fetch_absolute_y_address()]
    }

    /// Fetches the memory held by the address given by the absolute address plus the X index
    fn fetch_indirect_x(&mut self) -> u8 { self.mem[self.fetch_indirect_x_address()] }

    /// Fetches the memory held at the address pointed to by the given address plus the Y index
    fn fetch_indirect_y(&mut self) -> u8 { self.mem[self.fetch_indirect_y_address()] }
}
