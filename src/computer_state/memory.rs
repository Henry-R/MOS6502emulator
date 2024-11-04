use crate::computer_state::registers::ProgramCounter;

const MEMORY_SIZE: usize = 0x10000;
pub struct Memory {
    pub pc: ProgramCounter,
    pub x: usize,
    pub y: usize,

    mem: [u8; MEMORY_SIZE],
}
impl Memory {
    pub(crate) fn new() -> Memory {
        Memory {
            x: 0,
            y: 0,
            pc: ProgramCounter::new(0),
            mem: [0; MEMORY_SIZE]
        }
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

    /// Inserts the given value at the position pointed to by the PC; increments the PC
    pub fn insert_at_pc(&mut self, value: u8) {
        self.set_byte_at_addr(self.pc.get(), value);
        self.pc.add_unsigned(1);
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
    pub fn fetch_next_byte(&mut self) -> u8 {
        let result = self.fetch_byte_from_addr(self.pc.get());
        self.pc.add_unsigned(1);
        result
    }

    /// Fetches the nibble at the PC, and increments the PC by 2
    pub fn fetch_next_nibble(&mut self) -> u16 {
        let result = self.fetch_nibble_from_addr(self.pc.get());
        self.pc.add_unsigned(2);
        result
    }

    /// Fetches the 8-bit address at the PC, and increments the PC by 1
    pub fn fetch_next_zp_addr(&mut self) -> usize {
        let result = self.fetch_zp_addr_from_addr(self.pc.get());
        self.pc.add_unsigned(1);
        result
    }

    /// Fetches the 16-bit address at the PC, and increments the PC by 2
    pub fn fetch_next_ab_addr(&mut self) -> usize {
        let result = self.fetch_ab_addr_from_addr(self.pc.get());
        self.pc.add_unsigned(2);
        result
    }

    /// Fetches the operand as a zero-page address
    pub fn fetch_zero_page_address(&mut self) -> usize {
        self.fetch_next_byte() as usize
    }

    /// Fetches the operand as a zero_page address and adds the X index to that address
    /// If this addition overflows, it will wrap around
    pub fn fetch_zero_page_x_address(&mut self) -> usize {
        self.fetch_zero_page_address().wrapping_add(self.x)
    }

    /// Fetches the operand as a zero_page address and adds the Y index to that address
    /// If this addition overflows, it will wrap around
    pub fn fetch_zero_page_y_address(&mut self) -> usize {
        self.fetch_zero_page_address().wrapping_add(self.y)
    }

    /// Fetches the operand as an address of an absolute address mode instruction
    pub fn fetch_absolute_address(&mut self) -> usize {
        self.fetch_next_ab_addr()
    }

    /// Fetches the operand as an absolute address and adds the X index to that address
    /// If this addition overflows, it will wrap around
    pub fn fetch_absolute_x_address(&mut self) -> usize {
        self.fetch_absolute_address() + self.x
    }

    /// Fetches the operand as an absolute address and adds the Y index to that address
    /// If this addition overflows, it will wrap around
    pub fn fetch_absolute_y_address(&mut self) -> usize {
        self.fetch_absolute_address() + self.y
    }

    /// TODO finish this documentation when I have more sleep
    pub fn fetch_indirect_x_address(&mut self) -> usize {
        let indirect_addr = self.fetch_zero_page_x_address();
        self.fetch_ab_addr_from_addr(indirect_addr)
    }

    pub fn fetch_indirect_y_address(&mut self) -> usize {
        let indirect_addr = self.fetch_next_zp_addr();
        let y = self.y;

        self.fetch_ab_addr_from_addr(indirect_addr) + y
    }

    /// Moves the PC up by one and fetches that constant from memory
    /// Wrapper around fetch_next_byte to make its use clearer
    pub fn fetch_intermediate(&mut self) -> u8 {
        self.fetch_next_byte()
    }

    /// Fetches the byte of memory located at the zero-page address
    pub fn fetch_zero_page(&mut self) -> u8 {
        self.mem[self.fetch_zero_page_address()]
    }

    /// Fetches the byte of memory located at the zero-page address and adds the X index register to it
    /// The result of this addition wraps
    pub fn fetch_zero_page_x(&mut self) -> u8 {
        self.mem[self.fetch_zero_page_x_address()]
    }

    /// Fetches the byte of memory located at the zero-page address and adds the Y index register to it
    /// The result of this addition wraps
    /// Exactly the same as fetch_zero_page_x(), but for the Y index register. Used by fewer operations
    pub fn fetch_zero_page_y(&mut self) -> u8 {
        self.mem[self.fetch_zero_page_y_address()]
    }

    pub fn fetch_relative(&mut self) -> i8 {
        let raw_offset = self.fetch_next_byte();

        (if raw_offset & (1 << 0) != 0 { 1 << 0 } else { 0 }) |
        (if raw_offset & (1 << 1) != 0 { 1 << 1 } else { 0 }) |
        (if raw_offset & (1 << 2) != 0 { 1 << 2 } else { 0 }) |
        (if raw_offset & (1 << 3) != 0 { 1 << 3 } else { 0 }) |
        (if raw_offset & (1 << 4) != 0 { 1 << 4 } else { 0 }) |
        (if raw_offset & (1 << 5) != 0 { 1 << 5 } else { 0 }) |
        (if raw_offset & (1 << 6) != 0 { 1 << 6 } else { 0 }) |
        (if raw_offset & (1 << 7) != 0 { 1 << 7 } else { 0 })
    }

    /// Fetches the memory at the target location of an absolute address mode instruction
    pub fn fetch_absolute(&mut self) -> u8 { self.mem[self.fetch_absolute_address()] }

    /// Fetches the X index register to the absolute address, then fetches the memory from that
    /// address with the offset
    pub fn fetch_absolute_x(&mut self) -> u8 { self.mem[self.fetch_absolute_x_address()] }

    /// Fetches the Y index register to the absolute address, then fetches the memory from that
    /// address with the offset
    pub fn fetch_absolute_y(&mut self) -> u8 {
        self.mem[self.fetch_absolute_y_address()]
    }

    /// Fetches the memory held by the address given by the absolute address plus the X index
    pub fn fetch_indirect_x(&mut self) -> u8 { self.mem[self.fetch_indirect_x_address()] }

    /// Fetches the memory held at the address pointed to by the given address plus the Y index
    pub fn fetch_indirect_y(&mut self) -> u8 { self.mem[self.fetch_indirect_y_address()] }
}
