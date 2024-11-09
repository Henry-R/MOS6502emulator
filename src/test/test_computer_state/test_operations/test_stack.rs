use crate::computer_state::ComputerState;
use crate::computer_state::operations::opcode_from_operation;
use crate::computer_state::operations::stack::*;
use crate::computer_state::operations::flags::*;
use crate::computer_state::status_register::StatusRegister;

#[test]
fn test_pha() {
    let mut state = ComputerState::new();
    state.acc.set(0x13);
    state.set_up_state(&vec![
        opcode_from_operation(pha)
    ]);
    state.execute_next();

    assert_eq!(0x13, state.acc.get());
    assert_eq!(0x13, state.mem.pop_from_stack());
    assert!(state.sta.is_empty());
}

#[test]
fn test_php() {
    let mut state = ComputerState::new();
    state.sta = StatusRegister::from_byte(0xFF);
    state.set_up_state(&vec![
        opcode_from_operation(php)
    ]);
    state.execute_next();

    assert_eq!(0xFF, state.sta.as_byte());
    assert_eq!(0xFF,state.mem.pop_from_stack());
}

#[test]
fn test_pla() {
    let mut state = ComputerState::new();
    state.set_up_state(&vec![
        opcode_from_operation(pla)
    ]);
    state.mem.push_on_stack(0x05);
    state.execute_next();

    assert_eq!(0x05, state.acc.get());
    assert!(state.sta.is_empty());
}

#[test]
fn test_pla_zero_flag() {
    let mut state = ComputerState::new();
    state.set_up_state(&vec![
        opcode_from_operation(pla)
    ]);
    state.mem.push_on_stack(0);
    state.execute_next();

    assert_eq!(0, state.acc.get());
    assert!(state.sta.contains_only(StatusRegister::Z));
}

#[test]
fn test_pla_negative_flag() {
    let mut state = ComputerState::new();
    state.set_up_state(&vec![
        opcode_from_operation(pla)
    ]);
    state.mem.push_on_stack(0xF5);
    state.execute_next();

    assert_eq!(0xF5, state.acc.get());
    assert!(state.sta.contains_only(StatusRegister::N));
}

#[test]
fn test_plp() {
    let mut state = ComputerState::new();
    // Every flag set
    state.sta |= StatusRegister::V |
                 StatusRegister::Z |
                 StatusRegister::N |
                 StatusRegister::C |
                 StatusRegister::B |
                 StatusRegister::D |
                 StatusRegister::I;
    state.set_up_state(&vec![
        opcode_from_operation(php),
        opcode_from_operation(clc),
        opcode_from_operation(cld),
        opcode_from_operation(cli),
        opcode_from_operation(clv),
        opcode_from_operation(plp),
    ]);
    for _ in 0..6 {
        state.execute_next();
    }

    assert!(state.sta.contains_only(
        StatusRegister::V |
        StatusRegister::Z |
        StatusRegister::N |
        StatusRegister::C |
        StatusRegister::B |
        StatusRegister::D |
        StatusRegister::I
    ));
}
