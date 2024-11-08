use crate::computer_state::ComputerState;
use crate::computer_state::status_register::get_zero_neg_flags;

/// TAX (transfer accumulator to X)
/// Opcode: AA
pub fn tax(state: &mut ComputerState) {
    state.set_x(usize::from(state.acc.get()));
    state.sta |= get_zero_neg_flags(state.acc.get())
}

/// TAY (transfer accumulator to Y)
/// Opcode: A8
pub fn tay(state: &mut ComputerState) {
    state.set_y(usize::from(state.acc.get()));
    state.sta |= get_zero_neg_flags(state.acc.get())
}

/// TXA (transfer X to accumulator)
/// Opcode: 8A
pub fn txa(state: &mut ComputerState) {
    state.acc.set(state.get_x() as u8);
    state.sta |= get_zero_neg_flags(state.acc.get())
}

/// TYA (transfer Y to accumulator)
/// Opcode: 98
pub fn tya(state: &mut ComputerState) {
    state.acc.set(state.get_y() as u8);
    state.sta |= get_zero_neg_flags(state.acc.get())
}