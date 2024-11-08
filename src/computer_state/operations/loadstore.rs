use crate::computer_state::ComputerState;
use crate::computer_state::memory::Memory;

/// LDA (immediate addressing mode)
/// Opcode: A9
pub fn lda_im(state: &mut ComputerState) {
    state.acc.set(Memory::fetch_immediate(&mut state.mem))
}

/// LDA (zero-page addressing mode)
/// Opcode: A5
pub fn lda_zp(state: &mut ComputerState) {
    state.acc.set(Memory::fetch_zero_page(&mut state.mem))
}

/// LDA (zero-page X addressing mode)
/// Opcode: B5
pub fn lda_zpx(state: &mut ComputerState) {
    state.acc.set(Memory::fetch_zero_page_x(&mut state.mem))
}

/// LDA (absolute addressing mode)
/// Opcode: Ad
pub fn lda_ab(state: &mut ComputerState) {
    state.acc.set(Memory::fetch_absolute(&mut state.mem))
}

/// LDA (absolute X addressing mode)
/// Opcode: BD
pub fn lda_abx(state: &mut ComputerState) {
    state.acc.set(Memory::fetch_absolute_x(&mut state.mem))
}

/// LDA (absolute Y addressing mode)
/// Opcode: B9
pub fn lda_aby(state: &mut ComputerState) {
    state.acc.set(Memory::fetch_absolute_y(&mut state.mem))
}

/// LDA (indirect X addressing mode)
/// Opcode: A1
pub fn lda_inx(state: &mut ComputerState) {
    state.acc.set(Memory::fetch_indirect_x(&mut state.mem))
}

/// LDA (indirect Y addressing mode)
/// Opcode: B1
pub fn lda_iny(state: &mut ComputerState) {
    state.acc.set(Memory::fetch_indirect_y(&mut state.mem))
}


/// LDX (immediate addressing mode)
/// Opcode: A2
pub fn ldx_im(state: &mut ComputerState) {
    let value = Memory::fetch_immediate(&mut state.mem);
    state.set_x(usize::from(value))
}

/// LDX (zero-page addressing mode)
/// Opcode: A6
pub fn ldx_zp(state: &mut ComputerState) {
    let value = Memory::fetch_zero_page(&mut state.mem);
    state.set_x(usize::from(value))
}

/// LDX (zero-page Y addressing mode)
/// Opcode: B6
pub fn ldx_zpy(state: &mut ComputerState) {
    let value = Memory::fetch_zero_page_y(&mut state.mem);
    state.set_x(usize::from(value))
}

/// LDX (absolute addressing mode)
/// Opcode: AE
pub fn ldx_ab(state: &mut ComputerState) {
    let value = Memory::fetch_absolute(&mut state.mem);
    state.set_x(usize::from(value))
}

/// LDX (absolute Y addressing mode)
/// Opcode: BE
pub fn ldx_aby(state: &mut ComputerState) {
    let value = Memory::fetch_absolute_y(&mut state.mem);
    state.set_x(usize::from(value))
}


/// LDY (immediate addressing mode)
/// Opcode: A2
pub fn ldy_im(state: &mut ComputerState) {
    let value = Memory::fetch_immediate(&mut state.mem);
    state.set_y(usize::from(value))
}

/// LDY (zero-page addressing mode)
/// Opcode: A6
pub fn ldy_zp(state: &mut ComputerState) {
    let value = Memory::fetch_zero_page(&mut state.mem);
    state.set_y(usize::from(value))
}

/// LDY (zero-page X addressing mode)
/// Opcode: B6
pub fn ldy_zpy(state: &mut ComputerState) {
    let value = Memory::fetch_zero_page_x(&mut state.mem);
    state.set_y(usize::from(value))
}

/// LDY (absolute addressing mode)
/// Opcode: AE
pub fn ldy_ab(state: &mut ComputerState) {
    let value = Memory::fetch_absolute(&mut state.mem);
    state.set_y(usize::from(value))
}

/// LDY (absolute X addressing mode)
/// Opcode: BE
pub fn ldy_aby(state: &mut ComputerState) {
    let value = Memory::fetch_absolute_x(&mut state.mem);
    state.set_y(usize::from(value))
}

