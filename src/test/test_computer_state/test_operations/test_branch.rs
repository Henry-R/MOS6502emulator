use crate::computer_state::ComputerState;
use crate::computer_state::operations::branch::*;
use crate::computer_state::operations::opcode_from_operation;
use crate::computer_state::status_register::StatusRegister;

#[test]
fn test_brc_forward() {
    let mut state = ComputerState::new();
    state.sta |= StatusRegister::C;
    state.set_up_state(&vec![
        opcode_from_operation(bcs),
        0x23
    ]);
    state.execute_next();

    assert_eq!(2 + 0x23, state.mem.pc.get());
}

#[ignore]
#[test]
fn test_brc_backwards() {
    let mut state = ComputerState::new();
    state.sta |= StatusRegister::C;
    state.set_up_state(&vec![
        opcode_from_operation(bcs),
        0xF0
    ]);
    state.execute_next();

    assert_eq!(2 + 2u16.wrapping_sub(0x0F) as usize, state.mem.pc.get());
}
