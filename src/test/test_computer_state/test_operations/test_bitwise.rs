

use crate::computer_state::ComputerState;
use crate::computer_state::operations::bitwise::*;
use crate::computer_state::operations::opcode_from_operation;

#[test]
fn test_and_im() {
    let mut state: ComputerState = ComputerState::new();
    state.regs.acc = 0x13;
    state.set_up_state(vec![
        opcode_from_operation(and_im),
        0x23
    ]);
    state.execute_next();

    assert_eq!(0x03, state.regs.acc);
    assert!(state.regs.sta.is_empty());
}