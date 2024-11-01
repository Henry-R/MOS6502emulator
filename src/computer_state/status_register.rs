
struct StatusRegister {
    data: u8
}

impl StatusRegister {
    /// Negative flag
    pub const N: u8 = 1 << 0;
    /// Overflow flag
    pub const V: u8 = 1 << 1;
    /// Break flag
    pub const B: u8 = 1 << 2;
    /// Decimal flag
    pub const D: u8 = 1 << 3;
    /// Interrupt disable
    pub const I: u8 = 1 << 4;
    /// Zero flag
    pub const Z: u8 = 1 << 5;
    /// Carry flag
    pub const C: u8 = 1 << 6;

    pub const fn new() -> Self {
        Self {data: 0}
    }

    pub const fn get_flags_data(&self) -> u8 { self.data }

    pub const fn from_flags(flags: u8) -> Self {
        Self {data:flags}
    }

    pub const fn insert(&self, flag: u8) -> Self {
        Self::from_flags(flag | self.data)
    }

    pub const fn remove(&self, flag: u8) -> Self {
        Self::from_flags((!flag) & self.data)
    }

    pub const fn contains(&self, flag: u8) -> bool {
        (flag & self.data) != 0
    }

    pub const fn is_empty(&self) -> bool {
        self.data == 0
    }
}