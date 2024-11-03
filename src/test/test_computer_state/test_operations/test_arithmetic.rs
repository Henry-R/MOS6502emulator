
use crate::computer_state::{ComputerState};
use crate::computer_state::operations::arithmetic::*;
use crate::computer_state::operations::opcode_from_operation;
use crate::computer_state::status_register::StatusRegister;
// Unit tests shamelessly taken from https://github.com/tdinucci/6502-emulator
// All credit for coming up with the test cases goes to tdinucci

// ADC TESTS
#[test]
fn test_adc_im() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(36);
    state.set_up_state(vec![
        opcode_from_operation(adc_im),
        36
    ]);
    state.execute_next();

    assert_eq!(72, state.acc.get());
    assert!(state.sta.is_empty());
}

#[test]
fn test_adc_im_zero_flag() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(0);
    state.set_up_state(vec![
        opcode_from_operation(adc_im),
        0
    ]);
    state.execute_next();

    assert_eq!(0, state.acc.get());
    assert!(state.sta.contains_only(StatusRegister::Z));
}

#[test]
fn test_adc_im_carry_flag() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(72);
    state.set_up_state(vec![
        opcode_from_operation(adc_im),
        200
    ]);
    state.execute_next();

    assert_eq!(16, state.acc.get());
    assert!(state.sta.contains_only(StatusRegister::C));
}

#[test]
fn test_adc_im_zero_and_carry_flag() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(1);
    state.set_up_state(vec![
        opcode_from_operation(adc_im),
        0xFF
    ]);
    state.execute_next();

    assert_eq!(0, state.acc.get());
    assert!(state.sta.contains_only(StatusRegister::Z | StatusRegister::C));
}

#[test]
fn test_adc_im_negative_flag() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(32);
    state.set_up_state(vec![
        opcode_from_operation(adc_im),
        200
    ]);
    state.execute_next();

    assert_eq!(232, state.acc.get());
    assert!(state.sta.contains_only(StatusRegister::N));
}

#[test]
fn test_adc_im_overflow_positive() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(32);
    state.set_up_state(vec![
        opcode_from_operation(adc_im),
        120
    ]);
    state.execute_next();

    assert_eq!(152, state.acc.get());
    assert!(state.sta.contains_only(StatusRegister::N | StatusRegister::V));
}

#[test]
fn test_adc_im_overflow_negative() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(144);
    state.set_up_state(vec![
        opcode_from_operation(adc_im),
        208
    ]);
    state.execute_next();

    assert_eq!(96, state.acc.get());
    assert!(state.sta.contains_only(StatusRegister::C | StatusRegister::V));
}

#[test]
fn test_adc_zp() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(36);
    state.set_up_state(vec![
        opcode_from_operation(adc_zp),
        0xF1
    ]);
    state.mem.set_byte_at_addr(0xF1, 20);
    state.execute_next();

    assert_eq!(56, state.acc.get());
    assert!(state.sta.is_empty());
}

#[test]
fn test_adc_zpx() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(36);
    state.set_x(5);
    state.set_up_state(vec![
        opcode_from_operation(adc_zpx),
        0xF1
    ]);
    state.mem.set_byte_at_addr(0xF6, 20);
    state.execute_next();

    assert_eq!(56, state.acc.get());
    assert!(state.sta.is_empty());
}

#[test]
fn test_adc_ab() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(36);
    state.set_up_state(vec![
        opcode_from_operation(adc_ab),
        0xF1,
        0x36
    ]);
    state.mem.set_byte_at_addr(0x36F1, 20);
    state.execute_next();

    assert_eq!(56, state.acc.get());
    assert!(state.sta.is_empty());
}

#[test]
fn test_adc_abx() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(36);
    state.set_x(8);
    state.set_up_state(vec![
        opcode_from_operation(adc_abx),
        0xF1,
        0x36
    ]);
    state.mem.set_byte_at_addr(0x36F9, 20);
    state.execute_next();

    assert_eq!(56, state.acc.get());
    assert!(state.sta.is_empty());
}

#[test]
fn test_adc_aby() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(36);
    state.set_y(8);
    state.set_up_state(vec![
        opcode_from_operation(adc_aby),
        0xF1,
        0x36
    ]);
    state.mem.set_byte_at_addr(0x36F9, 20);
    state.execute_next();

    assert_eq!(56, state.acc.get());
    assert!(state.sta.is_empty());
}

#[test]
fn test_adc_inx() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(36);
    state.set_x(0x22);
    state.set_up_state(vec![
        opcode_from_operation(adc_inx),
        0x41
    ]);
    state.mem.set_nibble_at_addr(0x63, 0x1234);
    state.mem.set_byte_at_addr(0x1234, 20);
    state.execute_next();

    assert_eq!(56, state.acc.get());
    assert!(state.sta.is_empty());
}

#[test]
fn test_adc_iny() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(36);
    state.set_y(0x22);
    state.set_up_state(vec![
        opcode_from_operation(adc_iny),
        0x41
    ]);
    state.mem.set_nibble_at_addr(0x41, 0x1234);
    state.mem.set_byte_at_addr(0x1256, 20);
    state.execute_next();

    assert_eq!(56, state.acc.get());
    assert!(state.sta.is_empty());
}


// SBC TESTS
#[test]
fn test_sbc_im() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(0x50);
    state.set_up_state(vec![
        opcode_from_operation(sbc_im),
        0x20
    ]);
    state.execute_next();

    assert_eq!(0x30, state.acc.get());
    assert!(state.sta.contains_only(StatusRegister::C));
}

#[test]
fn test_sbc_im_borrow() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(0xD0);
    state.set_up_state(vec![
        opcode_from_operation(sbc_im),
        0xF0
    ]);
    state.execute_next();

    assert_eq!(0xE0, state.acc.get());
    assert!(state.sta.contains_only(StatusRegister::N));
}

#[test]
fn test_sbc_im_overflow() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(0x50);
    state.set_up_state(vec![
        opcode_from_operation(sbc_im),
        0xB0
    ]);
    state.execute_next();

    assert_eq!(0xA0, state.acc.get());
    assert!(state.sta.contains_only(StatusRegister::N | StatusRegister::V));
}

#[test]
fn test_sbc_im_zero_flag() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(50);
    state.set_up_state(vec![
        opcode_from_operation(sbc_im),
        50
    ]);
    state.execute_next();

    assert_eq!(0, state.acc.get());
    assert!(state.sta.contains_only(StatusRegister::Z | StatusRegister::C));
}

#[test]
fn test_sbc_zp() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(36);
    state.set_up_state(vec![
        opcode_from_operation(sbc_zp),
        0xF1
    ]);
    state.mem.set_byte_at_addr(0xF1, 20);
    state.execute_next();

    assert_eq!(16, state.acc.get());
    assert!(state.sta.contains_only(StatusRegister::C));
}

#[test]
fn test_sbc_zpx() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(36);
    state.set_x(5);
    state.set_up_state(vec![
        opcode_from_operation(sbc_zpx),
        0xF1
    ]);
    state.mem.set_byte_at_addr(0xF6, 20);
    state.execute_next();

    assert_eq!(16, state.acc.get());
    assert!(state.sta.contains_only(StatusRegister::C));
}

#[test]
fn test_sbc_ab() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(36);
    state.set_up_state(vec![
        opcode_from_operation(sbc_ab),
        0xF1,
        0x36
    ]);
    state.mem.set_byte_at_addr(0x36F1, 20);
    state.execute_next();

    assert_eq!(16, state.acc.get());
    assert!(state.sta.contains_only(StatusRegister::C));
}

#[test]
fn test_sbc_abx() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(36);
    state.set_x(8);
    state.set_up_state(vec![
        opcode_from_operation(sbc_abx),
        0xF1,
        0x36
    ]);
    state.mem.set_byte_at_addr(0x36F9, 20);
    state.execute_next();

    assert_eq!(16, state.acc.get());
    assert!(state.sta.contains_only(StatusRegister::C));
}

#[test]
fn test_sbc_aby() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(36);
    state.set_y(8);
    state.set_up_state(vec![
        opcode_from_operation(sbc_aby),
        0xF1,
        0x36
    ]);
    state.mem.set_byte_at_addr(0x36F9, 20);
    state.execute_next();

    assert_eq!(16, state.acc.get());
    assert!(state.sta.contains_only(StatusRegister::C));
}

#[test]
fn test_sbc_inx() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(36);
    state.set_x(0x22);
    state.set_up_state(vec![
        opcode_from_operation(sbc_inx),
        0x41
    ]);
    state.mem.set_nibble_at_addr(0x63, 0x1234);
    state.mem.set_byte_at_addr(0x1234, 20);
    state.execute_next();

    assert_eq!(16, state.acc.get());
    assert!(state.sta.contains_only(StatusRegister::C));
}

#[test]
fn test_sbc_iny() {
    let mut state: ComputerState = ComputerState::new();
    state.acc.set(36);
    state.set_y(0x22);
    state.set_up_state(vec![
        opcode_from_operation(sbc_iny),
        0x41
    ]);
    state.mem.set_nibble_at_addr(0x41, 0x1234);
    state.mem.set_byte_at_addr(0x1256, 20);
    state.execute_next();

    assert_eq!(16, state.acc.get());
    assert!(state.sta.contains_only(StatusRegister::C));
}


// DEC TESTS
#[test]
fn test_dec_zp() {
    let mut state: ComputerState = ComputerState::new();
    state.set_up_state(vec![
        opcode_from_operation(dec_zp),
        0xF1
    ]);
    state.mem.set_byte_at_addr(0xF1, 20);
    state.execute_next();

    assert_eq!(19, state.mem.fetch_byte_from_addr(0xF1));
    assert!(state.sta.is_empty());
}

#[test]
fn test_dec_zp_zero_flag() {
    let mut state: ComputerState = ComputerState::new();
    state.set_up_state(vec![
        opcode_from_operation(dec_zp),
        0xF1
    ]);
    state.mem.set_byte_at_addr(0xF1, 1);
    state.execute_next();

    assert_eq!(0, state.mem.fetch_byte_from_addr(0xF1));
    assert!(state.sta.contains_only(StatusRegister::Z));
}

#[test]
fn test_dec_zp_negative_flag() {
    let mut state: ComputerState = ComputerState::new();
    state.set_up_state(vec![
        opcode_from_operation(dec_zp),
        0xF1
    ]);
    state.mem.set_byte_at_addr(0xF1, 0);
    state.execute_next();

    assert_eq!(0xFF, state.mem.fetch_byte_from_addr(0xF1));
    assert!(state.sta.contains_only(StatusRegister::N));
}

#[test]
fn test_dec_zpx() {
    let mut state: ComputerState = ComputerState::new();
    state.set_x(0x10);
    state.set_up_state(vec![
        opcode_from_operation(dec_zpx),
        0x45
    ]);
    state.mem.set_byte_at_addr(0x55, 20);
    state.execute_next();

    assert_eq!(19, state.mem.fetch_byte_from_addr(0x55));
    assert!(state.sta.is_empty());
}

#[test]
fn test_dec_ab() {
    let mut state: ComputerState = ComputerState::new();
    state.set_up_state(vec![
        opcode_from_operation(dec_ab),
        0x45,
        0x1A
    ]);
    state.mem.set_byte_at_addr(0x1A45, 20);
    state.execute_next();

    assert_eq!(19, state.mem.fetch_byte_from_addr(0x1A45));
    assert!(state.sta.is_empty());
}

#[test]
fn test_dec_abx() {
    let mut state: ComputerState = ComputerState::new();
    state.set_x(0x10);
    state.set_up_state(vec![
        opcode_from_operation(dec_abx),
        0x45,
        0x1A
    ]);
    state.mem.set_byte_at_addr(0x1A55, 20);
    state.execute_next();

    assert_eq!(19, state.mem.fetch_byte_from_addr(0x1A55));
    assert!(state.sta.is_empty());
}


// INC TESTS
#[test]
fn test_inc_zp() {
    let mut state: ComputerState = ComputerState::new();
    state.set_up_state(vec![
        opcode_from_operation(inc_zp),
        0xF1
    ]);
    state.mem.set_byte_at_addr(0xF1, 20);
    state.execute_next();

    assert_eq!(21, state.mem.fetch_byte_from_addr(0xF1));
    assert!(state.sta.is_empty());
}

#[test]
fn test_inc_zp_zero_flag() {
    let mut state: ComputerState = ComputerState::new();
    state.set_up_state(vec![
        opcode_from_operation(inc_zp),
        0xF1
    ]);
    state.mem.set_byte_at_addr(0xF1, 0xFF);
    state.execute_next();

    assert_eq!(0, state.mem.fetch_byte_from_addr(0xF1));
    assert!(state.sta.contains_only(StatusRegister::Z));
}

#[test]
fn test_inc_zp_negative_flag() {
    let mut state: ComputerState = ComputerState::new();
    state.set_up_state(vec![
        opcode_from_operation(inc_zp),
        0xF1
    ]);
    state.mem.set_byte_at_addr(0xF1, 0x7F);
    state.execute_next();

    assert_eq!(0x80, state.mem.fetch_byte_from_addr(0xF1));
    assert!(state.sta.contains_only(StatusRegister::N));
}

#[test]
fn test_inc_zpx() {
    let mut state: ComputerState = ComputerState::new();
    state.set_x(0x10);
    state.set_up_state(vec![
        opcode_from_operation(inc_zpx),
        0x45
    ]);
    state.mem.set_byte_at_addr(0x55, 20);
    state.execute_next();

    assert_eq!(21, state.mem.fetch_byte_from_addr(0x55));
    assert!(state.sta.is_empty());
}

#[test]
fn test_inc_ab() {
    let mut state: ComputerState = ComputerState::new();
    state.set_up_state(vec![
        opcode_from_operation(inc_ab),
        0x45,
        0x1A
    ]);
    state.mem.set_byte_at_addr(0x1A45, 20);
    state.execute_next();

    assert_eq!(21, state.mem.fetch_byte_from_addr(0x1A45));
    assert!(state.sta.is_empty());
}

#[test]
fn test_inc_abx() {
    let mut state: ComputerState = ComputerState::new();
    state.set_x(0x10);
    state.set_up_state(vec![
        opcode_from_operation(inc_abx),
        0x45,
        0x1A
    ]);
    state.mem.set_byte_at_addr(0x1A55, 20);
    state.execute_next();

    assert_eq!(21, state.mem.fetch_byte_from_addr(0x1A55));
    assert!(state.sta.is_empty());
}


// TEST DEX
#[test]
fn test_dex() {
    let mut state: ComputerState = ComputerState::new();
    state.set_x(5);
    state.set_up_state(vec![
        opcode_from_operation(dex),
    ]);
    state.execute_next();

    assert_eq!(4, state.get_x());
    assert!(state.sta.is_empty());
}

#[test]
fn test_dex_zero_flag() {
    let mut state: ComputerState = ComputerState::new();
    state.set_x(1);
    state.set_up_state(vec![
        opcode_from_operation(dex),
    ]);
    state.execute_next();

    assert_eq!(0, state.get_x());
    assert!(state.sta.contains_only(StatusRegister::Z));
}

#[test]
fn test_dex_negative_flag() {
    let mut state: ComputerState = ComputerState::new();
    state.set_x(0);
    state.set_up_state(vec![
        opcode_from_operation(dex),
    ]);
    state.execute_next();

    assert_eq!(0xFF, state.get_x());
    assert!(state.sta.contains_only(StatusRegister::N));
}


// TEST DEY
#[test]
fn test_dey() {
    let mut state: ComputerState = ComputerState::new();
    state.set_y(5);
    state.set_up_state(vec![
        opcode_from_operation(dey),
    ]);
    state.execute_next();

    assert_eq!(4, state.get_y());
    assert!(state.sta.is_empty());
}

#[test]
fn test_dey_zero_flag() {
    let mut state: ComputerState = ComputerState::new();
    state.set_y(1);
    state.set_up_state(vec![
        opcode_from_operation(dey),
    ]);
    state.execute_next();

    assert_eq!(0, state.get_y());
    assert!(state.sta.contains_only(StatusRegister::Z));
}

#[test]
fn test_dey_negative_flag() {
    let mut state: ComputerState = ComputerState::new();
    state.set_y(0);
    state.set_up_state(vec![
        opcode_from_operation(dey),
    ]);
    state.execute_next();

    assert_eq!(0xFF, state.get_y());
    assert!(state.sta.contains_only(StatusRegister::N));
}


// TEST INX
#[test]
fn test_inx() {
    let mut state: ComputerState = ComputerState::new();
    state.set_x(5);
    state.set_up_state(vec![
        opcode_from_operation(inx),
    ]);
    state.execute_next();

    assert_eq!(6, state.get_x());
    assert!(state.sta.is_empty());
}

#[test]
fn test_inx_zero_flag() {
    let mut state: ComputerState = ComputerState::new();
    state.set_x(0xFF);
    state.set_up_state(vec![
        opcode_from_operation(inx),
    ]);
    state.execute_next();

    assert_eq!(0, state.get_x());
    assert!(state.sta.contains_only(StatusRegister::Z));
}

#[test]
fn test_inx_negative_flag() {
    let mut state: ComputerState = ComputerState::new();
    state.set_x(0x7F);
    state.set_up_state(vec![
        opcode_from_operation(inx),
    ]);
    state.execute_next();

    assert_eq!(0x80, state.get_x());
    assert!(state.sta.contains_only(StatusRegister::N));
}


// TEST INY
#[test]
fn test_iny() {
    let mut state: ComputerState = ComputerState::new();
    state.set_y(5);
    state.set_up_state(vec![
        opcode_from_operation(iny),
    ]);
    state.execute_next();

    assert_eq!(6, state.get_y());
    assert!(state.sta.is_empty());
}

#[test]
fn test_iny_zero_flag() {
    let mut state: ComputerState = ComputerState::new();
    state.set_y(0xFF);
    state.set_up_state(vec![
        opcode_from_operation(iny),
    ]);
    state.execute_next();

    assert_eq!(0, state.get_y());
    assert!(state.sta.contains_only(StatusRegister::Z));
}

#[test]
fn test_iny_negative_flag() {
    let mut state: ComputerState = ComputerState::new();
    state.set_y(0xFF);
    state.set_up_state(vec![
        opcode_from_operation(iny),
    ]);
    state.execute_next();

    assert_eq!(0, state.get_y());
    assert!(state.sta.contains_only(StatusRegister::Z));
}
