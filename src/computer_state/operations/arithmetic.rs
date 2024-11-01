use crate::computer_state::{ComputerState, StatusRegister};
use crate::computer_state::status_register::{get_zero_neg_flags};

// ADDITION
/// ADC with carry
/// Adds two integers and returns their sum. If an overflow occurs, the C and V flags will be set
const fn add(acc: u8, n: u8, carry: u8) -> (u8, StatusRegister) {
    let (sum, overflow_n) = acc.overflowing_add(n);
    let (result, overflow_c) = sum.overflowing_add(carry);
    let overflowed = overflow_n || overflow_c;

    let flags =
        get_zero_neg_flags(sum).union(
        StatusRegister::C.get_cond(overflowed).union(
        StatusRegister::V.get_cond(
            (n < 0x7F && acc < 0x7F && result > 0x7F) ||
            (n > 0x7F && acc > 0x7F && result < 0x7F))));

    (result, flags)
}

/// Mutates the state of the computer according to the result of addition
/// Acts as an adapter between the implementation of add and the computer
fn add_adapter(state: &mut ComputerState, addressing_mode: fn(&mut ComputerState) -> u8) {
    let (result, flags) = add(state.regs.acc, addressing_mode(state), state.get_carry());

    state.regs.acc = result;
    state.regs.sta |= flags;
}

/// ADC (intermediate addressing mode)
/// Opcode: 69
pub fn add_im(state: &mut ComputerState) { add_adapter(state, ComputerState::fetch_intermediate); }
/// ADC (zero-page addressing mode)
/// Opcode: 65
pub fn add_zp(state: &mut ComputerState) { add_adapter(state, ComputerState::fetch_zero_page); }
/// ADC (zero-page X addressing mode)
/// Opcode: 75
pub fn add_zpx(state: &mut ComputerState) { add_adapter(state, ComputerState::fetch_zero_page_x); }
/// ADC (absolute addressing mode)
/// Opcode: 6D
pub fn add_ab(state: &mut ComputerState) { add_adapter(state, ComputerState::fetch_absolute); }
/// ADC (absolute X addressing mode)
/// Opcode: 7D
pub fn add_abx(state: &mut ComputerState) { add_adapter(state, ComputerState::fetch_absolute_x); }
/// ADC (absolute Y addressing mode)
/// Opcode: 79
pub fn add_aby(state: &mut ComputerState) { add_adapter(state, ComputerState::fetch_absolute_y); }
/// ADC (indirect X addressing mode)
/// Opcode: 61
pub fn add_inx(state: &mut ComputerState) { add_adapter(state, ComputerState::fetch_indirect_x);}
/// ADC (indirect Y addressing mode)
/// Opcode: 71
pub fn add_iny(state: &mut ComputerState) { add_adapter(state, ComputerState::fetch_indirect_y); }


// SUBTRACTION
/// SBC (subtraction with carry)
const fn sub(acc: u8, n: u8, carry: u8) -> (u8, StatusRegister) {
    // Use a - b is equivalent to a + (-b)
    let negative_n = 0xFF ^ n;
    add(acc, negative_n, carry)
}

/// Mutates the state of the computer according to the result of subtraction
/// Acts as an adapter between the implementation of sub and the computer
fn sub_adapter(state: &mut ComputerState, addressing_mode: fn(&mut ComputerState) -> u8) {
    let (result, flags) = sub(state.regs.acc, addressing_mode(state), state.get_carry());

    state.regs.acc = result;
    state.regs.sta |= flags;
}

/// SBC (intermediate addressing mode)
/// Opcode: E9
pub fn sub_im(state: &mut ComputerState) { sub_adapter(state, ComputerState::fetch_intermediate); }
/// SBC (zero-page addressing mode)
/// Opcode: E5
pub fn sub_zp(state: &mut ComputerState) { sub_adapter(state, ComputerState::fetch_zero_page); }
/// SBC (zero-page X addressing mode)
/// Opcode:F5
pub fn sub_zpx(state: &mut ComputerState) { sub_adapter(state, ComputerState::fetch_zero_page_x); }
/// SBC (absolute addressing mode)
/// Opcode: ED
pub fn sub_ab(state: &mut ComputerState) { sub_adapter(state, ComputerState::fetch_absolute); }
/// SBC (absolute X addressing mode)
/// Opcode: FD
pub fn sub_abx(state: &mut ComputerState) { sub_adapter(state, ComputerState::fetch_absolute_x); }
/// SBC (absolute Y addressing mode)
/// Opcode: F9
pub fn sub_aby(state: &mut ComputerState) { sub_adapter(state, ComputerState::fetch_absolute_y); }
/// SBC (indirect X addressing mode)
/// Opcode: E1
pub fn sub_inx(state: &mut ComputerState) { sub_adapter(state, ComputerState::fetch_indirect_x); }
/// SBC (indirect Y addressing mode)
/// Opcode: F1
pub fn sub_iny(state: &mut ComputerState) { sub_adapter(state, ComputerState::fetch_indirect_y); }


/// DEC (Decrement memory by one)
/// Returns tuple containing the new value to place in memory,
/// and the status flags after the operation has completed
fn dec(val: u8) -> (u8, StatusRegister) {
    // No documentation says this function can wrap
    let result = val - 1;
    let mut flags = StatusRegister::new();

    if result == i8::MAX as u8 { flags = flags | StatusRegister::Z }
    if result <  i8::MAX as u8 { flags = flags | StatusRegister::N }

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
fn inc(val: u8) -> (u8, StatusRegister) {
    // No documentation says this function can wrap
    let result = val + 1;
    let mut flags = StatusRegister::new();

    if result == i8::MAX as u8 { flags = flags | StatusRegister::Z }
    if result <  i8::MAX as u8 { flags = flags | StatusRegister::N }

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
    use crate::computer_state::StatusRegister;
    use crate::computer_state::operations::arithmetic::{add, dec, sub};

    #[test]
    fn test_add() {
        // Normal add
        let (result, flags) = add(0, 10, 0);
        assert_eq!(result, 10);
        assert!(flags.is_empty());

        // Zero
        let (result, flags) = add(0, 0, 0);
        assert_eq!(result, 0);
        assert!(flags.difference(StatusRegister::Z).is_empty());

        // Negative
        let (result, flags) = add(0, 200, 0);
        assert_eq!(result, 200);
        assert!(flags.difference(StatusRegister::N).is_empty());

        // Negative and carry
        let (result, flags) = add(1, 0xFF, 0);
        assert_eq!(result, 0);
        assert!(flags.difference(StatusRegister::Z | StatusRegister::C).is_empty());

        // Negative and overflow
        let (result, flags) = add(32, 120, 0);
        assert_eq!(result, 152);
        assert!(flags.difference(StatusRegister::N | StatusRegister::V).is_empty());

        // Carry and overflow
        let (result, flags) = add(144, 208, 0);
        assert_eq!(result, 96);
        assert!(flags.difference(StatusRegister::C | StatusRegister::V).is_empty());
    }
    #[test]
    fn test_sub() {
        let (result, flags) = sub(0, 100, 0);
        assert_eq!(result, 156);
        assert!(flags.contains(StatusRegister::C));

        let (result, flags) = sub(result, 100, 0);
        assert!(flags.contains(StatusRegister::N));

        let (result, flags) = sub(result, 56, 0);
        assert_eq!(result, 0);
        assert!(flags.contains(StatusRegister::Z));
    }

    #[test]
    fn test_dec() {
        let (val, flags) = dec(128);
        assert!(flags.contains(StatusRegister::Z));
        assert_eq!(127, val);
        let (val, flags) = dec(127);
        assert!(flags.contains(StatusRegister::N));
        assert_eq!(126, val);
    }
}