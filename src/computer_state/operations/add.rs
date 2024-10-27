use crate::computer_state::ComputerState;

/// ADd with carry
fn add(state: &mut ComputerState, value: u8) {
    let raw_add: u16 = state.regs.acc as u16 + value as u16;
}