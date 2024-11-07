

use crate::computer_state::ComputerState;
use crate::computer_state::operations::bitwise::*;
use crate::computer_state::operations::opcode_from_operation;
use crate::computer_state::status_register::StatusRegister;
// AND TESTS

#[test]
fn test_and_im() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(0x13);
    state.set_up_state(&vec![
        opcode_from_operation(and_im),
        0x23
    ]);
    state.execute_next();

    assert_eq!(0x03, state.acc.get());
    assert!(state.sta.is_empty());
}

#[test]
fn test_and_im_zero_flag() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(0x22);
    state.set_up_state(&vec![
        opcode_from_operation(and_im),
        0x11
    ]);
    state.execute_next();

    assert_eq!(0, state.acc.get());
    assert!(state.sta.contains_only(StatusRegister::Z));
}

#[test]
fn test_and_im_negative_flag() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(0xA1);
    state.set_up_state(&vec![
        opcode_from_operation(and_im),
        0xF0
    ]);
    state.execute_next();

    assert_eq!(0xA0, state.acc.get());
    assert!(state.sta.contains_only(StatusRegister::N));
}

#[test]
fn test_and_zp() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(0x41);
    state.set_up_state(&vec![
        opcode_from_operation(and_zp),
        0xF5
    ]);
    state.mem.set_byte_at_addr(0xF5, 0x55);
    state.execute_next();

    assert_eq!(0x41, state.acc.get());
    assert!(state.sta.is_empty());
}

#[test]
fn test_and_zpx() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(0x41);
    state.set_x(0x50);
    state.set_up_state(&vec![
        opcode_from_operation(and_zpx),
        0x30
    ]);
    state.mem.set_byte_at_addr(0x80, 0x55);
    state.execute_next();

    assert_eq!(0x41, state.acc.get());
    assert!(state.sta.is_empty());
}

#[test]
fn test_and_ab() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(0x41);
    state.set_up_state(&vec![
        opcode_from_operation(and_ab),
        0x30,
        0x05
    ]);
    state.mem.set_byte_at_addr(0x0530, 0x55);
    state.execute_next();

    assert_eq!(0x41, state.acc.get());
    assert!(state.sta.is_empty());
}

#[test]
fn test_and_abx() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(0x41);
    state.set_x(0x0A);
    state.set_up_state(&vec![
        opcode_from_operation(and_abx),
        0x30,
        0x05
    ]);
    state.mem.set_byte_at_addr(0x053A, 0x55);
    state.execute_next();

    assert_eq!(0x41, state.acc.get());
    assert!(state.sta.is_empty());
}

#[test]
fn test_and_aby() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(0x41);
    state.set_y(0x0A);
    state.set_up_state(&vec![
        opcode_from_operation(and_aby),
        0x30,
        0x05
    ]);
    state.mem.set_byte_at_addr(0x053A, 0x55);
    state.execute_next();

    assert_eq!(0x41, state.acc.get());
    assert!(state.sta.is_empty());
}

#[test]
fn test_and_inx() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(0x41);
    state.set_x(0x22);
    state.set_up_state(&vec![
        opcode_from_operation(and_inx),
        0x55
    ]);
    state.mem.set_nibble_at_addr(0x77, 0x1234);
    state.mem.set_byte_at_addr(0x1234, 0x43);
    state.execute_next();

    assert_eq!(0x41, state.acc.get());
    assert!(state.sta.is_empty());
}

#[test]
fn test_and_iny() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(0xFF);
    state.set_y(0x22);
    state.set_up_state(&vec![
        opcode_from_operation(and_iny),
        0x41
    ]);
    state.mem.set_nibble_at_addr(0x41, 0x1234);
    state.mem.set_byte_at_addr(0x1256, 0x29);
    state.execute_next();

    assert_eq!(0x29, state.acc.get());
    assert!(state.sta.is_empty());
}


// OR TESTS
#[test]
fn test_or_im() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(0x13);
    state.set_up_state(&vec![
        opcode_from_operation(or_im),
        0x30
    ]);
    state.execute_next();

    assert_eq!(0x33, state.acc.get());
    assert!(state.sta.is_empty());
}

#[test]
fn test_or_im_zero_flag() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(0x00);
    state.set_up_state(&vec![
        opcode_from_operation(or_im),
        0x00
    ]);
    state.execute_next();

    assert_eq!(0x00, state.acc.get());
    assert!(state.sta.contains_only(StatusRegister::Z));
}

#[test]
fn test_or_im_negative_flag() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(0xA1);
    state.set_up_state(&vec![
        opcode_from_operation(or_im),
        0xF0
    ]);
    state.execute_next();

    assert_eq!(0xF1, state.acc.get());
    assert!(state.sta.contains_only(StatusRegister::N));
}

#[test]
fn test_or_zp() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(0x25);
    state.set_up_state(&vec![
        opcode_from_operation(or_zp),
        0xF5
    ]);
    state.mem.set_byte_at_addr(0xF5, 0x10);
    state.execute_next();

    assert_eq!(0x35, state.acc.get());
    assert!(state.sta.is_empty());
}

#[test]
fn test_or_zpx() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(0x42);
    state.set_x(0x50);
    state.set_up_state(&vec![
        opcode_from_operation(or_zpx),
        0x30
    ]);
    state.mem.set_byte_at_addr(0x80, 0x81);
    state.execute_next();

    assert_eq!(0xC3, state.acc.get());
    assert!(state.sta.contains_only(StatusRegister::N));
}

#[test]
fn test_or_ab() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(0x41);
    state.set_up_state(&vec![
        opcode_from_operation(or_ab),
        0x30,
        0x05
    ]);
    state.mem.set_byte_at_addr(0x0530, 0x55);
    state.execute_next();

    assert_eq!(0x55, state.acc.get());
    assert!(state.sta.is_empty());
}

#[test]
fn test_or_abx() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(0x53);
    state.set_x(0x0A);
    state.set_up_state(&vec![
        opcode_from_operation(or_abx),
        0x30,
        0x05
    ]);
    state.mem.set_byte_at_addr(0x053A, 0x40);
    state.execute_next();

    assert_eq!(0x53, state.acc.get());
    assert!(state.sta.is_empty());
}

#[test]
fn test_or_aby() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(0x02);
    state.set_y(0x0A);
    state.set_up_state(&vec![
        opcode_from_operation(or_aby),
        0x30,
        0x05
    ]);
    state.mem.set_byte_at_addr(0x053A, 0x20);
    state.execute_next();

    assert_eq!(0x22, state.acc.get());
    assert!(state.sta.is_empty());
}

#[test]
fn test_or_inx() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(1);
    state.set_x(0x22);
    state.set_up_state(&vec![
        opcode_from_operation(or_inx),
        0x55
    ]);
    state.mem.set_nibble_at_addr(0x77, 0x1234);
    state.mem.set_byte_at_addr(0x1234, 3);
    state.execute_next();

    assert_eq!(3, state.acc.get());
    assert!(state.sta.is_empty());
}

#[test]
fn test_or_iny() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(0xFF);
    state.set_y(0x22);
    state.set_up_state(&vec![
        opcode_from_operation(or_iny),
        0x41
    ]);
    state.mem.set_nibble_at_addr(0x41, 0x1234);
    state.mem.set_byte_at_addr(0x1256, 0x23);
    state.execute_next();

    assert_eq!(0xFF, state.acc.get());
    assert!(state.sta.contains_only(StatusRegister::N));
}


// EOR TESTS
#[test]
fn test_eor_im() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(0x13);
    state.set_up_state(&vec![
        opcode_from_operation(eor_im),
        0x30
    ]);
    state.execute_next();

    assert_eq!(0x23, state.acc.get());
    assert!(state.sta.is_empty());
}

#[test]
fn test_eor_im_zero_flag() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(0x11);
    state.set_up_state(&vec![
        opcode_from_operation(eor_im),
        0x11
    ]);
    state.execute_next();

    assert_eq!(0x00, state.acc.get());
    assert!(state.sta.contains_only(StatusRegister::Z));
}

#[test]
fn test_eor_im_negative_flag() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(0x05);
    state.set_up_state(&vec![
        opcode_from_operation(eor_im),
        0xF0
    ]);
    state.execute_next();

    assert_eq!(0xF5, state.acc.get());
    assert!(state.sta.contains_only(StatusRegister::N));
}

#[test]
fn test_eor_zp() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(0x25);
    state.set_up_state(&vec![
        opcode_from_operation(eor_zp),
        0xF5
    ]);
    state.mem.set_byte_at_addr(0xF5, 0x31);
    state.execute_next();

    assert_eq!(0x14, state.acc.get());
    assert!(state.sta.is_empty());
}

#[test]
fn test_eor_zpx() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(0x23);
    state.set_x(0x50);
    state.set_up_state(&vec![
        opcode_from_operation(eor_zpx),
        0x30
    ]);
    state.mem.set_byte_at_addr(0x80, 0x31);
    state.execute_next();

    assert_eq!(0x12, state.acc.get());
    assert!(state.sta.is_empty());
}

#[test]
fn test_eor_ab() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(0x41);
    state.set_up_state(&vec![
        opcode_from_operation(eor_ab),
        0x30,
        0x05
    ]);
    state.mem.set_byte_at_addr(0x0530, 0x55);
    state.execute_next();

    assert_eq!(0x14, state.acc.get());
    assert!(state.sta.is_empty());
}

#[test]
fn test_eor_abx() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(0x53);
    state.set_x(0x0A);
    state.set_up_state(&vec![
        opcode_from_operation(eor_abx),
        0x30,
        0x05
    ]);
    state.mem.set_byte_at_addr(0x053A, 0x40);
    state.execute_next();

    assert_eq!(0x13, state.acc.get());
    assert!(state.sta.is_empty());
}

#[test]
fn test_eor_aby() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(0x02);
    state.set_y(0x0A);
    state.set_up_state(&vec![
        opcode_from_operation(eor_aby),
        0x30,
        0x05
    ]);
    state.mem.set_byte_at_addr(0x053A, 0x20);
    state.execute_next();

    assert_eq!(0x22, state.acc.get());
    assert!(state.sta.is_empty());
}

#[test]
fn test_eor_inx() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(0x19);
    state.set_x(0x22);
    state.set_up_state(&vec![
        opcode_from_operation(eor_inx),
        0x55
    ]);
    state.mem.set_nibble_at_addr(0x77, 0x1234);
    state.mem.set_byte_at_addr(0x1234, 0x28);
    state.execute_next();

    assert_eq!(0x31, state.acc.get());
    assert!(state.sta.is_empty());
}

#[test]
fn test_eor_iny() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(0x35);
    state.set_y(0x22);
    state.set_up_state(&vec![
        opcode_from_operation(eor_iny),
        0x41
    ]);
    state.mem.set_nibble_at_addr(0x41, 0x1234);
    state.mem.set_byte_at_addr(0x1256, 0x23);
    state.execute_next();

    assert_eq!(0x16, state.acc.get());
    assert!(state.sta.is_empty());
}


// BIT TESTS
#[test]
fn test_bit_zp() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(0x23);
    state.set_up_state(&vec![
        opcode_from_operation(bit_zp),
        0x41
    ]);
    state.mem.set_byte_at_addr(0x41, 0x35);
    state.execute_next();

    assert_eq!(0x23, state.acc.get());
    assert_eq!(0x35, state.mem.fetch_byte_from_addr(0x41));
    assert!(state.sta.is_empty());
}

#[test]
fn test_bit_zp_zero_flag() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(0x11);
    state.set_up_state(&vec![
        opcode_from_operation(bit_zp),
        0x41
    ]);
    state.mem.set_byte_at_addr(0x41, 0x22);
    state.execute_next();

    assert!(state.sta.contains_only(StatusRegister::Z));
}

#[test]
fn test_bit_zp_negative_flag() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(0x23);
    state.set_up_state(&vec![
        opcode_from_operation(bit_zp),
        0x41
    ]);
    state.mem.set_byte_at_addr(0x41, 0xA5);
    state.execute_next();

    assert!(state.sta.contains_only(StatusRegister::N));
}

#[test]
fn test_bit_zp_overflow_flag() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(0x23);
    state.set_up_state(&vec![
        opcode_from_operation(bit_zp),
        0x41
    ]);
    state.mem.set_byte_at_addr(0x41, 0x45);
    state.execute_next();

    assert!(state.sta.contains_only(StatusRegister::V));
}

#[test]
fn test_bit_zp_zero_negative_overflow_flag() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(0x02);
    state.set_up_state(&vec![
        opcode_from_operation(bit_zp),
        0x41
    ]);
    state.mem.set_byte_at_addr(0x41, 0xC5);
    state.execute_next();

    assert!(state.sta.contains_only(StatusRegister::Z | StatusRegister::N | StatusRegister::V));
}

#[test]
fn test_bit_ab() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(0x23);
    state.set_up_state(&vec![
        opcode_from_operation(bit_ab),
        0x41,
        0x90
    ]);
    state.mem.set_byte_at_addr(0x9041, 0x35);
    state.execute_next();

    assert!(state.sta.is_empty());
}


// ASL TESTS
#[test]
fn test_asl_acc() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(0x09);
    state.set_up_state(&vec![
        opcode_from_operation(asl_acc),
    ]);
    state.execute_next();

    assert_eq!(0x12, state.acc.get());
    assert!(state.sta.is_empty());
}

#[test]
fn test_asl_acc_carry_flag() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(0x99);
    state.set_up_state(&vec![
        opcode_from_operation(asl_acc),
    ]);
    state.execute_next();

    assert_eq!(0x32, state.acc.get());
    assert!(state.sta.contains_only(StatusRegister::C));
}

#[test]
fn test_asl_acc_zero_flag() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(0x80);
    state.set_up_state(&vec![
        opcode_from_operation(asl_acc),
    ]);
    state.execute_next();

    assert_eq!(0x0, state.acc.get());
    assert!(state.sta.contains_only(StatusRegister::C | StatusRegister::Z));
}

#[test]
fn test_asl_acc_negative_flag() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(0xC0);
    state.set_up_state(&vec![
        opcode_from_operation(asl_acc),
    ]);
    state.execute_next();

    assert_eq!(0x80, state.acc.get());
    assert!(state.sta.contains_only(StatusRegister::C | StatusRegister::N));
}

#[test]
fn test_asl_zp() {
    let mut state: ComputerState = ComputerState::new();
    state.set_up_state(&vec![
        opcode_from_operation(asl_zp),
        0x10
    ]);
    state.mem.set_byte_at_addr(0x10, 0x09);
    state.execute_next();

    assert_eq!(0x12, state.mem.fetch_byte_from_addr(0x10));
    assert!(state.sta.is_empty());
}

#[test]
fn test_asl_zpx() {
    let mut state: ComputerState = ComputerState::new();
    state.set_x(0x05);
    state.set_up_state(&vec![
        opcode_from_operation(asl_zpx),
        0x10
    ]);
    state.mem.set_byte_at_addr(0x15, 0x09);
    state.execute_next();

    assert_eq!(0x12, state.mem.fetch_byte_from_addr(0x15));
    assert!(state.sta.is_empty());
}

#[test]
fn test_asl_ab() {
    let mut state: ComputerState = ComputerState::new();
    state.set_up_state(&vec![
        opcode_from_operation(asl_ab),
        0x10,
        0x85
    ]);
    state.mem.set_byte_at_addr(0x8510, 0x09);
    state.execute_next();

    assert_eq!(0x12, state.mem.fetch_byte_from_addr(0x8510));
    assert!(state.sta.is_empty());
}

#[test]
fn test_asl_abx() {
    let mut state: ComputerState = ComputerState::new();
    state.set_x(0x05);
    state.set_up_state(&vec![
        opcode_from_operation(asl_abx),
        0x10,
        0x85
    ]);
    state.mem.set_byte_at_addr(0x8515, 0x09);
    state.execute_next();

    assert_eq!(0x12, state.mem.fetch_byte_from_addr(0x8515));
    assert!(state.sta.is_empty());
}


// LSR TESTS
#[test]
fn test_lsr_acc() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(8);
    state.set_up_state(&vec![
        opcode_from_operation(lsr_acc),
    ]);
    state.execute_next();

    assert_eq!(4, state.acc.get());
    assert!(state.sta.is_empty());
}

#[test]
fn test_lsr_acc_carry_flag() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(9);
    state.set_up_state(&vec![
        opcode_from_operation(lsr_acc),
    ]);
    state.execute_next();

    assert_eq!(4, state.acc.get());
    assert!(state.sta.contains_only(StatusRegister::C));
}

#[test]
fn test_lsr_acc_zero_flag() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(1);
    state.set_up_state(&vec![
        opcode_from_operation(lsr_acc),
    ]);
    state.execute_next();

    assert_eq!(0, state.acc.get());
    assert!(state.sta.contains_only(StatusRegister::C | StatusRegister::Z));
}

#[test]
fn test_lsr_zp() {
    let mut state: ComputerState = ComputerState::new();
    state.set_up_state(&vec![
        opcode_from_operation(lsr_zp),
        0x05
    ]);
    state.mem.set_byte_at_addr(0x05, 8);
    state.execute_next();

    assert_eq!(4, state.mem.fetch_byte_from_addr(0x05));
    assert!(state.sta.is_empty());
}

#[test]
fn test_lsr_zpx() {
    let mut state: ComputerState = ComputerState::new();
    state.set_x(0x05);
    state.set_up_state(&vec![
        opcode_from_operation(lsr_zpx),
        0x05
    ]);
    state.mem.set_byte_at_addr(0x0A, 8);
    state.execute_next();

    assert_eq!(4, state.mem.fetch_byte_from_addr(0x0A));
    assert!(state.sta.is_empty());
}

#[test]
fn test_lsr_ab() {
    let mut state: ComputerState = ComputerState::new();
    state.set_up_state(&vec![
        opcode_from_operation(lsr_ab),
        0x05,
        0x08
    ]);
    state.mem.set_byte_at_addr(0x0805, 8);
    state.execute_next();

    assert_eq!(4, state.mem.fetch_byte_from_addr(0x0805));
    assert!(state.sta.is_empty());
}

#[test]
fn test_lsr_abx() {
    let mut state: ComputerState = ComputerState::new();
    state.set_x(0x05);
    state.set_up_state(&vec![
        opcode_from_operation(lsr_abx),
        0x05,
        0x08
    ]);
    state.mem.set_byte_at_addr(0x080A, 8);
    state.execute_next();

    assert_eq!(4, state.mem.fetch_byte_from_addr(0x080A));
    assert!(state.sta.is_empty());
}


// ROL TESTS
#[test]
fn test_rol_acc() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(0x09);
    state.set_up_state(&vec![
        opcode_from_operation(rol_acc),
    ]);
    state.execute_next();

    assert_eq!(0x12, state.acc.get());
    assert!(state.sta.is_empty());
}

#[test]
fn test_rol_acc_old_carry() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(0x09);
    state.sta |= StatusRegister::C;
    state.set_up_state(&vec![
        opcode_from_operation(rol_acc),
    ]);
    state.execute_next();

    assert_eq!(0x13, state.acc.get());
    assert!(state.sta.is_empty());
}

#[test]
fn test_rol_acc_carry_flag() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(0x89);
    state.sta |= StatusRegister::C;
    state.set_up_state(&vec![
        opcode_from_operation(rol_acc),
    ]);
    state.execute_next();

    assert_eq!(0x13, state.acc.get());
    assert!(state.sta.contains_only(StatusRegister::C));
}

#[test]
fn test_rol_acc_negative_flag() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(0xC9);
    state.sta |= StatusRegister::C;
    state.set_up_state(&vec![
        opcode_from_operation(rol_acc),
    ]);
    state.execute_next();

    assert_eq!(0x93, state.acc.get());
    assert!(state.sta.contains_only(StatusRegister::C | StatusRegister::N));
}

#[test]
fn test_rol_acc_zero_flag() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(0x80);
    state.set_up_state(&vec![
        opcode_from_operation(rol_acc),
    ]);
    state.execute_next();

    assert_eq!(0x0, state.acc.get());
    assert!(state.sta.contains_only(StatusRegister::C | StatusRegister::Z));
}

#[test]
fn test_rol_zp() {
    let mut state: ComputerState = ComputerState::new();
    state.set_up_state(&vec![
        opcode_from_operation(rol_zp),
        0x10
    ]);
    state.mem.set_byte_at_addr(0x10, 0x9);
    state.execute_next();

    assert_eq!(0x12, state.mem.fetch_byte_from_addr(0x10));
    assert!(state.sta.is_empty());
}

#[test]
fn test_rol_zpx() {
    let mut state: ComputerState = ComputerState::new();
    state.set_x(0x05);
    state.set_up_state(&vec![
        opcode_from_operation(rol_zpx),
        0x10
    ]);
    state.mem.set_byte_at_addr(0x15, 0x9);
    state.execute_next();

    assert_eq!(0x12, state.mem.fetch_byte_from_addr(0x15));
    assert!(state.sta.is_empty());
}

#[test]
fn test_rol_ab() {
    let mut state: ComputerState = ComputerState::new();
    state.set_up_state(&vec![
        opcode_from_operation(rol_ab),
        0x10,
        0x11
    ]);
    state.mem.set_byte_at_addr(0x1110, 0x9);
    state.execute_next();

    assert_eq!(0x12, state.mem.fetch_byte_from_addr(0x1110));
    assert!(state.sta.is_empty());
}

#[test]
fn test_rol_abx() {
    let mut state: ComputerState = ComputerState::new();
    state.set_x(0x05);
    state.set_up_state(&vec![
        opcode_from_operation(rol_abx),
        0x10,
        0x11
    ]);
    state.mem.set_byte_at_addr(0x1115, 0x9);
    state.execute_next();

    assert_eq!(0x12, state.mem.fetch_byte_from_addr(0x1115));
    assert!(state.sta.is_empty());
}


// ROR TESTS
#[test]
fn test_ror_acc() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(0x08);
    state.set_up_state(&vec![
        opcode_from_operation(ror_acc),
    ]);
    state.execute_next();

    assert_eq!(0x04, state.acc.get());
    assert!(state.sta.is_empty());
}

#[test]
fn test_ror_acc_old_carry() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(0x08);
    state.sta |= StatusRegister::C;
    state.set_up_state(&vec![
        opcode_from_operation(ror_acc),
    ]);
    state.execute_next();

    assert_eq!(0x84, state.acc.get());
    assert!(state.sta.contains_only(StatusRegister::N));
}

#[test]
fn test_ror_acc_carry_negative_flag() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(0x09);
    state.sta |= StatusRegister::C;
    state.set_up_state(&vec![
        opcode_from_operation(ror_acc),
    ]);
    state.execute_next();

    assert_eq!(0x84, state.acc.get());
    assert!(state.sta.contains_only(StatusRegister::C | StatusRegister::N));
}

#[test]
fn test_ror_acc_zero_flag() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(0x01);
    state.set_up_state(&vec![
        opcode_from_operation(ror_acc),
    ]);
    state.execute_next();

    assert_eq!(0x0, state.acc.get());
    assert!(state.sta.contains_only(StatusRegister::C | StatusRegister::Z));
}

#[test]
fn test_ror_zp() {
    let mut state: ComputerState = ComputerState::new();
    state.set_up_state(&vec![
        opcode_from_operation(ror_zp),
        0x10
    ]);
    state.mem.set_byte_at_addr(0x10, 0x08);
    state.execute_next();

    assert_eq!(0x04, state.mem.fetch_byte_from_addr(0x10));
    assert!(state.sta.is_empty());
}

#[test]
fn test_ror_zpx() {
    let mut state: ComputerState = ComputerState::new();
    state.set_x(0x05);
    state.set_up_state(&vec![
        opcode_from_operation(ror_zpx),
        0x10
    ]);
    state.mem.set_byte_at_addr(0x15, 0x08);
    state.execute_next();

    assert_eq!(0x04, state.mem.fetch_byte_from_addr(0x15));
    assert!(state.sta.is_empty());
}

#[test]
fn test_ror_ab() {
    let mut state: ComputerState = ComputerState::new();
    state.set_up_state(&vec![
        opcode_from_operation(ror_ab),
        0x10,
        0x75
    ]);
    state.mem.set_byte_at_addr(0x7510, 0x08);
    state.execute_next();

    assert_eq!(0x04, state.mem.fetch_byte_from_addr(0x7510));
    assert!(state.sta.is_empty());
}

#[test]
fn test_ror_abx() {
    let mut state: ComputerState = ComputerState::new();
    state.set_x(0x05);
    state.set_up_state(&vec![
        opcode_from_operation(ror_abx),
        0x10,
        0x75
    ]);
    state.mem.set_byte_at_addr(0x7515, 0x08);
    state.execute_next();

    assert_eq!(0x04, state.mem.fetch_byte_from_addr(0x7515));
    assert!(state.sta.is_empty());
}
