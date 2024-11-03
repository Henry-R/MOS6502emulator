
struct Accumulator {
    data: u8
}
impl Accumulator {
    /// Constructs accumulator initialised to the given value
    pub fn new(acc: u8) -> Self
    { Self { data: acc } }

    /// Gets the value inside the accumulator
    pub const fn get(&self) -> u8
    { self.data }

    /// Sets the value inside the accumulator
    pub fn set(&mut self, value: u8)
    { self.data = value }
}


struct ProgramCounter {
    data: usize
}
impl ProgramCounter {
    /// Constructs program counter initialised to point at the given address
    pub fn new(pc: usize) -> Self
    { Self { data: pc } }

    /// Gets current address in the PC
    pub const fn get(&self) -> usize
    { self.data }

    /// Sets new address in the PC
    pub fn set(&mut self, new_pc: usize)
    { self.data = new_pc }

    /// Adds an unsigned offset to the PC
    pub fn add_unsigned(&mut self, value: u8)
    { self.data += usize::from(value) }

    /// Reinterpret cast unsigned to signed
    const fn unsigned_reinterpret_signed(value: u8) -> i8 {
        ((0xFF ^ value) + 1) as i8
    }
    /// Adds a signed offset to an usize address
    const fn add_signed_to_usize(signed: i8, unsigned: usize) -> usize {
        if signed > 0 {
            unsigned + signed as usize
        } else {
            unsigned + (-signed) as usize
        }
    }

    /// Adds the raw signed offset to the PC.
    /// Although this method takes an unsigned value,
    /// this value will be reinterpreted as a signed value
    pub fn add_signed(&mut self, value: u8)
    {
        let offset = Self::unsigned_reinterpret_signed(value);
        self.data += Self::add_signed_to_usize(offset, self.data);
    }
}
