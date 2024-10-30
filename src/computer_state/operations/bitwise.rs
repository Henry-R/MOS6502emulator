use std::ops::BitAnd;
use crate::computer_state::{ComputerState, StatusFlags};
use crate::computer_state::operations::u8_to_i8;

// AND
/// AND (bitwise and)
fn and(state: &mut ComputerState, value: i8) {
    // REGISTERS
    state.regs.acc &= value;

    // FLAGS
    if state.regs.acc == 0 { state.regs.sta.insert(StatusFlags::z); }
    if state.regs.acc < 0 { state.regs.sta.insert(StatusFlags::n); }
}

/// AND (intermediate addressing mode)
/// Opcode: 29
pub fn and_im(state: &mut ComputerState) {
    let value = u8_to_i8(state.fetch_intermediate());
    and(state, value);
}
/// AND (zero-page addressing mode)
/// Opcode: 25
pub fn and_zp(state: &mut ComputerState) {
    let value = u8_to_i8(state.fetch_zero_page());
    and(state, value);
}
/// AND (zero-page X addressing mode)
/// Opcode: 35
pub fn and_zpx(state: &mut ComputerState) {
    let value = u8_to_i8(state.fetch_zero_page_x());
    and(state, value);
}
/// AND (absolute addressing mode)
/// Opcode: 2D
pub fn and_ab(state: &mut ComputerState) {
    let value = u8_to_i8(state.fetch_absolute());
    and(state, value);
}
/// AND (absolute X addressing mode)
/// Opcode: 3D
pub fn and_abx(state: &mut ComputerState) {
    let value = u8_to_i8(state.fetch_absolute_x());
    and(state, value);
}
/// AND (absolute Y addressing mode)
/// Opcode: 39
pub fn and_aby(state: &mut ComputerState) {
    let value = u8_to_i8(state.fetch_absolute_y());
    and(state, value);
}
/// AND (indirect X addressing mode)
/// Opcode: 21
pub fn and_inx(state: &mut ComputerState) {
    let value = u8_to_i8(state.fetch_indexed_indirect());
    and(state, value);
}
/// AND (indirect Y addressing mode)
/// Opcode: 31
pub fn and_iny(state: &mut ComputerState) {
    let value = u8_to_i8(state.fetch_indirect_indexed());
    and(state, value);
}


/// OR (logical bitwise inclusive or)
fn or(state: &mut ComputerState, value: i8) {
    // REGISTERS
    state.regs.acc |= value;

    // FLAGS
    if state.regs.acc == 0 { state.regs.sta.insert(StatusFlags::z); }
    if state.regs.acc < 0 { state.regs.sta.insert(StatusFlags::n); }
}


/// OR (intermediate addressing mode)
fn or_im(state: &mut ComputerState) {
    let value = u8_to_i8(state.fetch_intermediate());
    or(state, value);
}
/// OR (zero-page addressing mode)
fn or_zp(state: &mut ComputerState) {
    let value = u8_to_i8(state.fetch_zero_page());
    or(state, value);
}
/// OR (zero-page X addressing mode)
fn or_zpx(state: &mut ComputerState) {
    let value = u8_to_i8(state.fetch_zero_page_x());
    or(state, value);
}
/// OR (zero-page Y addressing mode)
fn or_zpy(state: &mut ComputerState) {
    let value = u8_to_i8(state.fetch_zero_page_y());
    or(state, value);
}
/// OR (absolute addressing mode)
fn or_ab(state: &mut ComputerState) {
    let value = u8_to_i8(state.fetch_absolute());
    or(state, value);
}
/// OR (absolute X addressing mode)
fn or_abx(state: &mut ComputerState) {
    let value = u8_to_i8(state.fetch_absolute_x());
    or(state, value);
}
/// OR (absolute Y addressing mode)
fn or_aby(state: &mut ComputerState) {
    let value = u8_to_i8(state.fetch_absolute_y());
    or(state, value);
}
/// OR (indirect X addressing mode)
fn or_inx(state: &mut ComputerState) {
    let value = u8_to_i8(state.fetch_indexed_indirect());
    or(state, value);
}
/// OR (indirect Y addressing mode)
fn or_iny(state: &mut ComputerState) {
    let value = u8_to_i8(state.fetch_indirect_indexed());
    or(state, value);
}


/// EOR (logical bitwise exclusive or)
fn eor(state: &mut ComputerState, value: i8) {
    // REGISTERS
    state.regs.acc ^= value;

    // FLAGS
    if state.regs.acc == 0 { state.regs.sta.insert(StatusFlags::z); }
    if state.regs.acc < 0 { state.regs.sta.insert(StatusFlags::n); }
}


/// EOR (intermediate addressing mode)
fn eor_im(state: &mut ComputerState) {
    let value = u8_to_i8(state.fetch_intermediate());
    eor(state, value);
}
/// EOR (zero-page addressing mode)
fn eor_zp(state: &mut ComputerState) {
    let value = u8_to_i8(state.fetch_zero_page());
    eor(state, value);
}
/// EOR (zero-page X addressing mode)
fn eor_zpx(state: &mut ComputerState) {
    let value = u8_to_i8(state.fetch_zero_page_x());
    eor(state, value);
}
/// EOR (zero-page Y addressing mode)
fn eor_zpy(state: &mut ComputerState) {
    let value = u8_to_i8(state.fetch_zero_page_y());
    eor(state, value);
}
/// EOR (absolute addressing mode)
fn eor_ab(state: &mut ComputerState) {
    let value = u8_to_i8(state.fetch_absolute());
    eor(state, value);
}
/// EOR (absolute X addressing mode)
fn eor_abx(state: &mut ComputerState) {
    let value = u8_to_i8(state.fetch_absolute_x());
    eor(state, value);
}
/// EOR (absolute Y addressing mode)
fn eor_aby(state: &mut ComputerState) {
    let value = u8_to_i8(state.fetch_absolute_y());
    eor(state, value);
}
/// EOR (indirect X addressing mode)
fn eor_inx(state: &mut ComputerState) {
    let value = u8_to_i8(state.fetch_indexed_indirect());
    eor(state, value);
}
/// EOR (indirect Y addressing mode)
fn eor_iny(state: &mut ComputerState) {
    let value = u8_to_i8(state.fetch_indirect_indexed());
    eor(state, value);
}


/// ASL (arithmetic shift left)
fn asl(state: &mut ComputerState) {
    let (result, overflow) = state.regs.acc.overflowing_shl(1);

    // REGISTERS
    state.regs.acc = result;

    // FLAGS
    if overflow    { state.regs.sta.insert(StatusFlags::c); }
    if result < 0  { state.regs.sta.insert(StatusFlags::n); }
    if result == 0 { state.regs.sta.insert(StatusFlags::z); }
}


/// ASR (arithmetic shift right)
fn asr(state: &mut ComputerState) {
    let (result, overflow) = state.regs.acc.overflowing_shr(1);

    // REGISTERS
    state.regs.acc = result;

    // FLAGS
    if overflow    { state.regs.sta.insert(StatusFlags::c); }
    if result < 0  { state.regs.sta.insert(StatusFlags::n); }
    if result == 0 { state.regs.sta.insert(StatusFlags::z); }
}


#[cfg(test)]
mod tests {
    use crate::computer_state::ComputerState;
    use crate::computer_state::operations::bitwise::{and, eor, or};

    #[test]
    fn test_and() {
        let mut state = ComputerState::new();

        and(&mut state, 0b0000_0000);
        assert_eq!(state.regs.acc, 0);
        state.regs.acc = 0b0101_0101;
        and(&mut state, 0b0000_1111);
        assert_eq!(state.regs.acc, 0b0000_0101);
        and(&mut state, 0b0011_0001);
        assert_eq!(state.regs.acc, 0b0000_0001);
    }

    #[test]
    fn test_or() {
        let mut state = ComputerState::new();

        or(&mut state, 0b0101_0101);
        assert_eq!(state.regs.acc, 0b0101_0101);
        or(&mut state, 0b0000_1111);
        assert_eq!(state.regs.acc, 0b0101_1111);
        or(&mut state, 0b0010_0000);
        assert_eq!(state.regs.acc, 0b0111_1111);
    }

    #[test]
    fn test_eor() {
        let mut state = ComputerState::new();

        eor(&mut state, 0b0101_0101);
        assert_eq!(state.regs.acc, 0b0101_0101);
        eor(&mut state, 0b0101_1010);
        assert_eq!(state.regs.acc, 0b0000_1111);
    }
}