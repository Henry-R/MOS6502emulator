use crate::computer_state::ComputerState;
use crate::computer_state::operations::branch::*;
use crate::computer_state::operations::opcode_from_operation;
use crate::computer_state::status_register::StatusRegister;

#[test]
fn test_brc_forward() {
    let mut state = ComputerState::new();
    let old_pc = state.mem.pc.get();
    state.sta |= StatusRegister::C;
    state.set_up_state(&vec![
        opcode_from_operation(bcs),
        0x23
    ]);
    state.execute_next();

    assert_eq!(old_pc + 2 + 0x23, state.mem.pc.get());
}

#[test]
fn test_brc_backwards() {
    let mut state = ComputerState::new();
    let old_pc = state.mem.pc.get();
    state.sta |= StatusRegister::C;
    state.set_up_state(&vec![
        opcode_from_operation(bcs),
        0xF0 // -16
    ]);
    state.execute_next();

    // TODO double check 0x10 is the correct offset
    assert_eq!(old_pc + 2u16.wrapping_sub(0x10) as usize, state.mem.pc.get());
}

#[test]
fn test_brc_not_take() {
    let mut state = ComputerState::new();
    let old_pc = state.mem.pc.get();
    state.set_up_state(&vec![
        opcode_from_operation(bcs),
        0xF0 // -16
    ]);
    state.execute_next();

    assert_eq!(old_pc + 2, state.mem.pc.get());
}
