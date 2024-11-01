use std::ops::BitOrAssign;

pub struct StatusRegister {
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
    /// Empty flag
    pub const EMPTY: u8 = 0;

    /// Constructs a status register with every flag set to 0
    pub const fn new() -> Self { Self {data: 0} }

    /// Returns the raw u8 representing the flags
    pub const fn get_flags_data(&self) -> u8 { self.data }

    /// Constructs a status register from a raw u8
    pub const fn from_flags(flags: u8) -> Self { Self {data: flags} }

    /// Returns the mathematical union of the internal flags and the given flag
    pub const fn union(&self, flags: u8) -> Self {
        Self::from_flags(self.data | flags)
    }

    /// Returns the mathematical intersection of the internal flags and the given flag
    pub const fn intersect(&self, flags: u8) -> Self {
        Self::from_flags(self.data & flags)
    }

    /// Returns the mathematical difference of the internal flags and the given flag
    pub const fn difference(&self, flags: u8) -> Self {
        Self::from_flags(self.data & !flags)
    }

    /// True if this status contains the given flag
    pub const fn contains(&self, flag: u8) -> bool { (flag & self.data) != 0 }

    /// True if this status contains no flags
    pub const fn is_empty(&self) -> bool { self.data == 0 }
}

// Operators
impl std::ops::BitOr for StatusRegister {
    type Output = Self;

    const fn bitor(self, rhs: Self) -> Self::Output {
        Self::from_flags(self.data | rhs.data)
    }
}
impl std::ops::BitOr<u8> for StatusRegister {
    type Output = Self;

    const fn bitor(self, rhs: u8) -> Self::Output {
        Self::from_flags(self.data | rhs)
    }
}
impl BitOrAssign for StatusRegister {
    fn bitor_assign(&mut self, rhs: Self) {
        self.data |= rhs.data
    }
}