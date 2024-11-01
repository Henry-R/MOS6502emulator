use std::ops::BitOrAssign;

pub struct StatusRegister {
    data: u8
}

impl StatusRegister {
    /// Negative flag
    pub const N: StatusRegister = StatusRegister {data: 1 << 0};
    /// Overflow flag
    pub const V: StatusRegister = StatusRegister {data: 1 << 1};
    /// Break flag
    pub const B: StatusRegister = StatusRegister {data: 1 << 2};
    /// Decimal flag
    pub const D: StatusRegister = StatusRegister {data: 1 << 3};
    /// Interrupt disable
    pub const I: StatusRegister = StatusRegister {data: 1 << 4};
    /// Zero flag
    pub const Z: StatusRegister = StatusRegister {data: 1 << 5};
    /// Carry flag
    pub const C: StatusRegister = StatusRegister {data: 1 << 6};
    /// Empty flag
    pub const EMPTY: StatusRegister = StatusRegister {data: 0};

    /// Constructs a status register with every flag set to 0
    pub const fn new() -> Self { Self {data: 0} }

    /// Constructs a status register from a raw u8
    const fn from_flags(flags: u8) -> Self { Self {data: flags} }

    /// Returns the mathematical union of the internal flags and the given flag
    pub const fn union(&self, flags: StatusRegister) -> Self {
        Self::from_flags(self.data | flags.data)
    }

    /// Returns the mathematical intersection of the internal flags and the given flag
    pub const fn intersect(&self, flags: StatusRegister) -> Self {
        Self::from_flags(self.data & flags.data)
    }

    /// Returns the mathematical difference of the internal flags and the given flag
    pub const fn difference(&self, flags: StatusRegister) -> Self {
        Self::from_flags(self.data & !flags.data)
    }

    /// True if this status contains no flags
    pub const fn is_empty(&self) -> bool { self.data == 0 }

    /// True if this status contains the given flag
    pub const fn contains(&self, flag: StatusRegister) -> bool { !self.intersect(flag).is_empty() }
}

// Operators
impl std::ops::BitOr for StatusRegister {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self::from_flags(self.data | rhs.data)
    }
}
impl std::ops::BitOr<u8> for StatusRegister {
    type Output = Self;

    fn bitor(self, rhs: u8) -> Self::Output {
        Self::from_flags(self.data | rhs)
    }
}
impl BitOrAssign for StatusRegister {
    fn bitor_assign(&mut self, rhs: Self) {
        self.data |= rhs.data
    }
}
impl BitOrAssign<u8> for StatusRegister {
    fn bitor_assign(&mut self, rhs: u8) {
        self.data |= rhs
    }
}

// Utils
/// Get Conditional Flag
/// Returns the given flag if the condition is true, else an empty flag
pub const fn get_cond_flag(flag: StatusRegister, cond: bool) -> StatusRegister {
        if cond { flag } else { StatusRegister::EMPTY }
}

pub const fn get_zero_neg_flags(n: u8) -> StatusRegister {
    const MOST_SIG_BIT: u8 = 0x80;

    get_cond_flag(StatusRegister::Z, n == 0).union(
    get_cond_flag(StatusRegister::N, (n & MOST_SIG_BIT) == MOST_SIG_BIT))
}
