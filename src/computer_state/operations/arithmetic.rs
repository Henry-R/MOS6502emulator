use crate::computer_state::{ComputerState, StatusFlags};

// ADDITION
/// ADC with carry
/// Adds two integers and returns their sum. If an overflow occurs, the C and V flags will be set
fn add(n: u8, m: u8) -> (u8, StatusFlags) {
    // Addition with remainder
    let (sum, overflow) = n.overflowing_add(m);

    // FLAGS
    let mut flags = StatusFlags::empty();
    if overflow {
        flags.insert(StatusFlags::c);
        flags.insert(StatusFlags::v);
    }
    if sum == i8::MAX as u8 { flags.insert(StatusFlags::z) }
    if sum < i8::MAX as u8 { flags.insert(StatusFlags::n) }

    (sum, flags)
}

/// ADC (intermediate addressing mode)
/// Opcode: 69
pub fn add_im(state: &mut ComputerState) {
    let (val, flags) = add(state.regs.acc, state.fetch_intermediate());
    state.regs.acc = val;
    state.regs.sta |= flags
}
/// ADC (zero-page addressing mode)
/// Opcode: 65
pub fn add_zp(state: &mut ComputerState) {
    let (val, flags) = add(state.regs.acc, state.fetch_zero_page());
    state.regs.acc = val;
    state.regs.sta |= flags
}
/// ADC (zero-page X addressing mode)
/// Opcode: 75
pub fn add_zpx(state: &mut ComputerState) {
    let (val, flags) = add(state.regs.acc, state.fetch_zero_page_x());
    state.regs.acc = val;
    state.regs.sta |= flags
}
/// ADC (absolute addressing mode)
/// Opcode: 6D
pub fn add_ab(state: &mut ComputerState) {
    let (val, flags) = add(state.regs.acc, state.fetch_absolute());
    state.regs.acc = val;
    state.regs.sta |= flags
}
/// ADC (absolute X addressing mode)
/// Opcode: 7D
pub fn add_abx(state: &mut ComputerState) {
    let (val, flags) = add(state.regs.acc, state.fetch_absolute_x());
    state.regs.acc = val;
    state.regs.sta |= flags
}
/// ADC (absolute Y addressing mode)
/// Opcode: 79
pub fn add_aby(state: &mut ComputerState) {
    let (val, flags) = add(state.regs.acc, state.fetch_absolute_y());
    state.regs.acc = val;
    state.regs.sta |= flags
}
/// ADC (indirect X addressing mode)
/// Opcode: 61
pub fn add_inx(state: &mut ComputerState) {
    let (val, flags) = add(state.regs.acc, state.fetch_indexed_indirect());
    state.regs.acc = val;
    state.regs.sta |= flags
}
/// ADC (indirect Y addressing mode)
/// Opcode: 71
pub fn add_iny(state: &mut ComputerState) {
    let (val, flags) = add(state.regs.acc, state.fetch_indirect_indexed());
    state.regs.acc = val;
    state.regs.sta |= flags
}


// SUBTRACTION
/// SBC (subtraction with carry)
fn sub(n: u8, m: u8) -> (u8, StatusFlags) {
    // Subtraction with remainder
    let (result, overflow) = n.overflowing_sub(m);

    // FLAGS
    let mut flags = StatusFlags::empty();
    // Carry and overflow bits set TODO maybe subtle carry logic could create bugs
    if overflow {
        flags.insert(StatusFlags::c);
        flags.insert(StatusFlags::v);
    }
    // Result was zero
    if result == 0 { flags.insert(StatusFlags::z) }
    if result < i8::MAX as u8 { flags.insert(StatusFlags::n) };
    (result, flags)
}

/// SBC (intermediate addressing mode)
/// Opcode: E9
pub fn sub_im(state: &mut ComputerState) {
    let (val, flags) = sub(state.regs.acc, state.fetch_intermediate());
    state.regs.acc = val;
    state.regs.sta |= flags
}
/// SBC (zero-page addressing mode)
/// Opcode: E5
pub fn sub_zp(state: &mut ComputerState) {
    let (val, flags) = sub(state.regs.acc, state.fetch_zero_page());
    state.regs.acc = val;
    state.regs.sta |= flags
}
/// SBC (zero-page X addressing mode)
/// Opcode:F5
pub fn sub_zpx(state: &mut ComputerState) {
    let (val, flags) = sub(state.regs.acc, state.fetch_zero_page_x());
    state.regs.acc = val;
    state.regs.sta |= flags
}
/// SBC (absolute addressing mode)
/// Opcode: ED
pub fn sub_ab(state: &mut ComputerState) {
    let (val, flags) = sub(state.regs.acc, state.fetch_absolute());
    state.regs.acc = val;
    state.regs.sta |= flags
}
/// SBC (absolute X addressing mode)
/// Opcode: FD
pub fn sub_abx(state: &mut ComputerState) {
    let (val, flags) = sub(state.regs.acc, state.fetch_absolute_x());
    state.regs.acc = val;
    state.regs.sta |= flags
}
/// SBC (absolute Y addressing mode)
/// Opcode: F9
pub fn sub_aby(state: &mut ComputerState) {
    let (val, flags) = sub(state.regs.acc, state.fetch_absolute_y());
    state.regs.acc = val;
    state.regs.sta |= flags
}
/// SBC (indirect X addressing mode)
/// Opcode: E1
pub fn sub_inx(state: &mut ComputerState) {
    let (val, flags) = sub(state.regs.acc, state.fetch_indexed_indirect());
    state.regs.acc = val;
    state.regs.sta |= flags
}
/// SBC (indirect Y addressing mode)
/// Opcode: F1
pub fn sub_iny(state: &mut ComputerState) {
    let (val, flags) = sub(state.regs.acc, state.fetch_indirect_indexed());
    state.regs.acc = val;
    state.regs.sta |= flags
}


/// DEC (Decrement memory by one)
/// Returns tuple containing the new value to place in memory,
/// and the status flags after the operation has completed
fn dec(val: u8) -> (u8, StatusFlags) {
    // No documentation says this function can wrap
    let result = val - 1;
    let mut flags = StatusFlags::empty();

    if result == i8::MAX as u8 { flags.insert(StatusFlags::z); }
    if result <  i8::MAX as u8 { flags.insert(StatusFlags::n); }

    (result, flags)
}
/// DEC (zero-page addressing mode)
/// Opcode: C6
pub fn dec_zp(state: &mut ComputerState) {
    let zp_addr = state.fetch_zero_page_address();
    (state.mem[zp_addr], state.regs.sta) = dec(state.mem[zp_addr])
}
/// DEC (zero-page X addressing mode)
/// Opcode: D6
pub fn dec_zpx(state: &mut ComputerState) {
    let zpx_addr = state.fetch_zero_page_x_address();
    (state.mem[zpx_addr], state.regs.sta) = dec(state.mem[zpx_addr])
}
/// DEC (absolute addressing mode)
/// Opcode: CE
pub fn dec_ab(state: &mut ComputerState) {
    let ab_addr = state.fetch_absolute_address();
    (state.mem[ab_addr], state.regs.sta) = dec(state.mem[ab_addr])
}
/// DEC (absolute X addressing mode)
/// Opcode: DE
pub fn dec_abx(state: &mut ComputerState) {
    let abx_addr = state.fetch_absolute_address_x();
    (state.mem[abx_addr], state.regs.sta) = dec(state.mem[abx_addr])
}

/// DEX (implied addressing mode)
/// Opcode: CA
pub fn dex(state: &mut ComputerState) {
    let (result, flags) = dec(state.regs.x);
    state.regs.x = result;
    state.regs.sta |= flags
}
/// DEY (implied addressing mode)
/// Opcode: 88
pub fn dey(state: &mut ComputerState) {
    let (result, flags) = dec(state.regs.y);
    state.regs.y = result;
    state.regs.sta |= flags
}


/// INC (Increment memory by one)
/// Returns tuple containing the new value to place in memory,
/// and the status flags after the operation has completed
fn inc(val: u8) -> (u8, StatusFlags) {
    // No documentation says this function can wrap
    let result = val + 1;
    let mut flags = StatusFlags::empty();

    if result == i8::MAX as u8 { flags.insert(StatusFlags::z); }
    if result <  i8::MAX as u8 { flags.insert(StatusFlags::n); }

    (result, flags)
}
/// INC (zero-page addressing mode)
/// Opcode: E6
pub fn inc_zp(state: &mut ComputerState) {
    let zp_addr = state.fetch_zero_page_address();
    (state.mem[zp_addr], state.regs.sta) = inc(state.mem[zp_addr])
}
/// INC (zero-page X addressing mode)
/// Opcode: F6
pub fn inc_zpx(state: &mut ComputerState) {
    let zpx_addr = state.fetch_zero_page_x_address();
    (state.mem[zpx_addr], state.regs.sta) = inc(state.mem[zpx_addr])
}
/// INC (absolute addressing mode)
/// Opcode: EE
pub fn inc_ab(state: &mut ComputerState) {
    let ab_addr = state.fetch_absolute_address();
    (state.mem[ab_addr], state.regs.sta) = inc(state.mem[ab_addr])
}
/// INC (absolute X addressing mode)
/// Opcode: FE
pub fn inc_abx(state: &mut ComputerState) {
    let abx_addr = state.fetch_absolute_address_x();
    (state.mem[abx_addr], state.regs.sta) = inc(state.mem[abx_addr])
}

/// INX (implied addressing mode)
/// Opcode: E8
pub fn inx(state: &mut ComputerState) {
    let (result, flags) = inc(state.regs.x);
    state.regs.x = result;
    state.regs.sta |= flags
}
/// INY (implied addressing mode)
/// Opcode: C8
pub fn iny(state: &mut ComputerState) {
    let (result, flags) = inc(state.regs.y);
    state.regs.y = result;
    state.regs.sta |= flags
}

#[cfg(test)]
mod tests {
    use crate::computer_state::StatusFlags;
    use crate::computer_state::operations::arithmetic::{add, dec, sub};

    #[test]
    fn test_add() {
        let (result, flags) = add(0, 129);
        assert_eq!(result, 129);
        assert!(flags.is_empty());

        let (result, flags) = add(result, 128);
        assert_eq!(result, 1);
        assert!(flags.contains(StatusFlags::c));
        assert!(flags.contains(StatusFlags::n));

        let (result, flags) = add(result, 126);
        assert_eq!(result, i8::MAX as u8);
        assert!(flags.contains(StatusFlags::z));
    }
    #[test]
    fn test_sub() {
        let (result, flags) = sub(0, 100);
        assert_eq!(result, 156);
        assert!(flags.contains(StatusFlags::c));

        let (result, flags) = sub(result, 100);
        assert!(flags.contains(StatusFlags::n));

        let (result, flags) = sub(result, 56);
        assert_eq!(result, 0);
        assert!(flags.contains(StatusFlags::z));
    }

    #[test]
    fn test_dec() {
        let (val, flags) = dec(128);
        assert!(flags.contains(StatusFlags::z));
        assert_eq!(127, val);
        let (val, flags) = dec(127);
        assert!(flags.contains(StatusFlags::n));
        assert_eq!(126, val);
    }
}