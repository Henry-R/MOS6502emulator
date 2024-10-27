use bitflags::bitflags;

mod operations;

bitflags! {
    struct StatusFlags: u8 {
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


struct Registers {
    // (A) Accumulator
    pub acc: u8,
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

const MEMORY_SIZE: usize = 2usize.pow(16);
struct ComputerState {
    // MEMORY
    // Each page is 256 bytes
    // First page is reserved for the Zero-Page ($0000-$00FF)
    // Second page is reserved for system stack ($0100-$01FF)
    // Last 6 bytes are reserved for interrupts ($FFFA-$FFFF)
    mem: [u8; MEMORY_SIZE],

    regs: Registers,
}

impl ComputerState {
    // Important memory locations
    const ZERO_PAGE: usize = 0x0000;
    const STACK_PAGE: usize = 0x0100;
    
    fn new() -> ComputerState {
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

    // STACK INSTRUCTIONS
    /// Returns the byte at the top of the stack without mutating memory
    pub fn stk_peek_byte(& self) -> u8 {
        self.mem[Self::STACK_PAGE + self.regs.stk as usize]
    }

    /// Returns the byte at the top of the stack and reduces the stack pointer by one
    pub fn stk_pop_byte(&mut self) -> u8 {
        let ret = self.stk_peek_byte();
        self.regs.stk += 1;
        ret
    }

    /// Increases the stack pointer by one, and pushed the supplied byte onto this stack location
    pub fn stk_push_byte(&mut self, byte: u8){
        self.regs.stk -= 1;
        self.mem[Self::STACK_PAGE + self.regs.stk as usize] = byte;
    }

    pub fn stk_pop_frame(&mut self) -> (u8, u8) {
        let lo_byte = self.stk_pop_byte();
        let hi_byte = self.stk_pop_byte();
        (lo_byte, hi_byte)
    }

    pub fn stk_push_frame(&mut self, frame: (u8, u8)) {
        self.stk_push_byte(frame.0);
        self.stk_push_byte(frame.1);
    }

    pub fn stk_pop_pc(&mut self) -> u16 {
        let lo_byte = self.stk_pop_byte() as u16;
        let hi_byte = self.stk_pop_byte() as u16;
        lo_byte + (hi_byte >> 8)
    }

    pub fn stk_push_pc(&mut self, pc: u16) {
        let lo_byte: u8 =  (pc | 0x00FF)       as u8;
        let hi_byte: u8 = ((pc | 0xFF00) >> 8) as u8;
        self.stk_push_byte(lo_byte);
        self.stk_push_byte(hi_byte);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_stk_peek_byte() {
        let state = ComputerState::new();
        let result = state.stk_peek_byte();
        assert_eq!(0u8, result);
    }
}

fn main() {


    println!("Hello, world!");
}
