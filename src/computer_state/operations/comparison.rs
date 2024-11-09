use crate::computer_state::ComputerState;
use crate::computer_state::status_register::StatusRegister;

const fn cmp(val: u8, mem: u8) -> StatusRegister {
    StatusRegister::C.get_cond(val >= mem).union(
    StatusRegister::Z.get_cond(val == mem).union(
    StatusRegister::N.get_cond(val <  mem)))
}


/// CMP (immediate memory addressing mode)
/// Opcode: C9
pub fn cmp_im(state: &mut ComputerState)
{ state.sta |= cmp(state.acc.get(), state.mem.fetch_immediate()); }

/// CMP (zero-page memory addressing mode)
/// Opcode: C5
pub fn cmp_zp(state: &mut ComputerState)
{ state.sta |= cmp(state.acc.get(), state.mem.fetch_zero_page()); }

/// CMP (zero-page X memory addressing mode)
/// Opcode: D5
pub fn cmp_zpx(state: &mut ComputerState)
{ state.sta |= cmp(state.acc.get(), state.mem.fetch_zero_page_x()); }

/// CMP (absolute memory addressing mode)
/// Opcode: CD
pub fn cmp_ab(state: &mut ComputerState)
{ state.sta |= cmp(state.acc.get(), state.mem.fetch_absolute()); }

/// CMP (absolute X memory addressing mode)
/// Opcode: DD
pub fn cmp_abx(state: &mut ComputerState)
{ state.sta |= cmp(state.acc.get(), state.mem.fetch_absolute_x()); }

/// CMP (absolute Y memory addressing mode)
/// Opcode: D9
pub fn cmp_aby(state: &mut ComputerState)
{ state.sta |= cmp(state.acc.get(), state.mem.fetch_absolute_y()); }

/// CMP (indirect X memory addressing mode)
/// Opcode: C1
pub fn cmp_inx(state: &mut ComputerState)
{ state.sta |= cmp(state.acc.get(), state.mem.fetch_indirect_x()); }

/// CMP (indirect Y memory addressing mode)
/// Opcode: D1
pub fn cmp_iny(state: &mut ComputerState)
{ state.sta |= cmp(state.acc.get(), state.mem.fetch_indirect_y()); }


/// CPX (immediate memory addressing mode)
/// Opcode: E0
pub fn cpx_im(state: &mut ComputerState)
{ state.sta |= cmp(state.get_x() as u8, state.mem.fetch_immediate()); }

/// CPX (zero-page memory addressing mode)
/// Opcode: E4
pub fn cpx_zp(state: &mut ComputerState)
{ state.sta |= cmp(state.get_x() as u8, state.mem.fetch_zero_page()); }

/// CPX (absolute memory addressing mode)
/// Opcode: EC
pub fn cpx_ab(state: &mut ComputerState)
{ state.sta |= cmp(state.get_x() as u8, state.mem.fetch_absolute()); }


/// CPY (immediate memory addressing mode)
/// Opcode: C0
pub fn cpy_im(state: &mut ComputerState)
{ state.sta |= cmp(state.get_y() as u8, state.mem.fetch_immediate()); }

/// CPY (zero-page memory addressing mode)
/// Opcode: C4
pub fn cpy_zp(state: &mut ComputerState)
{ state.sta |= cmp(state.get_y() as u8, state.mem.fetch_zero_page()); }

/// CPY (absolute memory addressing mode)
/// Opcode: CC
pub fn cpy_ab(state: &mut ComputerState)
{ state.sta |= cmp(state.get_y() as u8, state.mem.fetch_absolute()); }
