use crate::computer_state::ComputerState;
use crate::computer_state::memory::Memory;
use crate::computer_state::status_register::get_zero_neg_flags;

fn lda(computer_state: &mut ComputerState, value: u8) {
    computer_state.acc.set(value);
    computer_state.sta |= get_zero_neg_flags(value);
}

/// LDA (immediate addressing mode)
/// Opcode: A9
pub fn lda_im(state: &mut ComputerState) {
    let value = Memory::fetch_immediate(&mut state.mem);
    lda(state, value);
}

/// LDA (zero-page addressing mode)
/// Opcode: A5
pub fn lda_zp(state: &mut ComputerState) {
    let value = Memory::fetch_zero_page(&mut state.mem);
    lda(state, value);
}

/// LDA (zero-page X addressing mode)
/// Opcode: B5
pub fn lda_zpx(state: &mut ComputerState) {
    let value = Memory::fetch_zero_page_x(&mut state.mem);
    lda(state, value);
}

/// LDA (absolute addressing mode)
/// Opcode: AD
pub fn lda_ab(state: &mut ComputerState) {
    let value = Memory::fetch_absolute(&mut state.mem);
    lda(state, value);
}

/// LDA (absolute X addressing mode)
/// Opcode: BD
pub fn lda_abx(state: &mut ComputerState) {
    let value = Memory::fetch_absolute_x(&mut state.mem);
    lda(state, value);
}

/// LDA (absolute Y addressing mode)
/// Opcode: B9
pub fn lda_aby(state: &mut ComputerState) {
    let value = Memory::fetch_absolute_y(&mut state.mem);
    lda(state, value);
}

/// LDA (indirect X addressing mode)
/// Opcode: A1
pub fn lda_inx(state: &mut ComputerState) {
    let value = Memory::fetch_indirect_x(&mut state.mem);
    lda(state, value);
}

/// LDA (indirect Y addressing mode)
/// Opcode: B1
pub fn lda_iny(state: &mut ComputerState) {
    let value = Memory::fetch_indirect_y(&mut state.mem);
    lda(state, value);
}


fn ldx(state: &mut ComputerState, value: u8) {
    state.set_x(usize::from(value));
    state.sta |= get_zero_neg_flags(value);
}

/// LDX (immediate addressing mode)
/// Opcode: A2
pub fn ldx_im(state: &mut ComputerState) {
    let value = Memory::fetch_immediate(&mut state.mem);
    ldx(state, value);
}

/// LDX (zero-page addressing mode)
/// Opcode: A6
pub fn ldx_zp(state: &mut ComputerState) {
    let value = Memory::fetch_zero_page(&mut state.mem);
    ldx(state, value);
}

/// LDX (zero-page Y addressing mode)
/// Opcode: B6
pub fn ldx_zpy(state: &mut ComputerState) {
    let value = Memory::fetch_zero_page_y(&mut state.mem);
    ldx(state, value);
}

/// LDX (absolute addressing mode)
/// Opcode: AE
pub fn ldx_ab(state: &mut ComputerState) {
    let value = Memory::fetch_absolute(&mut state.mem);
    ldx(state, value);
}

/// LDX (absolute Y addressing mode)
/// Opcode: BE
pub fn ldx_aby(state: &mut ComputerState) {
    let value = Memory::fetch_absolute_y(&mut state.mem);
    ldx(state, value);
}


fn ldy(state: &mut ComputerState, value: u8) {
    state.set_y(usize::from(value));
    state.sta |= get_zero_neg_flags(value);
}

/// LDY (immediate addressing mode)
/// Opcode: A2
pub fn ldy_im(state: &mut ComputerState) {
    let value = Memory::fetch_immediate(&mut state.mem);
    ldy(state, value);
}

/// LDY (zero-page addressing mode)
/// Opcode: A6
pub fn ldy_zp(state: &mut ComputerState) {
    let value = Memory::fetch_zero_page(&mut state.mem);
    ldy(state, value);
}

/// LDY (zero-page X addressing mode)
/// Opcode: B6
pub fn ldy_zpx(state: &mut ComputerState) {
    let value = Memory::fetch_zero_page_x(&mut state.mem);
    ldy(state, value);
}

/// LDY (absolute addressing mode)
/// Opcode: AE
pub fn ldy_ab(state: &mut ComputerState) {
    let value = Memory::fetch_absolute(&mut state.mem);
    ldy(state, value);
}

/// LDY (absolute X addressing mode)
/// Opcode: BE
pub fn ldy_abx(state: &mut ComputerState) {
    let value = Memory::fetch_absolute_x(&mut state.mem);
    ldy(state, value);
}


/// STA (zero-page addressing mode)
/// Opcode: 95
pub fn sta_zp(state: &mut ComputerState) {
    let addr = Memory::fetch_zero_page_address(&mut state.mem);
    state.mem.set_byte_at_addr(addr, state.acc.get())
}
/// STA (zero-page X addressing mode)
/// Opcode: 85
pub fn sta_zpx(state: &mut ComputerState) {
    let addr = Memory::fetch_zero_page_x_address(&mut state.mem);
    state.mem.set_byte_at_addr(addr, state.acc.get())
}

/// STA (absolute addressing mode)
/// Opcode: 8D
pub fn sta_ab(state: &mut ComputerState) {
    let addr = Memory::fetch_absolute_address(&mut state.mem);
    state.mem.set_byte_at_addr(addr, state.acc.get())
}

/// STA (absolute X addressing mode)
/// Opcode: 9D
pub fn sta_abx(state: &mut ComputerState) {
    let addr = Memory::fetch_absolute_x_address(&mut state.mem);
    state.mem.set_byte_at_addr(addr, state.acc.get())
}

/// STA (absolute Y addressing mode)
/// Opcode: 99
pub fn sta_aby(state: &mut ComputerState) {
    let addr = Memory::fetch_absolute_y_address(&mut state.mem);
    state.mem.set_byte_at_addr(addr, state.acc.get())
}

/// STA (indirect X addressing mode)
/// Opcode: 81
pub fn sta_inx(state: &mut ComputerState) {
    let addr = Memory::fetch_indirect_x_address(&mut state.mem);
    state.mem.set_byte_at_addr(addr, state.acc.get())
}

/// STA (indirect Y addressing mode)
/// Opcode: 91
pub fn sta_iny(state: &mut ComputerState) {
    let addr = Memory::fetch_indirect_y_address(&mut state.mem);
    state.mem.set_byte_at_addr(addr, state.acc.get())
}


/// STX (zero-page addressing mode)
/// Opcode: 86
pub fn stx_zp(state: &mut ComputerState) {
    let addr = Memory::fetch_zero_page_address(&mut state.mem);
    state.mem.set_byte_at_addr(addr, state.get_x() as u8)
}

/// STX (zero-page Y addressing mode)
/// Opcode: 96
pub fn stx_zpy(state: &mut ComputerState) {
    let addr = Memory::fetch_zero_page_y_address(&mut state.mem);
    state.mem.set_byte_at_addr(addr, state.get_x() as u8)
}

/// STX (absolute addressing mode)
/// Opcode: 8E
pub fn stx_ab(state: &mut ComputerState) {
    let addr = Memory::fetch_absolute_address(&mut state.mem);
    state.mem.set_byte_at_addr(addr, state.get_x() as u8)
}


/// STY (zero-page addressing mode)
/// Opcode: 84
pub fn sty_zp(state: &mut ComputerState) {
    let addr = Memory::fetch_zero_page_address(&mut state.mem);
    state.mem.set_byte_at_addr(addr, state.get_y() as u8)
}

/// STY (zero-page X addressing mode)
/// Opcode: 94
pub fn sty_zpx(state: &mut ComputerState) {
    let addr = Memory::fetch_zero_page_x_address(&mut state.mem);
    state.mem.set_byte_at_addr(addr, state.get_y() as u8)
}

/// STY (absolute addressing mode)
/// Opcode: 8C
pub fn sty_ab(state: &mut ComputerState) {
    let addr = Memory::fetch_absolute_address(&mut state.mem);
    state.mem.set_byte_at_addr(addr, state.get_y() as u8)
}
