use crate::computer_state::{ComputerState, StatusFlags};

// CLEAR INSTRUCTIONS
/// CLR (Clear carry flag)
/// Opcode: 18
pub fn clr(state: &mut ComputerState) {
    state.regs.sta.remove(StatusFlags::c);
}

/// CLD (Clear decimal flag)
/// Opcode: D8
pub fn cld(state: &mut ComputerState) {
    state.regs.sta.remove(StatusFlags::d);
}

/// CLD (Clear interrupt disable status)
/// Opcode: 58
pub fn cli(state: &mut ComputerState) {
    state.regs.sta.remove(StatusFlags::i);
}

/// CLD (Clear overflow flag)
/// Opcode: B8
pub fn clo(state: &mut ComputerState) {
    state.regs.sta.remove(StatusFlags::v);
}

/// SEC (Set carry flag)
/// Opcode: 38
pub fn sec(state: &mut ComputerState) {
    state.regs.sta.insert(StatusFlags::c);
}

/// SED (Set decimal flag)
/// Opcode: F8
pub fn sed(state: &mut ComputerState) {
    state.regs.sta.insert(StatusFlags::d);
}

/// SEI (Set interrupt disable status)
/// Opcode: 78
pub fn sei(state: &mut ComputerState) {
    state.regs.sta.insert(StatusFlags::i);
}