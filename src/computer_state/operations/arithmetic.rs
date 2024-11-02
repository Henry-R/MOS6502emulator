use crate::computer_state::{ComputerState, StatusRegister};
use crate::computer_state::status_register::{get_zero_neg_flags};

// ADDITION
/// ADC (addition with carry)
const fn adc(acc: u8, n: u8, carry: u8) -> (u8, StatusRegister) {
    // Could overflow during regular addition or carry
    let (sum, overflow_n) = acc.overflowing_add(n);
    let (result, overflow_c) = sum.overflowing_add(carry);
    let overflowed = overflow_n || overflow_c;

    let flags =
        get_zero_neg_flags(result).union(
        StatusRegister::C.get_cond(overflowed).union(
            // Overflowed from negative to positive or vice versa
        StatusRegister::V.get_cond(
            (n < 0x7F && acc < 0x7F && result > 0x7F) ||
            (n > 0x7F && acc > 0x7F && result < 0x7F))));

    (result, flags)
}

/// Mutates the state of the computer according to the result of addition
/// Acts as an adapter between the implementation of add and the computer
fn adc_adapter(state: &mut ComputerState, addr_fn: fn(&mut ComputerState) -> u8) {
    let (result, flags) = adc(state.regs.acc, addr_fn(state), state.get_carry());

    state.regs.acc = result;
    state.regs.sta |= flags;
}

/// ADC (intermediate addressing mode)
/// Opcode: 69
pub fn adc_im(state: &mut ComputerState)
{ adc_adapter(state, ComputerState::fetch_intermediate); }
/// ADC (zero-page addressing mode)
/// Opcode: 65
pub fn adc_zp(state: &mut ComputerState)
{ adc_adapter(state, ComputerState::fetch_zero_page); }
/// ADC (zero-page X addressing mode)
/// Opcode: 75
pub fn adc_zpx(state: &mut ComputerState)
{ adc_adapter(state, ComputerState::fetch_zero_page_x); }
/// ADC (absolute addressing mode)
/// Opcode: 6D
pub fn adc_ab(state: &mut ComputerState)
{ adc_adapter(state, ComputerState::fetch_absolute); }
/// ADC (absolute X addressing mode)
/// Opcode: 7D
pub fn adc_abx(state: &mut ComputerState)
{ adc_adapter(state, ComputerState::fetch_absolute_x); }
/// ADC (absolute Y addressing mode)
/// Opcode: 79
pub fn adc_aby(state: &mut ComputerState)
{ adc_adapter(state, ComputerState::fetch_absolute_y); }
/// ADC (indirect X addressing mode)
/// Opcode: 61
pub fn adc_inx(state: &mut ComputerState)
{ adc_adapter(state, ComputerState::fetch_indirect_x);}
/// ADC (indirect Y addressing mode)
/// Opcode: 71
pub fn adc_iny(state: &mut ComputerState)
{ adc_adapter(state, ComputerState::fetch_indirect_y); }


// SUBTRACTION
/// SBC (subtraction with carry)
const fn sbc(acc: u8, n: u8, carry: u8) -> (u8, StatusRegister) {
    // Use a - b is equivalent to a + (-b)
    let negative_n = 0xFF ^ n;
    // Inversion of carry is used to indicate a borrow
    let negative_carry = if carry == 1 { 0 } else { 1 };
    adc(acc, negative_n, negative_carry)
}

/// Mutates the state of the computer according to the result of subtraction
/// Acts as an adapter between the implementation of sub and the computer
fn sbc_adapter(state: &mut ComputerState, addr_fn: fn(&mut ComputerState) -> u8) {
    let (result, flags) = sbc(state.regs.acc, addr_fn(state), state.get_carry());

    state.regs.acc = result;
    state.regs.sta |= flags;
}

/// SBC (intermediate addressing mode)
/// Opcode: E9
pub fn sbc_im(state: &mut ComputerState)
{ sbc_adapter(state, ComputerState::fetch_intermediate); }
/// SBC (zero-page addressing mode)
/// Opcode: E5
pub fn sbc_zp(state: &mut ComputerState)
{ sbc_adapter(state, ComputerState::fetch_zero_page); }
/// SBC (zero-page X addressing mode)
/// Opcode:F5
pub fn sbc_zpx(state: &mut ComputerState)
{ sbc_adapter(state, ComputerState::fetch_zero_page_x); }
/// SBC (absolute addressing mode)
/// Opcode: ED
pub fn sbc_ab(state: &mut ComputerState)
{ sbc_adapter(state, ComputerState::fetch_absolute); }
/// SBC (absolute X addressing mode)
/// Opcode: FD
pub fn sbc_abx(state: &mut ComputerState)
{ sbc_adapter(state, ComputerState::fetch_absolute_x); }
/// SBC (absolute Y addressing mode)
/// Opcode: F9
pub fn sbc_aby(state: &mut ComputerState)
{ sbc_adapter(state, ComputerState::fetch_absolute_y); }
/// SBC (indirect X addressing mode)
/// Opcode: E1
pub fn sbc_inx(state: &mut ComputerState)
{ sbc_adapter(state, ComputerState::fetch_indirect_x); }
/// SBC (indirect Y addressing mode)
/// Opcode: F1
pub fn sbc_iny(state: &mut ComputerState)
{ sbc_adapter(state, ComputerState::fetch_indirect_y); }


/// DEC (Decrement memory by one)
/// Returns tuple containing the new value to place in memory,
/// and the status flags after the operation has completed
const fn dec(val: u8) -> (u8, StatusRegister) {
    let result = val.wrapping_sub(1);
    (result, get_zero_neg_flags(result))
}

/// Mutates the state of the computer according to the result of taking the decrement
/// Acts as an adapter between the implementation of dec and the computer
fn dec_adapter(state: &mut ComputerState, addr_fn: fn(&mut ComputerState) -> usize) {
    let addr = addr_fn(state);
    let (result, flags) = dec(state.fetch_byte_from_addr(addr));
    state.set_byte_at_addr(addr, result);
    state.regs.sta |= flags;
}

/// DEC (zero-page addressing mode)
/// Opcode: C6
pub fn dec_zp(state: &mut ComputerState)
{ dec_adapter(state, ComputerState::fetch_zero_page_address) }
/// DEC (zero-page X addressing mode)
/// Opcode: D6
pub fn dec_zpx(state: &mut ComputerState)
{ dec_adapter(state, ComputerState::fetch_zero_page_x_address) }
/// DEC (absolute addressing mode)
/// Opcode: CE
pub fn dec_ab(state: &mut ComputerState)
{ dec_adapter(state, ComputerState::fetch_absolute_address) }
/// DEC (absolute X addressing mode)
/// Opcode: DE
pub fn dec_abx(state: &mut ComputerState)
{ dec_adapter(state, ComputerState::fetch_absolute_x_address) }

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
const fn inc(val: u8) -> (u8, StatusRegister) {
    let result = val.wrapping_add(1);
    (result, get_zero_neg_flags(result))
}

/// Mutates the state of the computer according to the result of taking the increment
/// Acts as an adapter between the implementation of inc and the computer
fn inc_adapter(state: &mut ComputerState, addr_fn: fn(&mut ComputerState) -> usize) {
    let addr = addr_fn(state);
    let (result, flags) = inc(state.fetch_byte_from_addr(addr));
    state.set_byte_at_addr(addr, result);
    state.regs.sta |= flags;
}

/// INC (zero-page addressing mode)
/// Opcode: E6
pub fn inc_zp(state: &mut ComputerState)
{ inc_adapter(state, ComputerState::fetch_zero_page_address) }
/// INC (zero-page X addressing mode)
/// Opcode: F6
pub fn inc_zpx(state: &mut ComputerState)
{ inc_adapter(state, ComputerState::fetch_zero_page_x_address) }
/// INC (absolute addressing mode)
/// Opcode: EE
pub fn inc_ab(state: &mut ComputerState)
{ inc_adapter(state, ComputerState::fetch_absolute_address) }
/// INC (absolute X addressing mode)
/// Opcode: FE
pub fn inc_abx(state: &mut ComputerState)
{ inc_adapter(state, ComputerState::fetch_absolute_x_address) }

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
