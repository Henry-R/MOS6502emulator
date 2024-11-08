use crate::computer_state::ComputerState;
use crate::computer_state::status_register::StatusRegister;

fn branch(state: &mut ComputerState, condition: bool) {
    let addr = state.mem.fetch_relative();
    if condition {
        state.mem.pc.add_signed(addr);
    }
}

/// BCS (Branch if carry set)
/// Opcode: B0
pub fn bcs(state: &mut ComputerState)
{ branch(state, state.sta.contains(StatusRegister::C)); }

/// BCC (Branch if carry clear)
/// Opcode: 90
pub fn bcc(state: &mut ComputerState)
{ branch(state, !state.sta.contains(StatusRegister::C)); }

/// BEQ (Branch if equal)
/// Opcode: F0
pub fn beq(state: &mut ComputerState)
{ branch(state, state.sta.contains(StatusRegister::Z)); }

/// BNE (Branch if not equal)
/// Opcode: D0
pub fn bne(state: &mut ComputerState)
{ branch(state, !state.sta.contains(StatusRegister::Z)); }

/// BMI (Branch if minus)
/// Opcode: 30
pub fn bmi(state: &mut ComputerState)
{ branch(state, state.sta.contains(StatusRegister::N)); }

/// BPL (Branch if positive)
/// Opcode: 10
pub fn bpl(state: &mut ComputerState)
{ branch(state, !state.sta.contains(StatusRegister::N)); }

/// BVS (Branch if overflow set)
/// Opcode: 70
pub fn bvs(state: &mut ComputerState)
{ branch(state, state.sta.contains(StatusRegister::V)); }

/// BVC (Branch if overflow)
/// Opcode: 50
pub fn bvc(state: &mut ComputerState)
{ branch(state, !state.sta.contains(StatusRegister::V)); }
