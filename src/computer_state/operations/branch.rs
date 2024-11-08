use crate::computer_state::ComputerState;
use crate::computer_state::status_register::StatusRegister;

/// BCS (Branch if carry set)
/// Opcode: B0
pub fn bcs(state: &mut ComputerState) {
    let addr = state.mem.fetch_relative();
    if state.sta.contains(StatusRegister::C) {
        state.mem.pc.add_signed(addr);
    }
}