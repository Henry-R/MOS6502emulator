use crate::computer_state::ComputerState;
use crate::computer_state::operations::jumps_calls::*;
use crate::computer_state::operations::load_store::lda_im;
use crate::computer_state::operations::opcode_from_operation;

#[test]
fn test_jmp_ab() {
    let mut state = ComputerState::new();
    state.set_up_state(&vec![
        opcode_from_operation(jmp_ab),
        0x34,
        0x12
    ]);
    state.execute_next();

    assert_eq!(0x1234, state.mem.pc.get());
}

#[test]
fn test_jmp_in() {
    let mut state = ComputerState::new();
    state.set_up_state(&vec![
        opcode_from_operation(jmp_in),
        0x34,
        0x12
    ]);
    state.mem.set_nibble_at_addr(0x1234, 0xABF6);
    state.execute_next();

    assert_eq!(0xABF6, state.mem.pc.get());
}

#[test]
fn test_jsr() {
    let mut state = ComputerState::new();
    state.set_up_state(&vec![
        opcode_from_operation(jsr),
        0x34,
        0x12
    ]);
    state.execute_next();

    assert_eq!(0x1234, state.mem.pc.get());
    assert_eq!(3, state.mem.pop_nibble_from_stack());
}

#[test]
fn test_rts() {
    let mut state = ComputerState::new();
    state.set_up_state(&vec![
        opcode_from_operation(jsr),
        0x07,
        0x06,
    ]);
    state.mem.set_byte_at_addr(0x0607, opcode_from_operation(lda_im));
    state.mem.set_byte_at_addr(0x0608, 0x99);
    state.mem.set_byte_at_addr(0x0609, opcode_from_operation(rts));

    for _ in 1..4 {
        state.execute_next()
    }

    assert_eq!(0x99, state.acc.get());
    assert_eq!(3, state.mem.pc.get());
}
