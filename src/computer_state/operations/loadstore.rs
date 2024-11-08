use crate::computer_state::ComputerState;
use crate::computer_state::memory::Memory;

/// LDA (immediate addressing mode)
/// Opcode: A9
pub fn lda_im(state: &mut ComputerState)
{ state.acc.set(Memory::fetch_intermediate(&mut state.mem)) }

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


