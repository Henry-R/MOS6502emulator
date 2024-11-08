
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
            self.pc = self.pc.wrapping_sub(value.wrapping_abs() as u16)
        }
    }
}


