use crate::computer_state::{ComputerState, StatusRegister};
use crate::computer_state::status_register::{get_zero_neg_flags};

// ADDITION
/// ADC (addition with carry)
const fn add(acc: u8, n: u8, carry: u8) -> (u8, StatusRegister) {
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
fn add_adapter(state: &mut ComputerState, addr_fn: fn(&mut ComputerState) -> u8) {
    let (result, flags) = add(state.regs.acc, addr_fn(state), state.get_carry());

    state.regs.acc = result;
    state.regs.sta |= flags;
}

/// ADC (intermediate addressing mode)
/// Opcode: 69
pub fn add_im(state: &mut ComputerState)
{ add_adapter(state, ComputerState::fetch_intermediate); }
/// ADC (zero-page addressing mode)
/// Opcode: 65
pub fn add_zp(state: &mut ComputerState)
{ add_adapter(state, ComputerState::fetch_zero_page); }
/// ADC (zero-page X addressing mode)
/// Opcode: 75
pub fn add_zpx(state: &mut ComputerState)
{ add_adapter(state, ComputerState::fetch_zero_page_x); }
/// ADC (absolute addressing mode)
/// Opcode: 6D
pub fn add_ab(state: &mut ComputerState)
{ add_adapter(state, ComputerState::fetch_absolute); }
/// ADC (absolute X addressing mode)
/// Opcode: 7D
pub fn add_abx(state: &mut ComputerState)
{ add_adapter(state, ComputerState::fetch_absolute_x); }
/// ADC (absolute Y addressing mode)
/// Opcode: 79
pub fn add_aby(state: &mut ComputerState)
{ add_adapter(state, ComputerState::fetch_absolute_y); }
/// ADC (indirect X addressing mode)
/// Opcode: 61
pub fn add_inx(state: &mut ComputerState)
{ add_adapter(state, ComputerState::fetch_indirect_x);}
/// ADC (indirect Y addressing mode)
/// Opcode: 71
pub fn add_iny(state: &mut ComputerState)
{ add_adapter(state, ComputerState::fetch_indirect_y); }


// SUBTRACTION
/// SBC (subtraction with carry)
const fn sub(acc: u8, n: u8, carry: u8) -> (u8, StatusRegister) {
    // Use a - b is equivalent to a + (-b)
    let negative_n = 0xFF ^ n;
    // Inversion of carry is used to indicate a borrow
    let negative_carry = if carry == 1 { 0 } else { 1 };
    add(acc, negative_n, negative_carry)
}

/// Mutates the state of the computer according to the result of subtraction
/// Acts as an adapter between the implementation of sub and the computer
fn sub_adapter(state: &mut ComputerState, addr_fn: fn(&mut ComputerState) -> u8) {
    let (result, flags) = sub(state.regs.acc, addr_fn(state), state.get_carry());

    state.regs.acc = result;
    state.regs.sta |= flags;
}

/// SBC (intermediate addressing mode)
/// Opcode: E9
pub fn sub_im(state: &mut ComputerState)
{ sub_adapter(state, ComputerState::fetch_intermediate); }
/// SBC (zero-page addressing mode)
/// Opcode: E5
pub fn sub_zp(state: &mut ComputerState)
{ sub_adapter(state, ComputerState::fetch_zero_page); }
/// SBC (zero-page X addressing mode)
/// Opcode:F5
pub fn sub_zpx(state: &mut ComputerState)
{ sub_adapter(state, ComputerState::fetch_zero_page_x); }
/// SBC (absolute addressing mode)
/// Opcode: ED
pub fn sub_ab(state: &mut ComputerState)
{ sub_adapter(state, ComputerState::fetch_absolute); }
/// SBC (absolute X addressing mode)
/// Opcode: FD
pub fn sub_abx(state: &mut ComputerState)
{ sub_adapter(state, ComputerState::fetch_absolute_x); }
/// SBC (absolute Y addressing mode)
/// Opcode: F9
pub fn sub_aby(state: &mut ComputerState)
{ sub_adapter(state, ComputerState::fetch_absolute_y); }
/// SBC (indirect X addressing mode)
/// Opcode: E1
pub fn sub_inx(state: &mut ComputerState)
{ sub_adapter(state, ComputerState::fetch_indirect_x); }
/// SBC (indirect Y addressing mode)
/// Opcode: F1
pub fn sub_iny(state: &mut ComputerState)
{ sub_adapter(state, ComputerState::fetch_indirect_y); }


/// DEC (Decrement memory by one)
/// Returns tuple containing the new value to place in memory,
/// and the status flags after the operation has completed
const fn dec(val: u8) -> (u8, StatusRegister) {
    let result = val - 1;
    (result, get_zero_neg_flags(result))
}

/// Mutates the state of the computer according to the result of taking the decrement
/// Acts as an adapter between the implementation of dec and the computer
fn dec_adapter(state: &mut ComputerState, addr_fn: fn(&mut ComputerState) -> usize) {
    let addr = addr_fn(state);
    let (result, flags) = dec(state.get_addr(addr));
    state.set_addr(result, addr);
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
{ dec_adapter(state, ComputerState::fetch_absolute_address_x) }

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
    let result = val + 1;
    (result, get_zero_neg_flags(result))
}

/// Mutates the state of the computer according to the result of taking the increment
/// Acts as an adapter between the implementation of inc and the computer
fn inc_adapter(state: &mut ComputerState, addr_fn: fn(&mut ComputerState) -> usize) {
    let addr = addr_fn(state);
    let (result, flags) = inc(state.get_addr(addr));
    state.set_addr(result, addr);
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
{ inc_adapter(state, ComputerState::fetch_absolute_address_x) }

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
        assert!(flags.contains_only(StatusRegister::Z));

        // Negative
        let (result, flags) = add(0, 200, 0);
        assert_eq!(result, 200);
        assert!(flags.contains_only(StatusRegister::N));

        // Negative and carry
        let (result, flags) = add(1, 0xFF, 0);
        assert_eq!(result, 0);
        assert!(flags.contains_only(StatusRegister::Z | StatusRegister::C));

        // Negative and overflow
        let (result, flags) = add(32, 120, 0);
        assert_eq!(result, 152);
        assert!(flags.contains_only(StatusRegister::N | StatusRegister::V));

        // Carry and overflow
        let (result, flags) = add(144, 208, 0);
        assert_eq!(result, 96);
        assert!(flags.contains_only(StatusRegister::C | StatusRegister::V));

        // Using carry flag
        let (result, flags) = add(0, 10, 1);
        assert_eq!(result, 11);
        assert!(flags.is_empty());
    }
    #[test]
    fn test_sub() {
        // No borrow
        let (result, flags) = sub(0x50, 0x20, 0);
        assert_eq!(result, 0x30);
        assert!(flags.contains_only(StatusRegister::C));

        // Borrow
        let (result, flags) = sub(0xD0, 0xF0, 0);
        assert_eq!(result, 0xE0);
        assert!(flags.contains_only(StatusRegister::N));

        // Overflow
        let (result, flags) = sub(0x50, 0xB0, 0);
        assert_eq!(result, 0xA0);
        assert!(flags.contains_only(StatusRegister::N | StatusRegister::V));

        // Zero
        let (result, flags) = sub(50, 50, 0);
        assert_eq!(result, 0);
        assert!(flags.contains_only(StatusRegister::Z | StatusRegister::C));

        // Using carry flag
        let (result, flags) = sub(11, 10, 1);
        assert_eq!(result, 0);
        assert!(flags.contains_only(StatusRegister::Z | StatusRegister::C));
    }

    #[test]
    fn test_dec() {

    }
}