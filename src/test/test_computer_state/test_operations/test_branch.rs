use crate::computer_state::ComputerState;
use crate::computer_state::operations::branch::*;
use crate::computer_state::operations::opcode_from_operation;
use crate::computer_state::status_register::StatusRegister;

#[test]
fn test_bcs_forward() {
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
fn test_bcs_backwards() {
    let mut state = ComputerState::new();
    let old_pc = state.mem.pc.get();
    state.sta |= StatusRegister::C;
    state.set_up_state(&vec![
        opcode_from_operation(bcs),
        0xF0 // -16
    ]);
    state.execute_next();

    assert_eq!(old_pc + 2u16.wrapping_sub(0x10) as usize, state.mem.pc.get());
}

#[test]
fn test_bcs_not_take() {
    let mut state = ComputerState::new();
    let old_pc = state.mem.pc.get();
    state.set_up_state(&vec![
        opcode_from_operation(bcs),
        0xF0 // -16
    ]);
    state.execute_next();

    assert_eq!(old_pc + 2, state.mem.pc.get());
}

#[test]
fn test_bcc_forward() {
    let mut state = ComputerState::new();
    let old_pc = state.mem.pc.get();
    state.set_up_state(&vec![
        opcode_from_operation(bcc),
        0x23
    ]);
    state.execute_next();

    assert_eq!(old_pc + 2 + 0x23, state.mem.pc.get());
}

#[test]
fn test_bcc_backwards() {
    let mut state = ComputerState::new();
    let old_pc = state.mem.pc.get();
    state.set_up_state(&vec![
        opcode_from_operation(bcc),
        0xF0 // -16
    ]);
    state.execute_next();

    assert_eq!(old_pc + 2u16.wrapping_sub(0x10) as usize, state.mem.pc.get());
}

#[test]
fn test_bcc_not_take() {
    let mut state = ComputerState::new();
    let old_pc = state.mem.pc.get();
    state.sta |= StatusRegister::C;
    state.set_up_state(&vec![
        opcode_from_operation(bcc),
        0xF0 // -16
    ]);
    state.execute_next();

    assert_eq!(old_pc + 2, state.mem.pc.get());
}

#[test]
fn test_beq_forward() {
    let mut state = ComputerState::new();
    let old_pc = state.mem.pc.get();
    state.sta |= StatusRegister::Z;
    state.set_up_state(&vec![
        opcode_from_operation(beq),
        0x23
    ]);
    state.execute_next();

    assert_eq!(old_pc + 2 + 0x23, state.mem.pc.get());
}

#[test]
fn test_beq_backwards() {
    let mut state = ComputerState::new();
    let old_pc = state.mem.pc.get();
    state.sta |= StatusRegister::Z;
    state.set_up_state(&vec![
        opcode_from_operation(beq),
        0xF0 // -16
    ]);
    state.execute_next();

    assert_eq!(old_pc + 2u16.wrapping_sub(0x10) as usize, state.mem.pc.get());
}

#[test]
fn test_beq_not_take() {
    let mut state = ComputerState::new();
    let old_pc = state.mem.pc.get();
    state.set_up_state(&vec![
        opcode_from_operation(beq),
        0xF0 // -16
    ]);
    state.execute_next();

    assert_eq!(old_pc + 2, state.mem.pc.get());
}

#[test]
fn test_bne_forward() {
    let mut state = ComputerState::new();
    let old_pc = state.mem.pc.get();
    state.set_up_state(&vec![
        opcode_from_operation(bne),
        0x23
    ]);
    state.execute_next();

    assert_eq!(old_pc + 2 + 0x23, state.mem.pc.get());
}

#[test]
fn test_bne_backwards() {
    let mut state = ComputerState::new();
    let old_pc = state.mem.pc.get();
    state.set_up_state(&vec![
        opcode_from_operation(bne),
        0xF0 // -16
    ]);
    state.execute_next();

    assert_eq!(old_pc + 2u16.wrapping_sub(0x10) as usize, state.mem.pc.get());
}

#[test]
fn test_bne_not_take() {
    let mut state = ComputerState::new();
    let old_pc = state.mem.pc.get();
    state.sta |= StatusRegister::Z;
    state.set_up_state(&vec![
        opcode_from_operation(bne),
        0xF0 // -16
    ]);
    state.execute_next();

    assert_eq!(old_pc + 2, state.mem.pc.get());
}

#[test]
fn test_bpl_forward() {
    let mut state = ComputerState::new();
    let old_pc = state.mem.pc.get();
    state.set_up_state(&vec![
        opcode_from_operation(bpl),
        0x23
    ]);
    state.execute_next();

    assert_eq!(old_pc + 2 + 0x23, state.mem.pc.get());
}

#[test]
fn test_bpl_backwards() {
    let mut state = ComputerState::new();
    let old_pc = state.mem.pc.get();
    state.set_up_state(&vec![
        opcode_from_operation(bpl),
        0xF0 // -16
    ]);
    state.execute_next();

    assert_eq!(old_pc + 2u16.wrapping_sub(0x10) as usize, state.mem.pc.get());
}

#[test]
fn test_bpl_not_take() {
    let mut state = ComputerState::new();
    let old_pc = state.mem.pc.get();
    state.sta |= StatusRegister::N;
    state.set_up_state(&vec![
        opcode_from_operation(bpl),
        0xF0 // -16
    ]);
    state.execute_next();

    assert_eq!(old_pc + 2, state.mem.pc.get());
}

#[test]
fn test_bmi_forward() {
    let mut state = ComputerState::new();
    let old_pc = state.mem.pc.get();
    state.sta |= StatusRegister::N;
    state.set_up_state(&vec![
        opcode_from_operation(bmi),
        0x23
    ]);
    state.execute_next();

    assert_eq!(old_pc + 2 + 0x23, state.mem.pc.get());
}

#[test]
fn test_bmi_backwards() {
    let mut state = ComputerState::new();
    let old_pc = state.mem.pc.get();
    state.sta |= StatusRegister::N;
    state.set_up_state(&vec![
        opcode_from_operation(bmi),
        0xF0 // -16
    ]);
    state.execute_next();

    assert_eq!(old_pc + 2u16.wrapping_sub(0x10) as usize, state.mem.pc.get());
}

#[test]
fn test_bmi_not_take() {
    let mut state = ComputerState::new();
    let old_pc = state.mem.pc.get();
    state.set_up_state(&vec![
        opcode_from_operation(bmi),
        0xF0 // -16
    ]);
    state.execute_next();

    assert_eq!(old_pc + 2, state.mem.pc.get());
}

#[test]
fn test_bvs_forward() {
    let mut state = ComputerState::new();
    let old_pc = state.mem.pc.get();
    state.sta |= StatusRegister::V;
    state.set_up_state(&vec![
        opcode_from_operation(bvs),
        0x23
    ]);
    state.execute_next();

    assert_eq!(old_pc + 2 + 0x23, state.mem.pc.get());
}

#[test]
fn test_bvs_backwards() {
    let mut state = ComputerState::new();
    let old_pc = state.mem.pc.get();
    state.sta |= StatusRegister::V;
    state.set_up_state(&vec![
        opcode_from_operation(bvs),
        0xF0 // -16
    ]);
    state.execute_next();

    assert_eq!(old_pc + 2u16.wrapping_sub(0x10) as usize, state.mem.pc.get());
}

#[test]
fn test_bvs_not_take() {
    let mut state = ComputerState::new();
    let old_pc = state.mem.pc.get();
    state.set_up_state(&vec![
        opcode_from_operation(bvs),
        0xF0 // -16
    ]);
    state.execute_next();

    assert_eq!(old_pc + 2, state.mem.pc.get());
}

#[test]
fn test_bvc_forward() {
    let mut state = ComputerState::new();
    let old_pc = state.mem.pc.get();
    state.set_up_state(&vec![
        opcode_from_operation(bvc),
        0x23
    ]);
    state.execute_next();

    assert_eq!(old_pc + 2 + 0x23, state.mem.pc.get());
}

#[test]
fn test_bvc_backwards() {
    let mut state = ComputerState::new();
    let old_pc = state.mem.pc.get();
    state.set_up_state(&vec![
        opcode_from_operation(bvc),
        0xF0 // -16
    ]);
    state.execute_next();

    assert_eq!(old_pc + 2u16.wrapping_sub(0x10) as usize, state.mem.pc.get());
}

#[test]
fn test_bvc_not_take() {
    let mut state = ComputerState::new();
    let old_pc = state.mem.pc.get();
    state.sta |= StatusRegister::V;
    state.set_up_state(&vec![
        opcode_from_operation(bvc),
        0xF0 // -16
    ]);
    state.execute_next();

    assert_eq!(old_pc + 2, state.mem.pc.get());
}
