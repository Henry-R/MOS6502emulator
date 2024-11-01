use crate::computer_state::{ComputerState, StatusRegister};

// CLEAR INSTRUCTIONS
/// CLR (Clear carry flag)
/// Opcode: 18
pub fn clr(state: &mut ComputerState) {
    state.regs.sta = state.regs.sta.difference(StatusRegister::C)
}

/// CLD (Clear decimal flag)
/// Opcode: D8
pub fn cld(state: &mut ComputerState) {
    state.regs.sta = state.regs.sta.difference(StatusRegister::D)
}

/// CLD (Clear interrupt disable status)
/// Opcode: 58
pub fn cli(state: &mut ComputerState) {
    state.regs.sta = state.regs.sta.difference(StatusRegister::I)
}

/// CLD (Clear overflow flag)
/// Opcode: B8
pub fn clo(state: &mut ComputerState) {
    state.regs.sta = state.regs.sta.difference(StatusRegister::V)
}

/// SEC (Set carry flag)
/// Opcode: 38
pub fn sec(state: &mut ComputerState) {
    state.regs.sta |= StatusRegister::C
}

/// SED (Set decimal flag)
/// Opcode: F8
pub fn sed(state: &mut ComputerState) {
    state.regs.sta |= StatusRegister::D
}

/// SEI (Set interrupt disable status)
/// Opcode: 78
pub fn sei(state: &mut ComputerState) {
    state.regs.sta |= StatusRegister::I
}