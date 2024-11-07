
pub struct Accumulator {
    acc: u8
}
impl Accumulator {
    /// Constructs accumulator initialised to the given value
    pub fn new(acc: u8) -> Self
    { Self { acc } }

    /// Gets the value inside the accumulator
    pub const fn get(&self) -> u8
    { self.acc }

    /// Sets the value inside the accumulator
    pub fn set(&mut self, value: u8)
    { self.acc = value }
}


pub struct ProgramCounter {
    pc: u16
}
impl ProgramCounter {
    /// Constructs program counter initialised to point at the given address
    pub fn new(pc: usize) -> Self
    { Self { pc: pc as u16 } }

    /// Gets current address in the PC
    pub fn get(&self) -> usize
    { usize::from(self.pc) }

    /// Sets new address in the PC
    pub fn set(&mut self, new_pc: usize)
    { self.pc = new_pc as u16 }

    /// Adds an unsigned offset to the PC
    pub fn add_unsigned(&mut self, value: u8)
    { self.pc += u16::from(value) }

    /// Adds the raw signed offset to the PC.
    /// Although this method takes an unsigned value,
    /// this value will be reinterpreted as a signed value
    pub fn add_signed(&mut self, value: i8)
    {
        if value > 0 {
            self.pc = self.pc.wrapping_add(value as u16)
        } else {
            self.pc = self.pc.wrapping_sub((-value) as u16)
        }
    }
}


pub struct Stack<'a> {
    stk: usize,
    stk_mem: &'a mut [u8],
}
impl Stack<'_> {
    /// Constructs stack pointer initialised to the top of the stack page
    pub fn new(stk_mem: &mut [u8]) -> Stack
    { Stack {
        stk: stk_mem.len(),
        stk_mem,
    } }

    /// Returns the stack pointer's index in memory
    const fn get_index(&self) -> usize { self.stk }

    /// Returns the byte at the top of the stack without mutating memory
    const fn peek_byte(&self) -> u8 {
        self.stk_mem[self.get_index()]
    }

    /// Points the stack pointer at the next stack byte
    /// Pushes the supplied byte onto this stack location
    fn push_byte(&mut self, byte: u8) {
        self.stk -= 1;
        self.stk_mem[self.get_index()] = byte;
    }

    /// Points the stack pointer at the next stack frame (two byte difference)
    /// Pushes the supplied stack frame onto this stack location
    fn push_frame(&mut self, frame: (u8, u8)) {
        self.push_byte(frame.0);
        self.push_byte(frame.1);
    }

    /// Points the stack pointer at the next stack frame (two byte difference)
    /// Pushes the supplied program counter onto this stack location
    fn push_pc(&mut self, pc: usize) {
        let lo_byte: u8 = (pc & 0x00FF) as u8;
        let hi_byte: u8 = ((pc & 0xFF00) >> 8) as u8;
        self.push_frame((lo_byte, hi_byte));
    }

    /// Returns the byte at the top of the stack and reduces the stack pointer by one
    fn pop_byte(&mut self) -> u8 {
        let ret = self.peek_byte();
        self.stk += 1;
        ret
    }

    /// Returns the two bytes at the top of the stack and reduces the stack pointer by two
    fn pop_frame(&mut self) -> (u8, u8) {
        let lo_byte = self.pop_byte();
        let hi_byte = self.pop_byte();
        (hi_byte, lo_byte)
    }

    /// Returns the top two bytes at the top of the stack and interprets them as one number
    /// Reduces the stack pointer by two.
    fn pop_pc(&mut self) -> u16 {
        let frame = self.pop_frame();
        frame.1 as u16 + ((frame.0 as u16) << 8)
    }
}
