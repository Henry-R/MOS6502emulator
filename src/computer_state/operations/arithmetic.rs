use crate::computer_state::{ComputerState, StatusFlags};

// ADDITION
/// ADd with carry
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

/// ADc (intermediate addressing mode)
/// Opcode: 69
pub fn add_im(state: &mut ComputerState) {
    let (val, flags) = add(state.regs.acc, state.fetch_intermediate());
    state.regs.acc = val;
    state.regs.sta |= flags
}
/// ADc (zero-page addressing mode)
/// Opcode: 65
pub fn add_zp(state: &mut ComputerState) {
    let (val, flags) = add(state.regs.acc, state.fetch_zero_page());
    state.regs.acc = val;
    state.regs.sta |= flags
}
/// ADc (zero-page X addressing mode)
/// Opcode: 75
pub fn add_zpx(state: &mut ComputerState) {
    let (val, flags) = add(state.regs.acc, state.fetch_zero_page_x());
    state.regs.acc = val;
    state.regs.sta |= flags
}
/// ADc (absolute addressing mode)
/// Opcode: 6D
pub fn add_ab(state: &mut ComputerState) {
    let (val, flags) = add(state.regs.acc, state.fetch_absolute());
    state.regs.acc = val;
    state.regs.sta |= flags
}
/// ADc (absolute X addressing mode)
/// Opcode: 7D
pub fn add_abx(state: &mut ComputerState) {
    let (val, flags) = add(state.regs.acc, state.fetch_absolute_x());
    state.regs.acc = val;
    state.regs.sta |= flags
}
/// ADc (absolute Y addressing mode)
/// Opcode: 79
pub fn add_aby(state: &mut ComputerState) {
    let (val, flags) = add(state.regs.acc, state.fetch_absolute_y());
    state.regs.acc = val;
    state.regs.sta |= flags
}
/// ADc (indirect X addressing mode)
/// Opcode: 61
pub fn add_inx(state: &mut ComputerState) {
    let (val, flags) = add(state.regs.acc, state.fetch_indexed_indirect());
    state.regs.acc = val;
    state.regs.sta |= flags
}
/// ADc (indirect Y addressing mode)
/// Opcode: 71
pub fn add_iny(state: &mut ComputerState) {
    let (val, flags) = add(state.regs.acc, state.fetch_indirect_indexed());
    state.regs.acc = val;
    state.regs.sta |= flags
}


// SUBTRACTION
/// SBc (subtraction with carry)
fn sub(state: &mut ComputerState, value: u8) {
    // Subtraction with remainder
    let (sum, overflow) = state.regs.acc.overflowing_sub(value);

    // REGISTERS
    state.regs.acc = sum;

    // FLAGS
    // Carry and overflow bits set TODO maybe subtle carry logic could create bugs
    if overflow {
        state.regs.sta.insert(StatusFlags::c);
        state.regs.sta.insert(StatusFlags::v);
    }
    // Result was zero
    if sum == 0 { state.regs.sta.insert(StatusFlags::z) }
    if sum < i8::MAX as u8 { state.regs.sta.insert(StatusFlags::n) };
}

/// SBc (intermediate addressing mode)
/// Opcode: E9
pub fn sub_im(state: &mut ComputerState) {
    let value = state.fetch_intermediate();
    sub(state, value);
}
/// SBc (zero-page addressing mode)
/// Opcode: E5
pub fn sub_zp(state: &mut ComputerState) {
    let value = state.fetch_zero_page();
    sub(state, value);
}
/// SBc (zero-page X addressing mode)
/// Opcode:F5
pub fn sub_zpx(state: &mut ComputerState) {
    let value = state.fetch_zero_page_x();
    sub(state, value);
}
/// SBc (absolute addressing mode)
/// Opcode: ED
pub fn sub_ab(state: &mut ComputerState) {
    let value = state.fetch_absolute();
    sub(state, value);
}
/// SBc (absolute X addressing mode)
/// Opcode: FD
pub fn sub_abx(state: &mut ComputerState) {
    let value = state.fetch_absolute_x();
    sub(state, value);
}
/// SBc (absolute Y addressing mode)
/// Opcode: F9
pub fn sub_aby(state: &mut ComputerState) {
    let value = state.fetch_absolute_y();
    sub(state, value);
}
/// SBc (indirect X addressing mode)
/// Opcode: E1
pub fn sub_inx(state: &mut ComputerState) {
    let value = state.fetch_indexed_indirect();
    sub(state, value);
}
/// SBc (indirect Y addressing mode)
/// Opcode: F1
pub fn sub_iny(state: &mut ComputerState) {
    let value = state.fetch_indirect_indexed();
    sub(state, value);
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

#[cfg(test)]
mod tests {
    use crate::computer_state::{ComputerState, StatusFlags};
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
        let mut state = ComputerState::new();

        sub(&mut state, 100);
        assert_eq!(state.regs.acc, 156);
        assert!(state.regs.sta.contains(StatusFlags::c));

        sub(&mut state, 100);
        assert!(state.regs.sta.contains(StatusFlags::n));

        sub(&mut state, 56);
        assert_eq!(state.regs.acc, 0);
        assert!(state.regs.sta.contains(StatusFlags::z));
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