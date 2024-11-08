use crate::computer_state::ComputerState;
use crate::computer_state::memory::Memory;

/// LDA ( load accumulator)
fn lda(state: &mut ComputerState, value: u8) {
    state.acc.set(value);
}

/// LDA (immediate addressing mode)
/// Opcode: A9
pub fn lda_im(state: &mut ComputerState)
{ state.acc.set(Memory::fetch_intermediate(&mut state.mem)) }

/// LDA (zero-page addressing mode)
/// Opcode: A9
pub fn lda_zp(state: &mut ComputerState) {
    let value = Memory::fetch_zero_page(&mut state.mem);
    lda(state, value)
}

/// LDA (zero-page X addressing mode)
/// Opcode: A9
pub fn lda_zpx(state: &mut ComputerState) {
    let value = Memory::fetch_zero_page_x(&mut state.mem);
    lda(state, value)
}

/// LDA (absolute addressing mode)
/// Opcode: A9
pub fn lda_ab(state: &mut ComputerState) {
    let value = Memory::fetch_absolute(&mut state.mem);
    lda(state, value)
}

/// LDA (absolute X addressing mode)
/// Opcode: A9
pub fn lda_abx(state: &mut ComputerState) {
    let value = Memory::fetch_absolute_x(&mut state.mem);
    lda(state, value)
}

/// LDA (absolute Y addressing mode)
/// Opcode: A9
pub fn lda_aby(state: &mut ComputerState) {
    let value = Memory::fetch_absolute_y(&mut state.mem);
    lda(state, value)
}

/// LDA (indirect X addressing mode)
/// Opcode: A9
pub fn lda_inx(state: &mut ComputerState) {
    let value = Memory::fetch_indirect_x(&mut state.mem);
    lda(state, value)
}

/// LDA (indirect Y addressing mode)
/// Opcode: A9
pub fn lda_iny(state: &mut ComputerState) {
    let value = Memory::fetch_indirect_y(&mut state.mem);
    lda(state, value)
}

