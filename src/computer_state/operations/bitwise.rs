use crate::computer_state::{ComputerState, StatusRegister};
use crate::computer_state::status_register::{get_zero_neg_flags};

// AND
/// AND (bitwise and)
fn and(state: &mut ComputerState, value: u8) {
    state.regs.acc &= value;
    state.regs.sta = get_zero_neg_flags(state.regs.acc);
}

/// AND (intermediate addressing mode)
/// Opcode: 29
pub fn and_im(state: &mut ComputerState) {
    let value = state.fetch_intermediate();
    and(state, value);
}
/// AND (zero-page addressing mode)
/// Opcode: 25
pub fn and_zp(state: &mut ComputerState) {
    let value = state.fetch_zero_page();
    and(state, value);
}
/// AND (zero-page X addressing mode)
/// Opcode: 35
pub fn and_zpx(state: &mut ComputerState) {
    let value = state.fetch_zero_page_x();
    and(state, value);
}
/// AND (absolute addressing mode)
/// Opcode: 2D
pub fn and_ab(state: &mut ComputerState) {
    let value = state.fetch_absolute();
    and(state, value);
}
/// AND (absolute X addressing mode)
/// Opcode: 3D
pub fn and_abx(state: &mut ComputerState) {
    let value = state.fetch_absolute_x();
    and(state, value);
}
/// AND (absolute Y addressing mode)
/// Opcode: 39
pub fn and_aby(state: &mut ComputerState) {
    let value = state.fetch_absolute_y();
    and(state, value);
}
/// AND (indirect X addressing mode)
/// Opcode: 21
pub fn and_inx(state: &mut ComputerState) {
    let value = state.fetch_indirect_x();
    and(state, value);
}
/// AND (indirect Y addressing mode)
/// Opcode: 31
pub fn and_iny(state: &mut ComputerState) {
    let value = state.fetch_indirect_y();
    and(state, value);
}


/// OR (logical bitwise inclusive or)
fn or(state: &mut ComputerState, value: u8) {
    state.regs.acc |= value;
    state.regs.sta = get_zero_neg_flags(state.regs.acc);
}


/// OR (intermediate addressing mode)
/// Opcode: 09
pub fn or_im(state: &mut ComputerState) {
    let value = state.fetch_intermediate();
    or(state, value);
}
/// OR (zero-page addressing mode)
/// Opcode: 05
pub fn or_zp(state: &mut ComputerState) {
    let value = state.fetch_zero_page();
    or(state, value);
}
/// OR (zero-page X addressing mode)
/// Opcode: 15
pub fn or_zpx(state: &mut ComputerState) {
    let value = state.fetch_zero_page_x();
    or(state, value);
}
/// OR (absolute addressing mode)
/// Opcode: 0D
pub fn or_ab(state: &mut ComputerState) {
    let value = state.fetch_absolute();
    or(state, value);
}
/// OR (absolute X addressing mode)
/// Opcode: 1D
pub fn or_abx(state: &mut ComputerState) {
    let value = state.fetch_absolute_x();
    or(state, value);
}
/// OR (absolute Y addressing mode)
/// Opcode: 19
pub fn or_aby(state: &mut ComputerState) {
    let value = state.fetch_absolute_y();
    or(state, value);
}
/// OR (indirect X addressing mode)
/// Opcode: 01
pub fn or_inx(state: &mut ComputerState) {
    let value = state.fetch_indirect_x();
    or(state, value);
}
/// OR (indirect Y addressing mode)
/// Opcode: 11
pub fn or_iny(state: &mut ComputerState) {
    let value = state.fetch_indirect_y();
    or(state, value);
}


/// EOR (logical bitwise exclusive or)
fn eor(state: &mut ComputerState, value: u8) {
    state.regs.acc ^= value;
    state.regs.sta = get_zero_neg_flags(state.regs.acc);
}


/// EOR (intermediate addressing mode)
/// Opcode: 49
pub fn eor_im(state: &mut ComputerState) {
    let value = state.fetch_intermediate();
    eor(state, value);
}
/// EOR (zero-page addressing mode)
/// Opcode: 45
pub fn eor_zp(state: &mut ComputerState) {
    let value = state.fetch_zero_page();
    eor(state, value);
}
/// EOR (zero-page X addressing mode)
/// Opcode: 55
pub fn eor_zpx(state: &mut ComputerState) {
    let value = state.fetch_zero_page_x();
    eor(state, value);
}
/// EOR (absolute addressing mode)
/// Opcode: 4D
pub fn eor_ab(state: &mut ComputerState) {
    let value = state.fetch_absolute();
    eor(state, value);
}
/// EOR (absolute X addressing mode)
/// Opcode: 5D
pub fn eor_abx(state: &mut ComputerState) {
    let value = state.fetch_absolute_x();
    eor(state, value);
}
/// EOR (absolute Y addressing mode)
/// Opcode: 59
pub fn eor_aby(state: &mut ComputerState) {
    let value = state.fetch_absolute_y();
    eor(state, value);
}
/// EOR (indirect X addressing mode)
/// Opcode: 41
pub fn eor_inx(state: &mut ComputerState) {
    let value = state.fetch_indirect_x();
    eor(state, value);
}
/// EOR (indirect Y addressing mode)
/// Opcode: 51
pub fn eor_iny(state: &mut ComputerState) {
    let value = state.fetch_indirect_y();
    eor(state, value);
}


/// ASL (arithmetic shift left)
fn asl(state: &mut ComputerState, value: u8) -> u8 {
    let (result, overflow) = value.overflowing_shl(1);

    // FLAGS
    state.regs.sta =
        StatusRegister::C.get_cond(overflow) |
        get_zero_neg_flags(result);

    result
}
/// ASL (accumulator addressing mode)
/// Opcode: 0A
pub fn asl_acc(state: &mut ComputerState) {
    state.regs.acc = asl(state, state.regs.acc);
}
/// ASL (zero_page addressing mode)
/// Opcode: 06
pub fn asl_zp(state: &mut ComputerState) {
    let zp_addr = state.fetch_zero_page_address();
    let zp_val = state.mem[zp_addr];
    state.mem[zp_addr] = asl(state, zp_val);
}
/// ASL (zero_page X addressing mode)
/// Opcode: 16
pub fn asl_zpx(state: &mut ComputerState) {
    let zpx_addr = state.fetch_zero_page_x_address();
    let zpx_val = state.mem[zpx_addr];
    state.mem[zpx_addr] = asl(state, zpx_val);
}
/// ASL (absolute addressing mode)
/// Opcode: 0E
pub fn asl_ab(state: &mut ComputerState) {
    let ab_addr = state.fetch_absolute_address();
    let ab_val = state.mem[ab_addr];
    state.mem[ab_addr] = asl(state, ab_val);
}
/// ASL (absolute X addressing mode)
/// Opcode: 1E
pub fn asl_abx(state: &mut ComputerState) {
    let abx_addr = state.fetch_absolute_x_address();
    let abx_val = state.mem[abx_addr];
    state.mem[abx_addr] = asl(state, abx_val);
}


/// LSR (logical shift right)
fn lsr(state: &mut ComputerState, value: u8) -> u8 {
    let (result, overflow) = value.overflowing_shr(1);

    // FLAGS
    state.regs.sta =
        StatusRegister::C.get_cond(overflow) |
        get_zero_neg_flags(result);

    result
}
/// LSR (accumulator addressing mode)
/// Opcode: 4A
pub fn lsr_acc(state: &mut ComputerState) {
    state.regs.acc = lsr(state, state.regs.acc);
}
/// LSR (zero_page addressing mode)
/// Opcode: 46
pub fn lsr_zp(state: &mut ComputerState) {
    let zp_addr = state.fetch_zero_page_address();
    let zp_val = state.mem[zp_addr];
    state.mem[zp_addr] = lsr(state, zp_val);
}
/// LSR (zero_page X addressing mode)
/// Opcode: 56
pub fn lsr_zpx(state: &mut ComputerState) {
    let zpx_addr = state.fetch_zero_page_x_address();
    let zpx_val = state.mem[zpx_addr];
    state.mem[zpx_addr] = lsr(state, zpx_val);
}
/// LSR (absolute addressing mode)
/// Opcode: 4E
pub fn lsr_ab(state: &mut ComputerState) {
    let ab_addr = state.fetch_absolute_address();
    let ab_val = state.mem[ab_addr];
    state.mem[ab_addr] = lsr(state, ab_val);
}
/// LSR (absolute X addressing mode)
/// Opcode: 5E
pub fn lsr_abx(state: &mut ComputerState) {
    let abx_addr = state.fetch_absolute_x_address();
    let abx_val = state.mem[abx_addr];
    state.mem[abx_addr] = lsr(state, abx_val);
}


/// ROL (Rotate left one bit)
fn rol(_n: u8, _old_flags: StatusRegister) -> (u8, StatusRegister) {
    // TODO()
    (0, StatusRegister::new())
}


/// BIT (Bit test)
fn bit(_state: &mut ComputerState, _value: u8) {
    // TODO()
}
/// BIT (zero-page addressing mode)
/// Opcode: 24
pub fn bit_zp(state: &mut ComputerState) {
    let zp_val = state.fetch_zero_page();
    bit(state, zp_val)
}
/// BIT (absolute addressing mode)
/// Opcode: 2C
pub fn bit_ab(state: &mut ComputerState) {
    let ab_val = state.fetch_absolute();
    bit(state, ab_val)
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