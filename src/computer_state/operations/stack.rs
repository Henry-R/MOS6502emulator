use crate::computer_state::ComputerState;
use crate::computer_state::status_register::{get_zero_neg_flags, StatusRegister};

/// TSX (transfer stack pointer to X)
/// Opcode: BA
pub fn tsx(state: &mut ComputerState) {
    state.set_x(state.get_stk());
    state.sta |= get_zero_neg_flags(state.get_x() as u8);
}

/// TXS (transfer X to stack pointer)
/// Opcode: 9A
pub fn txs(state: &mut ComputerState) {
    state.set_stk(state.get_x());
}

/// PHA (push accumulator)
/// Opcode: 48
pub fn pha(state: &mut ComputerState) {
    state.mem.push_on_stack(state.acc.get())
}

/// PHP (push processor status)
/// Opcode: 08
pub fn php(state: &mut ComputerState) {
    state.mem.push_on_stack(state.sta.as_byte())
}

/// PLA (pull accumulator)
/// Opcode: 68
pub fn pla(state: &mut ComputerState) {
    state.acc.set(state.mem.pop_from_stack());
    state.sta |= get_zero_neg_flags(state.acc.get());
}

/// PLP (pull processor status)
/// Opcode: 28
pub fn plp(state: &mut ComputerState) {
    state.sta = StatusRegister::from_byte(state.mem.pop_from_stack());
}
