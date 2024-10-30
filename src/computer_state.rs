
pub(crate) mod operations;

use bitflags::bitflags;

bitflags! {
    pub struct StatusFlags: u8 {
        // (n) Negative
        const n = 0b0000_0001;
        // (v) Overflow
        const v = 0b0000_0010;
        // (b) Break
        const b = 0b0000_0100;
        // (d) Decimal
        const d = 0b0000_1000;
        // (i) Interrupt disable
        const i = 0b0001_0000;
        // (z) Zero
        const z = 0b0010_0000;
        // (c) Carry
        const c = 0b0100_0000;
    }
}


pub struct Registers {
    // (A) Accumulator
    pub acc: i8,
    // (P) Status register
    pub sta: StatusFlags,
    // (PC) Program counter
    pub pc: u16,
    // (S) Stack pointer
    pub stk: u8,
    // (X) Index register
    pub x: u8,
    // (Y) Index register
    pub y: u8
}

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

    pub regs: Registers,
}

impl ComputerState {
    pub fn new() -> ComputerState {
        ComputerState {
            mem: [0u8; MEMORY_SIZE],
            regs: Registers {
                acc: 0,
                sta: StatusFlags::empty(),
                pc: 0,
                stk: 0xFF,  // Stack grows downwards, so initialise stack to top of memory
                x: 0,
                y: 0,
            },
        }
    }

    // FETCH INSTRUCTIONS
    // These instructions help the emulator fetch memory according to addressing modes
    /// Moves the PC up by one and fetches that constant from memory
    fn fetch_next_byte(&mut self) -> u8 {
        self.regs.pc += 1;
        self.mem[self.regs.pc as usize]
    }

    /// Fetches the operand as a zero-page address
    fn fetch_zero_page_address(&mut self) -> usize {
        self.fetch_next_byte() as usize
    }

    /// Fetches the operand as a zero_page address and adds the X index to that address
    /// If this addition overflows, it will wrap around
    fn fetch_zero_page_x_address(&mut self) -> usize {
        self.fetch_zero_page_address().wrapping_add(self.regs.x as usize)
    }

    /// Fetches the operand as a zero_page address and adds the Y index to that address
    /// If this addition overflows, it will wrap around
    fn fetch_zero_page_y_address(&mut self) -> usize {
        self.fetch_zero_page_address().wrapping_add(self.regs.y as usize)
    }

    /// Fetches the operand as an address of an absolute address mode instruction
    fn fetch_absolute_address(&mut self) -> usize {
        let lo_byte = self.fetch_next_byte();
        let hi_byte = self.fetch_next_byte();
        lo_byte as usize + (hi_byte as usize >> 8)
    }

    /// Fetches the operand as an absolute address and adds the X index to that address
    /// If this addition overflows, it will wrap around
    fn fetch_absolute_address_x(&mut self) -> usize {
        self.fetch_absolute_address() + self.regs.x as usize
    }

    /// Fetches the operand as an absolute address and adds the Y index to that address
    /// If this addition overflows, it will wrap around
    fn fetch_absolute_address_y(&mut self) -> usize {
        self.fetch_absolute_address() + self.regs.y as usize
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

    /// Fetches the memory at the target location of an absolute address mode instruction
    fn fetch_absolute(&mut self) -> u8 {
        self.mem[self.fetch_absolute_address()]
    }

    /// Fetches the X index register to the absolute address, then fetches the memory from that
    /// address with the offset
    fn fetch_absolute_x(&mut self) -> u8 {
        self.mem[self.fetch_absolute_address_x()]
    }

    /// Fetches the Y index register to the absolute address, then fetches the memory from that
    /// address with the offset
    fn fetch_absolute_y(&mut self) -> u8 {
        self.mem[self.fetch_absolute_address_y()]
    }

    /// Fetches the memory held by the address given by the absolute address plus the X index
    fn fetch_indexed_indirect(&mut self) -> u8 {
        self.mem[self.fetch_absolute_x() as usize]
    }

    /// Fetches the memory held at the address pointed to by the given address plus the Y index
    fn fetch_indirect_indexed(&mut self) -> u8 {
        // Get zero-page index of 16-bit address
        let zp_index = self.fetch_next_byte();
        let lo_byte = self.mem[zp_index as usize];
        let hi_byte = self.mem[(zp_index + 1) as usize];
        // Add Y index offset
        let indirect_addr = lo_byte as u16 + (hi_byte as u16 >> 8);
        self.mem[(indirect_addr + self.regs.y as u16) as usize]
    }


    // STACK INSTRUCTIONS
    // These are instructions which help the emulator use the hardware stack
    /// Returns the stack pointer's index in memory
    const fn stk_index(&self) -> usize {
        STACK_PAGE + self.regs.stk as usize
    }

    /// Returns the byte at the top of the stack without mutating memory
    fn stk_peek_byte(&self) -> u8 {
        self.mem[self.stk_index()]
    }

    /// Points the stack pointer at the next stack byte
    /// Pushes the supplied byte onto this stack location
    fn stk_push_byte(&mut self, byte: u8) {
        self.regs.stk -= 1;
        self.mem[self.stk_index()] = byte;
    }

    /// Points the stack pointer at the next stack frame (two byte difference)
    /// Pushes the supplied stack frame onto this stack location
    fn stk_push_frame(&mut self, frame: (u8, u8)) {
        self.stk_push_byte(frame.0);
        self.stk_push_byte(frame.1);
    }

    /// Points the stack pointer at the next stack frame (two byte difference)
    /// Pushes the supplied program counter onto this stack location
    fn stk_push_pc(&mut self, pc: u16) {
        let lo_byte: u8 = (pc & 0x00FF) as u8;
        let hi_byte: u8 = ((pc & 0xFF00) >> 8) as u8;
        self.stk_push_frame((lo_byte, hi_byte));
    }

    /// Returns the byte at the top of the stack and reduces the stack pointer by one
    fn stk_pop_byte(&mut self) -> u8 {
        let ret = self.stk_peek_byte();
        self.regs.stk += 1;
        ret
    }

    /// Returns the two bytes at the top of the stack and reduces the stack pointer by two
    fn stk_pop_frame(&mut self) -> (u8, u8) {
        let lo_byte = self.stk_pop_byte();
        let hi_byte = self.stk_pop_byte();
        (lo_byte, hi_byte)
    }

    /// Returns the top two bytes at the top of the stack and interprets them as one number
    /// Reduces the stack pointer by two.
    fn stk_pop_pc(&mut self) -> u16 {
        let frame = self.stk_pop_frame();
        frame.1 as u16 + ((frame.0 as u16) << 8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stk_peek_byte() {
        let mut state = ComputerState::new();
        assert_eq!(0, state.stk_peek_byte());
        state.mem[STACK_PAGE + PAGE_SIZE] = 1;
        assert_eq!(1, state.stk_peek_byte());
        state.mem[STACK_PAGE + PAGE_SIZE - 1] = 2;
        state.regs.stk -= 1;
        assert_eq!(2, state.stk_peek_byte());
    }

    #[test]
    fn test_stk_push_byte() {
        let mut state = ComputerState::new();
        let old_stk_ptr = state.regs.stk;
        state.stk_push_byte(1);
        assert_ne!(old_stk_ptr, state.regs.stk);
        assert_eq!(1, state.mem[state.stk_index()]);
    }

    #[test]
    fn test_stk_push_frame() {
        let mut state = ComputerState::new();
        // Pushes 0xAB then 0xCD onto the stack in that order
        state.stk_push_frame((0x01, 0x02));
        assert_eq!(0x02, state.mem[state.stk_index()]);
        assert_eq!(0x01, state.mem[state.stk_index() + 1]);
    }

    #[test]
    fn test_stk_push_pc() {
        let mut state = ComputerState::new();
        state.stk_push_pc(0x0102);
        assert_eq!(0x01, state.mem[state.stk_index()]);
        assert_eq!(0x02, state.mem[state.stk_index() + 1]);
    }

    #[test]
    fn test_stk_pop_byte() {
        let mut state = ComputerState::new();
        state.stk_push_byte(1);
        assert_eq!(1, state.stk_pop_byte());
    }

    #[test]
    fn test_stk_pop_pc() {
        let mut state = ComputerState::new();
        state.stk_push_pc(0x0102);
        assert_eq!(0x0102, state.stk_pop_pc());
    }
}