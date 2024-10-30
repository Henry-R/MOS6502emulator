use crate::computer_state::{ComputerState, StatusFlags};
use crate::computer_state::operations::u8_to_i8;

// ADDITION
/// ADd with carry
fn add(state: &mut ComputerState, value: i8) {
    // Addition with remainder
    let (sum, overflow) = state.regs.acc.overflowing_add(value);

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
}

/// ADc (intermediate addressing mode)
/// Opcode: 69
pub fn add_im(state: &mut ComputerState) {
    let value = u8_to_i8(state.fetch_intermediate());
    add(state, value);
}
/// ADc (zero-page addressing mode)
/// Opcode: 65
pub fn add_zp(state: &mut ComputerState) {
    let value = u8_to_i8(state.fetch_zero_page());
    add(state, value);
}
/// ADc (zero-page X addressing mode)
/// Opcode: 75
pub fn add_zpx(state: &mut ComputerState) {
    let value = u8_to_i8(state.fetch_zero_page_x());
    add(state, value);
}
/// ADc (absolute addressing mode)
/// Opcode: 6D
pub fn add_ab(state: &mut ComputerState) {
    let value = u8_to_i8(state.fetch_absolute());
    add(state, value);
}
/// ADc (absolute X addressing mode)
/// Opcode: 7D
pub fn add_abx(state: &mut ComputerState) {
    let value = u8_to_i8(state.fetch_absolute_x());
    add(state, value);
}
/// ADc (absolute Y addressing mode)
/// Opcode: 79
pub fn add_aby(state: &mut ComputerState) {
    let value = u8_to_i8(state.fetch_absolute_y());
    add(state, value);
}
/// ADc (indirect X addressing mode)
/// Opcode: 61
pub fn add_inx(state: &mut ComputerState) {
    let value = u8_to_i8(state.fetch_indexed_indirect());
    add(state, value);
}
/// ADc (indirect Y addressing mode)
/// Opcode: 71
pub fn add_iny(state: &mut ComputerState) {
    let value = u8_to_i8(state.fetch_indirect_indexed());
    add(state, value);
}


// SUBTRACTION
/// SBc (subtraction with carry)
fn sub(state: &mut ComputerState, value: i8) {
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
}

/// SBc (intermediate addressing mode)
/// Opcode: E9
pub fn sub_im(state: &mut ComputerState) {
    let value = u8_to_i8(state.fetch_intermediate());
    sub(state, value);
}
/// SBc (zero-page addressing mode)
/// Opcode: E5
pub fn sub_zp(state: &mut ComputerState) {
    let value = u8_to_i8(state.fetch_zero_page());
    sub(state, value);
}
/// SBc (zero-page X addressing mode)
/// Opcode:F5
pub fn sub_zpx(state: &mut ComputerState) {
    let value = u8_to_i8(state.fetch_zero_page_x());
    sub(state, value);
}
/// SBc (absolute addressing mode)
/// Opcode: ED
pub fn sub_ab(state: &mut ComputerState) {
    let value = u8_to_i8(state.fetch_absolute());
    sub(state, value);
}
/// SBc (absolute X addressing mode)
/// Opcode: FD
pub fn sub_abx(state: &mut ComputerState) {
    let value = u8_to_i8(state.fetch_absolute_x());
    sub(state, value);
}
/// SBc (absolute Y addressing mode)
/// Opcode: F9
pub fn sub_aby(state: &mut ComputerState) {
    let value = u8_to_i8(state.fetch_absolute_y());
    sub(state, value);
}
/// SBc (indirect X addressing mode)
/// Opcode: E1
pub fn sub_inx(state: &mut ComputerState) {
    let value = u8_to_i8(state.fetch_indexed_indirect());
    sub(state, value);
}
/// SBc (indirect Y addressing mode)
/// Opcode: F1
pub fn sub_iny(state: &mut ComputerState) {
    let value = u8_to_i8(state.fetch_indirect_indexed());
    sub(state, value);
}


#[cfg(test)]
mod tests {
    use crate::computer_state::{ComputerState, StatusFlags};
    use crate::computer_state::operations::arithmetic::{add, sub};

    #[test]
    fn test_add() {
        let mut state = ComputerState::new();

        add(&mut state, 100);
        assert_eq!(state.regs.acc, 100);
        assert!(state.regs.sta.is_empty());

        add(&mut state, 100);
        assert_eq!(state.regs.acc, -56);
        assert!(state.regs.sta.contains(StatusFlags::c));

        add(&mut state, 56);
        assert_eq!(state.regs.acc, 0);
        assert!(state.regs.sta.contains(StatusFlags::z));
    }
    #[test]
    fn test_sub() {
        let mut state = ComputerState::new();

        sub(&mut state, 100);
        assert_eq!(state.regs.acc, -100);
        assert!(state.regs.sta.is_empty());

        sub(&mut state, 100);
        assert_eq!(state.regs.acc, 56);
        assert!(state.regs.sta.contains(StatusFlags::c));

        sub(&mut state, 56);
        assert_eq!(state.regs.acc, 0);
        assert!(state.regs.sta.contains(StatusFlags::z));
    }
}