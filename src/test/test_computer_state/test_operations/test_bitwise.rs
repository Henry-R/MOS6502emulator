

use crate::computer_state::ComputerState;
use crate::computer_state::operations::bitwise::*;
use crate::computer_state::operations::opcode_from_operation;
use crate::computer_state::status_register::StatusRegister;
// AND TESTS

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

#[test]
fn test_and_im_zero_flag() {
    let mut state: ComputerState = ComputerState::new();
    state.regs.acc = 0x22;
    state.set_up_state(vec![
        opcode_from_operation(and_im),
        0x11
    ]);
    state.execute_next();

    assert_eq!(0, state.regs.acc);
    assert!(state.regs.sta.contains_only(StatusRegister::Z));
}

#[test]
fn test_and_im_negative_flag() {
    let mut state: ComputerState = ComputerState::new();
    state.regs.acc = 0xA1;
    state.set_up_state(vec![
        opcode_from_operation(and_im),
        0xF0
    ]);
    state.execute_next();

    assert_eq!(0xA0, state.regs.acc);
    assert!(state.regs.sta.contains_only(StatusRegister::N));
}

#[test]
fn test_and_zp() {
    let mut state: ComputerState = ComputerState::new();
    state.regs.acc = 0x41;
    state.set_up_state(vec![
        opcode_from_operation(and_zp),
        0xF5
    ]);
    state.set_byte_at_addr(0xF5, 0x55);
    state.execute_next();

    assert_eq!(0x41, state.regs.acc);
    assert!(state.regs.sta.is_empty());
}

#[test]
fn test_and_zpx() {
    let mut state: ComputerState = ComputerState::new();
    state.regs.acc = 0x41;
    state.regs.x = 0x50;
    state.set_up_state(vec![
        opcode_from_operation(and_zpx),
        0x30
    ]);
    state.set_byte_at_addr(0x80, 0x55);
    state.execute_next();

    assert_eq!(0x41, state.regs.acc);
    assert!(state.regs.sta.is_empty());
}

#[test]
fn test_and_ab() {
    let mut state: ComputerState = ComputerState::new();
    state.regs.acc = 0x41;
    state.set_up_state(vec![
        opcode_from_operation(and_ab),
        0x30,
        0x05
    ]);
    state.set_byte_at_addr(0x0530, 0x55);
    state.execute_next();

    assert_eq!(0x41, state.regs.acc);
    assert!(state.regs.sta.is_empty());
}

#[test]
fn test_and_abx() {
    let mut state: ComputerState = ComputerState::new();
    state.regs.acc = 0x41;
    state.regs.x = 0x0A;
    state.set_up_state(vec![
        opcode_from_operation(and_abx),
        0x30,
        0x05
    ]);
    state.set_byte_at_addr(0x053A, 0x55);
    state.execute_next();

    assert_eq!(0x41, state.regs.acc);
    assert!(state.regs.sta.is_empty());
}

#[test]
fn test_and_aby() {
    let mut state: ComputerState = ComputerState::new();
    state.regs.acc = 0x41;
    state.regs.y = 0x0A;
    state.set_up_state(vec![
        opcode_from_operation(and_aby),
        0x30,
        0x05
    ]);
    state.set_byte_at_addr(0x053A, 0x55);
    state.execute_next();

    assert_eq!(0x41, state.regs.acc);
    assert!(state.regs.sta.is_empty());
}

#[test]
fn test_and_inx() {
    let mut state: ComputerState = ComputerState::new();
    state.regs.acc = 0x41;
    state.regs.x = 0x22;
    state.set_up_state(vec![
        opcode_from_operation(and_inx),
        0x55
    ]);
    state.set_nibble_at_addr(0x77, 0x1234);
    state.set_byte_at_addr(0x1234, 0x43);
    state.execute_next();

    assert_eq!(0x41, state.regs.acc);
    assert!(state.regs.sta.is_empty());
}

#[test]
fn test_and_iny() {
    let mut state: ComputerState = ComputerState::new();
    state.regs.acc = 0xFF;
    state.regs.y = 0x22;
    state.set_up_state(vec![
        opcode_from_operation(and_iny),
        0x41
    ]);
    state.set_nibble_at_addr(0x41, 0x1234);
    state.set_byte_at_addr(0x1256, 0x29);
    state.execute_next();

    assert_eq!(0x29, state.regs.acc);
    assert!(state.regs.sta.is_empty());
}


// TEST OR
#[test]
fn test_or_im() {
    let mut state: ComputerState = ComputerState::new();
    state.regs.acc = 0x13;
    state.set_up_state(vec![
        opcode_from_operation(or_im),
        0x30
    ]);
    state.execute_next();

    assert_eq!(0x33, state.regs.acc);
    assert!(state.regs.sta.is_empty());
}

#[test]
fn test_or_im_zero_flag() {
    let mut state: ComputerState = ComputerState::new();
    state.regs.acc = 0x00;
    state.set_up_state(vec![
        opcode_from_operation(or_im),
        0x00
    ]);
    state.execute_next();

    assert_eq!(0x00, state.regs.acc);
    assert!(state.regs.sta.contains_only(StatusRegister::Z));
}

#[test]
fn test_or_im_negative_flag() {
    let mut state: ComputerState = ComputerState::new();
    state.regs.acc = 0xA1;
    state.set_up_state(vec![
        opcode_from_operation(or_im),
        0xF0
    ]);
    state.execute_next();

    assert_eq!(0xF1, state.regs.acc);
    assert!(state.regs.sta.contains_only(StatusRegister::N));
}

#[test]
fn test_or_zp() {
    let mut state: ComputerState = ComputerState::new();
    state.regs.acc = 0x25;
    state.set_up_state(vec![
        opcode_from_operation(or_zp),
        0xF5
    ]);
    state.set_byte_at_addr(0xF5, 0x10);
    state.execute_next();

    assert_eq!(0x35, state.regs.acc);
    assert!(state.regs.sta.is_empty());
}

#[test]
fn test_or_zpx() {
    let mut state: ComputerState = ComputerState::new();
    state.regs.acc = 0x42;
    state.regs.x = 0x50;
    state.set_up_state(vec![
        opcode_from_operation(or_zpx),
        0x30
    ]);
    state.set_byte_at_addr(0x80, 0x81);
    state.execute_next();

    assert_eq!(0xC3, state.regs.acc);
    assert!(state.regs.sta.contains_only(StatusRegister::N));
}

#[test]
fn test_or_ab() {
    let mut state: ComputerState = ComputerState::new();
    state.regs.acc = 0x41;
    state.set_up_state(vec![
        opcode_from_operation(or_ab),
        0x30,
        0x05
    ]);
    state.set_byte_at_addr(0x0530, 0x55);
    state.execute_next();

    assert_eq!(0x55, state.regs.acc);
    assert!(state.regs.sta.is_empty());
}

#[test]
fn test_or_abx() {
    let mut state: ComputerState = ComputerState::new();
    state.regs.acc = 0x53;
    state.regs.x = 0x0A;
    state.set_up_state(vec![
        opcode_from_operation(or_abx),
        0x30,
        0x05
    ]);
    state.set_byte_at_addr(0x053A, 0x40);
    state.execute_next();

    assert_eq!(0x53, state.regs.acc);
    assert!(state.regs.sta.is_empty());
}

#[test]
fn test_or_aby() {
    let mut state: ComputerState = ComputerState::new();
    state.regs.acc = 0x02;
    state.regs.y = 0x0A;
    state.set_up_state(vec![
        opcode_from_operation(or_aby),
        0x30,
        0x05
    ]);
    state.set_byte_at_addr(0x053A, 0x20);
    state.execute_next();

    assert_eq!(0x22, state.regs.acc);
    assert!(state.regs.sta.is_empty());
}

#[test]
fn test_or_inx() {
    let mut state: ComputerState = ComputerState::new();
    state.regs.acc = 0x18;
    state.regs.x = 0x22;
    state.set_up_state(vec![
        opcode_from_operation(or_inx),
        0x30,
        0x05
    ]);
    state.set_nibble_at_addr(0x77, 0x1234);
    state.set_byte_at_addr(0x1234, 0x18);
    state.execute_next();

    assert_eq!(0x18, state.regs.acc);
    assert!(state.regs.sta.is_empty());
}

#[test]
fn test_or_iny() {
    let mut state: ComputerState = ComputerState::new();
    state.regs.acc = 0xFF;
    state.regs.y = 0x22;
    state.set_up_state(vec![
        opcode_from_operation(or_iny),
        0x41
    ]);
    state.set_nibble_at_addr(0x41, 0x1234);
    state.set_byte_at_addr(0x1256, 0x23);
    state.execute_next();

    assert_eq!(0xFF, state.regs.acc);
    assert!(state.regs.sta.contains_only(StatusRegister::N));
}


// TEST EOR

