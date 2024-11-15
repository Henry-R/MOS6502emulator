use crate::computer_state::{ComputerState, StatusRegister};
use crate::computer_state::memory::Memory;
use crate::computer_state::status_register::{get_zero_neg_flags};

// AND
/// AND (bitwise and)
const fn and(acc: u8, value: u8) -> (u8, StatusRegister) {
    let result = acc & value;
    (result, get_zero_neg_flags(result))
}

/// Mutates the state of the computer according to the result of logical and
/// Acts as an adapter between the implementation of 'AND' and the computer
fn and_adapter(state: &mut ComputerState, addr_fn: fn(&mut Memory) -> u8) {
    let (result, flags) = and(state.acc.get(), addr_fn(&mut state.mem));
    state.acc.set(result);
    state.sta |= flags;
}

/// AND (intermediate addressing mode)
/// Opcode: 29
pub fn and_im(state: &mut ComputerState)
{ and_adapter(state, Memory::fetch_immediate) }
/// AND (zero-page addressing mode)
/// Opcode: 25
pub fn and_zp(state: &mut ComputerState)
{ and_adapter(state, Memory::fetch_zero_page) }
/// AND (zero-page X addressing mode)
/// Opcode: 35
pub fn and_zpx(state: &mut ComputerState)
{ and_adapter(state, Memory::fetch_zero_page_x) }
/// AND (absolute addressing mode)
/// Opcode: 2D
pub fn and_ab(state: &mut ComputerState)
{ and_adapter(state, Memory::fetch_absolute) }
/// AND (absolute X addressing mode)
/// Opcode: 3D
pub fn and_abx(state: &mut ComputerState)
{ and_adapter(state, Memory::fetch_absolute_x) }
/// AND (absolute Y addressing mode)
/// Opcode: 39
pub fn and_aby(state: &mut ComputerState)
{ and_adapter(state, Memory::fetch_absolute_y) }
/// AND (indirect X addressing mode)
/// Opcode: 21
pub fn and_inx(state: &mut ComputerState)
{ and_adapter(state, Memory::fetch_indirect_x) }
/// AND (indirect Y addressing mode)
/// Opcode: 31
pub fn and_iny(state: &mut ComputerState)
{ and_adapter(state, Memory::fetch_indirect_y) }


/// OR (logical bitwise inclusive or)
const fn or(acc: u8, value: u8) -> (u8, StatusRegister) {
    let result = acc | value;
    (result, get_zero_neg_flags(result))
}

/// Mutates the state of the computer according to the result of logical or
/// Acts as an adapter between the implementation of 'OR' and the computer
fn or_adapter(state: &mut ComputerState, addr_fn: fn(&mut Memory) -> u8) {
    let (result, flags) = or(state.acc.get(), addr_fn(&mut state.mem));
    state.acc.set(result);
    state.sta |= flags;
}

/// OR (intermediate addressing mode)
/// Opcode: 09
pub fn or_im(state: &mut ComputerState)
{ or_adapter(state, Memory::fetch_immediate) }
/// OR (zero-page addressing mode)
/// Opcode: 05
pub fn or_zp(state: &mut ComputerState)
{ or_adapter(state, Memory::fetch_zero_page) }
/// OR (zero-page X addressing mode)
/// Opcode: 15
pub fn or_zpx(state: &mut ComputerState)
{ or_adapter(state, Memory::fetch_zero_page_x) }
/// OR (absolute addressing mode)
/// Opcode: 0D
pub fn or_ab(state: &mut ComputerState)
{ or_adapter(state, Memory::fetch_absolute) }
/// OR (absolute X addressing mode)
/// Opcode: 1D
pub fn or_abx(state: &mut ComputerState)
{ or_adapter(state, Memory::fetch_absolute_x) }
/// OR (absolute Y addressing mode)
/// Opcode: 19
pub fn or_aby(state: &mut ComputerState)
{ or_adapter(state, Memory::fetch_absolute_y) }
/// OR (indirect X addressing mode)
/// Opcode: 01
pub fn or_inx(state: &mut ComputerState)
{ or_adapter(state, Memory::fetch_indirect_x) }
/// OR (indirect Y addressing mode)
/// Opcode: 11
pub fn or_iny(state: &mut ComputerState)
{ or_adapter(state, Memory::fetch_indirect_y) }


/// EOR (logical bitwise exclusive or)
fn eor(acc: u8, value: u8) -> (u8, StatusRegister) {
    let result = acc ^ value;
    (result, get_zero_neg_flags(result))
}

/// Mutates the state of the computer according to the result of logical exclusive or
/// Acts as an adapter between the implementation of 'XOR' and the computer
fn eor_adapter(state: &mut ComputerState, addr_fn: fn(&mut Memory) -> u8) {
    let (result, flags) = eor(state.acc.get(), addr_fn(&mut state.mem));
    state.acc.set(result);
    state.sta |= flags;
}

/// EOR (intermediate addressing mode)
/// Opcode: 49
pub fn eor_im(state: &mut ComputerState)
{ eor_adapter(state, Memory::fetch_immediate) }
/// EOR (zero-page addressing mode)
/// Opcode: 45
pub fn eor_zp(state: &mut ComputerState)
{ eor_adapter(state, Memory::fetch_zero_page) }
/// EOR (zero-page X addressing mode)
/// Opcode: 55
pub fn eor_zpx(state: &mut ComputerState)
{ eor_adapter(state, Memory::fetch_zero_page_x) }
/// EOR (absolute addressing mode)
/// Opcode: 4D
pub fn eor_ab(state: &mut ComputerState)
{ eor_adapter(state, Memory::fetch_absolute) }
/// EOR (absolute X addressing mode)
/// Opcode: 5D
pub fn eor_abx(state: &mut ComputerState)
{ eor_adapter(state, Memory::fetch_absolute_x) }
/// EOR (absolute Y addressing mode)
/// Opcode: 59
pub fn eor_aby(state: &mut ComputerState)
{ eor_adapter(state, Memory::fetch_absolute_y) }
/// EOR (indirect X addressing mode)
/// Opcode: 41
pub fn eor_inx(state: &mut ComputerState)
{ eor_adapter(state, Memory::fetch_indirect_x) }
/// EOR (indirect Y addressing mode)
/// Opcode: 51
pub fn eor_iny(state: &mut ComputerState)
{ eor_adapter(state, Memory::fetch_indirect_y) }


/// BIT (Bit test)
const fn bit(acc: u8, value: u8) -> StatusRegister {
    StatusRegister::Z.get_cond(acc & value == 0).union(
    StatusRegister::N.get_cond((value & 0x80) == 0x80).union(
    StatusRegister::V.get_cond((value & 0x40) == 0x40)))
}

/// Mutates the state of the computer according to the result of the bit test
/// Acts as an adapter between the implementation of 'BIT' and the computer
fn bit_adapter(state: &mut ComputerState, addr_fn: fn(&mut Memory) -> u8) {
    let flags = bit(state.acc.get(), addr_fn(&mut state.mem));
    state.sta |= flags;
}

/// BIT (zero-page addressing mode)
/// Opcode: 24
pub fn bit_zp(state: &mut ComputerState)
{ bit_adapter(state, Memory::fetch_zero_page) }
/// BIT (absolute addressing mode)
/// Opcode: 2C
pub fn bit_ab(state: &mut ComputerState)
{ bit_adapter(state, Memory::fetch_absolute) }


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

/// Mutates the state of the computer according to the result of an arithmetic shift left
/// Acts as an adapter between the implementation of 'ASL' and the computer
fn asl_adapter(state: &mut ComputerState, addr_fn: fn(&mut Memory) -> usize) {
    let zp_addr = addr_fn(&mut state.mem);
    let zp_val = state.mem.fetch_byte_from_addr(zp_addr);
    let (result, flags) = asl(zp_val);
    state.mem.set_byte_at_addr(zp_addr, result);
    state.sta |= flags;
}

/// ASL (accumulator addressing mode)
/// Opcode: 0A
pub fn asl_acc(state: &mut ComputerState) {
    let (result, flags) = asl(state.acc.get());
    state.acc.set(result);
    state.sta |= flags;
}
/// ASL (zero_page addressing mode)
/// Opcode: 06
pub fn asl_zp(state: &mut ComputerState)
{ asl_adapter(state, Memory::fetch_zero_page_address) }
/// ASL (zero_page X addressing mode)
/// Opcode: 16
pub fn asl_zpx(state: &mut ComputerState)
{ asl_adapter(state, Memory::fetch_zero_page_x_address) }
/// ASL (absolute addressing mode)
/// Opcode: 0E
pub fn asl_ab(state: &mut ComputerState)
{ asl_adapter(state, Memory::fetch_absolute_address) }
/// ASL (absolute X addressing mode)
/// Opcode: 1E
pub fn asl_abx(state: &mut ComputerState)
{ asl_adapter(state, Memory::fetch_absolute_x_address) }


/// LSR (logical shift right)
const fn lsr(value: u8) -> (u8, StatusRegister) {
    let bits = value.count_ones();
    let result = value >> 1;
    let overflow = bits != result.count_ones();

    let flags =
        StatusRegister::C.get_cond(overflow).union(
            get_zero_neg_flags(result));

    (result, flags)
}

/// Mutates the state of the computer according to the result of a logical shift right
/// Acts as an adapter between the implementation of 'LSR' and the computer
fn lsr_adapter(state: &mut ComputerState, addr_fn: fn(&mut Memory) -> usize) {
    let zp_addr = addr_fn(&mut state.mem);
    let zp_val = state.mem.fetch_byte_from_addr(zp_addr);
    let (result, flags) = lsr(zp_val);
    state.mem.set_byte_at_addr(zp_addr, result);
    state.sta |= flags;
}

/// LSR (accumulator addressing mode)
/// Opcode: 4A
pub fn lsr_acc(state: &mut ComputerState) {
    let (result, flags) = lsr(state.acc.get());
    state.acc.set(result);
    state.sta |= flags;
}
/// LSR (zero_page addressing mode)
/// Opcode: 46
pub fn lsr_zp(state: &mut ComputerState)
{ lsr_adapter(state, Memory::fetch_zero_page_address) }
/// LSR (zero_page X addressing mode)
/// Opcode: 56
pub fn lsr_zpx(state: &mut ComputerState)
{ lsr_adapter(state, Memory::fetch_zero_page_x_address) }
/// LSR (absolute addressing mode)
/// Opcode: 4E
pub fn lsr_ab(state: &mut ComputerState)
{ lsr_adapter(state, Memory::fetch_absolute_address) }
/// LSR (absolute X addressing mode)
/// Opcode: 5E
pub fn lsr_abx(state: &mut ComputerState)
{ lsr_adapter(state, Memory::fetch_absolute_x_address) }


/// ROL (Rotate left one bit)
const fn rol(value: u8, carry: u8) -> (u8, StatusRegister) {
    let result = (value << 1) + carry;
    let flags = get_zero_neg_flags(result).union(
        StatusRegister::C.get_cond((value & 0x80) != 0));

    (result, flags)
}

/// Mutates the state of the computer according to the result of a left rotation
/// Acts as an adapter between the implementation of 'ROL' and the computer
fn rol_adapter(state: &mut ComputerState, addr_fn: fn(&mut Memory) -> usize) {
    let carry = state.get_carry();
    let zp_addr = addr_fn(&mut state.mem);
    let zp_val = state.mem.fetch_byte_from_addr(zp_addr);
    let (result, flags) = rol(zp_val, carry);

    state.mem.set_byte_at_addr(zp_addr, result);
    state.sta = state.sta.difference(StatusRegister::C);
    state.sta |= flags;
}

/// ROL (accumulator addressing mode)
/// Opcode: 2A
pub fn rol_acc(state: &mut ComputerState) {
    let carry = state.get_carry();
    let (result, flags) = rol(state.acc.get(), carry);

    state.acc.set(result);
    state.sta = state.sta.difference(StatusRegister::C);
    state.sta |= flags;
}
/// ROL (zero_page addressing mode)
/// Opcode: 26
pub fn rol_zp(state: &mut ComputerState)
{ rol_adapter(state, Memory::fetch_zero_page_address) }
/// ROL (zero_page X addressing mode)
/// Opcode: 36
pub fn rol_zpx(state: &mut ComputerState)
{ rol_adapter(state, Memory::fetch_zero_page_x_address) }
/// ROL (absolute addressing mode)
/// Opcode: 2E
pub fn rol_ab(state: &mut ComputerState)
{ rol_adapter(state, Memory::fetch_absolute_address) }
/// ROL (absolute X addressing mode)
/// Opcode: 3E
pub fn rol_abx(state: &mut ComputerState)
{ rol_adapter(state, Memory::fetch_absolute_x_address) }


/// ROR (Rotate right one bit)
const fn ror(value: u8, carry: u8) -> (u8, StatusRegister) {
    let result = (value >> 1) + (carry << 7);
    let flags = get_zero_neg_flags(result).union(
        StatusRegister::C.get_cond((value & 0x01) != 0));

    (result, flags)
}

/// Mutates the state of the computer according to the result of a right rotation
/// Acts as an adapter between the implementation of 'ROR' and the computer
fn ror_adapter(state: &mut ComputerState, addr_fn: fn(&mut Memory) -> usize) {
    let carry = state.get_carry();
    let zp_addr = addr_fn(&mut state.mem);
    let zp_val = state.mem.fetch_byte_from_addr(zp_addr);
    let (result, flags) = ror(zp_val, carry);

    state.mem.set_byte_at_addr(zp_addr, result);
    state.sta = state.sta.difference(StatusRegister::C);
    state.sta |= flags;
}

/// ROR (accumulator addressing mode)
/// Opcode: 6A
pub fn ror_acc(state: &mut ComputerState) {
    let carry = state.get_carry();
    let (result, flags) = ror(state.acc.get(), carry);

    state.acc.set(result);
    state.sta = state.sta.difference(StatusRegister::C);
    state.sta |= flags;
}
/// ROR (zero_page addressing mode)
/// Opcode: 66
pub fn ror_zp(state: &mut ComputerState)
{ ror_adapter(state, Memory::fetch_zero_page_address) }
/// ROR (zero_page X addressing mode)
/// Opcode: 76
pub fn ror_zpx(state: &mut ComputerState)
{ ror_adapter(state, Memory::fetch_zero_page_x_address) }
/// ROR (absolute addressing mode)
/// Opcode: 6E
pub fn ror_ab(state: &mut ComputerState)
{ ror_adapter(state, Memory::fetch_absolute_address) }
/// ROR (absolute X addressing mode)
/// Opcode: 7E
pub fn ror_abx(state: &mut ComputerState)
{ ror_adapter(state, Memory::fetch_absolute_x_address) }