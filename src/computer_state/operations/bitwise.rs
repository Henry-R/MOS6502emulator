use crate::computer_state::{ComputerState, StatusRegister};
use crate::computer_state::status_register::{get_zero_neg_flags};

// AND
/// AND (bitwise and)
const fn and(acc: u8, value: u8) -> (u8, StatusRegister) {
    let result = acc & value;
    (result, get_zero_neg_flags(result))
}

fn and_adapter(state: &mut ComputerState, addr_fn: fn(&mut ComputerState) -> u8) {
    let (result, flags) = and(state.regs.acc, addr_fn(state));
    state.regs.acc = result;
    state.regs.sta |= flags;
}

/// AND (intermediate addressing mode)
/// Opcode: 29
pub fn and_im(state: &mut ComputerState)
{ and_adapter(state, ComputerState::fetch_intermediate) }
/// AND (zero-page addressing mode)
/// Opcode: 25
pub fn and_zp(state: &mut ComputerState)
{ and_adapter(state, ComputerState::fetch_zero_page) }
/// AND (zero-page X addressing mode)
/// Opcode: 35
pub fn and_zpx(state: &mut ComputerState)
{ and_adapter(state, ComputerState::fetch_zero_page_x) }
/// AND (absolute addressing mode)
/// Opcode: 2D
pub fn and_ab(state: &mut ComputerState)
{ and_adapter(state, ComputerState::fetch_absolute) }
/// AND (absolute X addressing mode)
/// Opcode: 3D
pub fn and_abx(state: &mut ComputerState)
{ and_adapter(state, ComputerState::fetch_absolute_x) }
/// AND (absolute Y addressing mode)
/// Opcode: 39
pub fn and_aby(state: &mut ComputerState)
{ and_adapter(state, ComputerState::fetch_absolute_y) }
/// AND (indirect X addressing mode)
/// Opcode: 21
pub fn and_inx(state: &mut ComputerState)
{ and_adapter(state, ComputerState::fetch_indirect_x) }
/// AND (indirect Y addressing mode)
/// Opcode: 31
pub fn and_iny(state: &mut ComputerState)
{ and_adapter(state, ComputerState::fetch_indirect_y) }


/// OR (logical bitwise inclusive or)
const fn or(acc: u8, value: u8) -> (u8, StatusRegister) {
    let result = acc | value;
    (result, get_zero_neg_flags(result))
}

fn or_adapter(state: &mut ComputerState, addr_fn: fn(&mut ComputerState) -> u8) {
    let (result, flags) = or(state.regs.acc, addr_fn(state));
    state.regs.acc = result;
    state.regs.sta |= flags;
}

/// OR (intermediate addressing mode)
/// Opcode: 09
pub fn or_im(state: &mut ComputerState)
{ or_adapter(state, ComputerState::fetch_intermediate) }
/// OR (zero-page addressing mode)
/// Opcode: 05
pub fn or_zp(state: &mut ComputerState)
{ or_adapter(state, ComputerState::fetch_zero_page) }
/// OR (zero-page X addressing mode)
/// Opcode: 15
pub fn or_zpx(state: &mut ComputerState)
{ or_adapter(state, ComputerState::fetch_zero_page_x) }
/// OR (absolute addressing mode)
/// Opcode: 0D
pub fn or_ab(state: &mut ComputerState)
{ or_adapter(state, ComputerState::fetch_absolute) }
/// OR (absolute X addressing mode)
/// Opcode: 1D
pub fn or_abx(state: &mut ComputerState)
{ or_adapter(state, ComputerState::fetch_absolute_x) }
/// OR (absolute Y addressing mode)
/// Opcode: 19
pub fn or_aby(state: &mut ComputerState)
{ or_adapter(state, ComputerState::fetch_absolute_y) }
/// OR (indirect X addressing mode)
/// Opcode: 01
pub fn or_inx(state: &mut ComputerState)
{ or_adapter(state, ComputerState::fetch_indirect_x) }
/// OR (indirect Y addressing mode)
/// Opcode: 11
pub fn or_iny(state: &mut ComputerState)
{ or_adapter(state, ComputerState::fetch_indirect_y) }


/// EOR (logical bitwise exclusive or)
fn eor(acc: u8, value: u8) -> (u8, StatusRegister) {
    let result = acc ^ value;
    (result, get_zero_neg_flags(result))
}

fn eor_adapter(state: &mut ComputerState, addr_fn: fn(&mut ComputerState) -> u8) {
    let (result, flags) = eor(state.regs.acc, addr_fn(state));
    state.regs.acc = result;
    state.regs.sta |= flags;
}

/// EOR (intermediate addressing mode)
/// Opcode: 49
pub fn eor_im(state: &mut ComputerState)
{ eor_adapter(state, ComputerState::fetch_intermediate) }
/// EOR (zero-page addressing mode)
/// Opcode: 45
pub fn eor_zp(state: &mut ComputerState)
{ eor_adapter(state, ComputerState::fetch_zero_page) }
/// EOR (zero-page X addressing mode)
/// Opcode: 55
pub fn eor_zpx(state: &mut ComputerState)
{ eor_adapter(state, ComputerState::fetch_zero_page_x) }
/// EOR (absolute addressing mode)
/// Opcode: 4D
pub fn eor_ab(state: &mut ComputerState)
{ eor_adapter(state, ComputerState::fetch_absolute) }
/// EOR (absolute X addressing mode)
/// Opcode: 5D
pub fn eor_abx(state: &mut ComputerState)
{ eor_adapter(state, ComputerState::fetch_absolute_x) }
/// EOR (absolute Y addressing mode)
/// Opcode: 59
pub fn eor_aby(state: &mut ComputerState)
{ eor_adapter(state, ComputerState::fetch_absolute_y) }
/// EOR (indirect X addressing mode)
/// Opcode: 41
pub fn eor_inx(state: &mut ComputerState)
{ eor_adapter(state, ComputerState::fetch_indirect_x) }
/// EOR (indirect Y addressing mode)
/// Opcode: 51
pub fn eor_iny(state: &mut ComputerState)
{ eor_adapter(state, ComputerState::fetch_indirect_y) }


/// BIT (Bit test)
const fn bit(acc: u8, value: u8) -> StatusRegister {
    StatusRegister::Z.get_cond(acc & value == 0).union(
    StatusRegister::N.get_cond((value & 0x80) == 0x80).union(
    StatusRegister::V.get_cond((value & 0x40) == 0x40)))
}

fn bit_adapter(state: &mut ComputerState, addr_fn: fn(&mut ComputerState) -> u8) {
    let flags = bit(state.regs.acc, addr_fn(state));
    state.regs.sta |= flags;
}

/// BIT (zero-page addressing mode)
/// Opcode: 24
pub fn bit_zp(state: &mut ComputerState)
{ bit_adapter(state, ComputerState::fetch_zero_page) }
/// BIT (absolute addressing mode)
/// Opcode: 2C
pub fn bit_ab(state: &mut ComputerState)
{ bit_adapter(state, ComputerState::fetch_absolute) }


/// ASL (arithmetic shift left)
const fn asl(value: u8) -> (u8, StatusRegister) {
    let bits = value.count_ones();
    let result = value << 1;
    let overflow = bits != result.count_ones();

    let flags =
        StatusRegister::C.get_cond(overflow).union(
        get_zero_neg_flags(result));

    (result, flags)
}

fn asl_adapter(state: &mut ComputerState, addr_fn: fn(&mut ComputerState) -> usize) {
    let zp_addr = addr_fn(state);
    let zp_val = state.fetch_byte_from_addr(zp_addr);
    let (result, flags) = asl(zp_val);
    state.set_byte_at_addr(zp_addr, result);
    state.regs.sta |= flags;
}

/// ASL (accumulator addressing mode)
/// Opcode: 0A
pub fn asl_acc(state: &mut ComputerState) {
    let (result, flags) = asl(state.regs.acc);
    state.regs.acc = result;
    state.regs.sta |= flags;
}
/// ASL (zero_page addressing mode)
/// Opcode: 06
pub fn asl_zp(state: &mut ComputerState)
{ asl_adapter(state, ComputerState::fetch_zero_page_address) }
/// ASL (zero_page X addressing mode)
/// Opcode: 16
pub fn asl_zpx(state: &mut ComputerState)
{ asl_adapter(state, ComputerState::fetch_zero_page_x_address) }
/// ASL (absolute addressing mode)
/// Opcode: 0E
pub fn asl_ab(state: &mut ComputerState)
{ asl_adapter(state, ComputerState::fetch_absolute_address) }
/// ASL (absolute X addressing mode)
/// Opcode: 1E
pub fn asl_abx(state: &mut ComputerState)
{ asl_adapter(state, ComputerState::fetch_absolute_x_address) }


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

