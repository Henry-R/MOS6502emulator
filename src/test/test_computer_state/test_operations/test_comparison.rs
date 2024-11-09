use crate::computer_state::ComputerState;
use crate::computer_state::operations::comparison::*;
use crate::computer_state::operations::opcode_from_operation;
use crate::computer_state::status_register::StatusRegister;

#[test]
fn test_cmp_im_equal() {
    let mut state = ComputerState::new();
    state.acc.set(100);
    state.set_up_state(&vec![
        opcode_from_operation(cmp_im),
        100
    ]);
    state.execute_next();

    assert!(state.sta.contains_only(StatusRegister::Z | StatusRegister::C));
}

#[test]
fn test_cmp_im_less_than() {
    let mut state = ComputerState::new();
    state.acc.set(99);
    state.set_up_state(&vec![
        opcode_from_operation(cmp_im),
        100
    ]);
    state.execute_next();

    assert!(state.sta.contains_only(StatusRegister::N));
}

#[test]
fn test_cmp_im_greater_than() {
    let mut state = ComputerState::new();
    state.acc.set(101);
    state.set_up_state(&vec![
        opcode_from_operation(cmp_im),
        100
    ]);
    state.execute_next();

    assert!(state.sta.contains_only(StatusRegister::C));
}

#[test]
fn test_cmp_zp() {
    let mut state = ComputerState::new();
    state.acc.set(100);
    state.set_up_state(&vec![
        opcode_from_operation(cmp_zp),
        0x10
    ]);
    state.mem.set_byte_at_addr(0x10, 100);
    state.execute_next();

    assert!(state.sta.contains_only(StatusRegister::Z | StatusRegister::C));
}

#[test]
fn test_cmp_zpx() {
    let mut state = ComputerState::new();
    state.acc.set(100);
    state.set_x(0x5);
    state.set_up_state(&vec![
        opcode_from_operation(cmp_zpx),
        0x10
    ]);
    state.mem.set_byte_at_addr(0x15, 100);
    state.execute_next();

    assert!(state.sta.contains_only(StatusRegister::Z | StatusRegister::C));
}

#[test]
fn test_cmp_ab() {
    let mut state = ComputerState::new();
    state.acc.set(100);
    state.set_up_state(&vec![
        opcode_from_operation(cmp_ab),
        0x10
    ]);
    state.mem.set_byte_at_addr(0x10, 100);
    state.execute_next();

    assert!(state.sta.contains_only(StatusRegister::Z | StatusRegister::C));
}

#[test]
fn test_cmp_abx() {
    let mut state = ComputerState::new();
    state.acc.set(100);
    state.set_x(0x5);
    state.set_up_state(&vec![
        opcode_from_operation(cmp_abx),
        0x10
    ]);
    state.mem.set_byte_at_addr(0x15, 100);
    state.execute_next();

    assert!(state.sta.contains_only(StatusRegister::Z | StatusRegister::C));
}

#[test]
fn test_cmp_aby() {
    let mut state = ComputerState::new();
    state.acc.set(100);
    state.set_y(0x5);
    state.set_up_state(&vec![
        opcode_from_operation(cmp_aby),
        0x10
    ]);
    state.mem.set_byte_at_addr(0x15, 100);
    state.execute_next();

    assert!(state.sta.contains_only(StatusRegister::Z | StatusRegister::C));
}

#[test]
fn test_cmp_inx() {
    let mut state = ComputerState::new();
    state.acc.set(100);
    state.set_x(0x22);
    state.set_up_state(&vec![
        opcode_from_operation(cmp_inx),
        0x55
    ]);
    state.mem.set_nibble_at_addr(0x77, 0x1234);
    state.mem.set_byte_at_addr(0x1234, 100);
    state.execute_next();

    assert!(state.sta.contains_only(StatusRegister::Z | StatusRegister::C));
}

#[test]
fn test_cmp_iny() {
    let mut state = ComputerState::new();
    state.acc.set(100);
    state.set_y(0x22);
    state.set_up_state(&vec![
        opcode_from_operation(cmp_iny),
        0x41
    ]);
    state.mem.set_nibble_at_addr(0x41, 0x1234);
    state.mem.set_byte_at_addr(0x1256, 100);
    state.execute_next();

    assert!(state.sta.contains_only(StatusRegister::Z | StatusRegister::C));
}

#[test]
fn test_cpx_im() {
    let mut state = ComputerState::new();
    state.set_x(100);
    state.set_up_state(&vec![
        opcode_from_operation(cpx_im),
        100
    ]);
    state.execute_next();

    assert!(state.sta.contains_only(StatusRegister::Z | StatusRegister::C));
}

#[test]
fn test_cpx_zp() {
    let mut state = ComputerState::new();
    state.set_x(100);
    state.set_up_state(&vec![
        opcode_from_operation(cpx_zp),
        0x10
    ]);
    state.mem.set_byte_at_addr(0x10, 100);
    state.execute_next();

    assert!(state.sta.contains_only(StatusRegister::Z | StatusRegister::C));
}

#[test]
fn test_cpx_ab() {
    let mut state = ComputerState::new();
    state.set_x(100);
    state.set_up_state(&vec![
        opcode_from_operation(cpx_ab),
        0x10,
        0x2A
    ]);
    state.mem.set_byte_at_addr(0x2A10, 100);
    state.execute_next();

    assert!(state.sta.contains_only(StatusRegister::Z | StatusRegister::C));
}

#[test]
fn test_cpy_im() {
    let mut state = ComputerState::new();
    state.set_y(100);
    state.set_up_state(&vec![
        opcode_from_operation(cpy_im),
        100
    ]);
    state.execute_next();

    assert!(state.sta.contains_only(StatusRegister::Z | StatusRegister::C));
}

#[test]
fn test_cpy_zp() {
    let mut state = ComputerState::new();
    state.set_y(100);
    state.set_up_state(&vec![
        opcode_from_operation(cpy_zp),
        0x10
    ]);
    state.mem.set_byte_at_addr(0x10, 100);
    state.execute_next();

    assert!(state.sta.contains_only(StatusRegister::Z | StatusRegister::C));
}

#[test]
fn test_cpy_ab() {
    let mut state = ComputerState::new();
    state.set_y(100);
    state.set_up_state(&vec![
        opcode_from_operation(cpy_ab),
        0x10,
        0x2A
    ]);
    state.mem.set_byte_at_addr(0x2A10, 100);
    state.execute_next();

    assert!(state.sta.contains_only(StatusRegister::Z | StatusRegister::C));
}
