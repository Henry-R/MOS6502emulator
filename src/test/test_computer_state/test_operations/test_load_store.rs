use crate::computer_state::ComputerState;
use crate::computer_state::operations::load_store::*;
use crate::computer_state::operations::opcode_from_operation;
use crate::computer_state::status_register::StatusRegister;

#[test]
fn test_lda_im() {
    let mut state = ComputerState::new();
    state.set_up_state(&vec![
       opcode_from_operation(lda_im),
       36
    ]);
    state.execute_next();

    assert_eq!(36, state.acc.get());
    assert!(state.sta.is_empty())
}

#[test]
fn test_lda_im_zero_flag() {
    let mut state = ComputerState::new();
    state.set_up_state(&vec![
        opcode_from_operation(lda_im),
        0
    ]);
    state.execute_next();

    assert_eq!(0, state.acc.get());
    assert!(state.sta.contains(StatusRegister::Z));
}

#[test]
fn test_lda_im_negative_flag() {
    let mut state = ComputerState::new();
    state.set_up_state(&vec![
        opcode_from_operation(lda_im),
        128
    ]);
    state.execute_next();

    assert_eq!(128, state.acc.get());
    assert!(state.sta.contains(StatusRegister::N));
}

#[test]
fn test_lda_zp() {
    let mut state = ComputerState::new();
    state.set_up_state(&vec![
        opcode_from_operation(lda_zp),
        123
    ]);
    state.mem.set_byte_at_addr(123, 78);
    state.execute_next();

    assert_eq!(78, state.acc.get());
    assert!(state.sta.is_empty())
}

#[test]
fn test_lda_zp_zero_flag() {
    let mut state = ComputerState::new();
    state.set_up_state(&vec![
        opcode_from_operation(lda_zp),
        123
    ]);
    state.mem.set_byte_at_addr(123, 0);
    state.execute_next();

    assert_eq!(0, state.acc.get());
    assert!(state.sta.contains(StatusRegister::Z))
}

#[test]
fn test_lda_zp_negative_flag() {
    let mut state = ComputerState::new();
    state.set_up_state(&vec![
        opcode_from_operation(lda_zp),
        123
    ]);
    state.mem.set_byte_at_addr(123, 150);
    state.execute_next();

    assert_eq!(150, state.acc.get());
    assert!(state.sta.contains(StatusRegister::N))
}

#[test]
fn test_lda_zpx() {
    let mut state = ComputerState::new();
    state.set_x(0x15);
    state.set_up_state(&vec![
        opcode_from_operation(lda_zpx),
        0x10
    ]);
    state.mem.set_byte_at_addr(0x25, 78);
    state.execute_next();

    assert_eq!(78, state.acc.get());
    assert!(state.sta.is_empty())
}

#[test]
fn test_lda_zpx_wrap() {
    let mut state = ComputerState::new();
    state.set_x(0x80);
    state.set_up_state(&vec![
        opcode_from_operation(lda_zpx),
        0xFF
    ]);
    state.mem.set_byte_at_addr(0x7F, 78);
    state.execute_next();

    assert_eq!(78, state.acc.get());
    assert!(state.sta.is_empty())
}

#[test]
fn test_lda_zpx_zero_flag() {
    let mut state = ComputerState::new();
    state.set_x(0x10);
    state.set_up_state(&vec![
        opcode_from_operation(lda_zpx),
        0x15
    ]);
    state.mem.set_byte_at_addr(0x25, 0);
    state.execute_next();

    assert_eq!(0, state.acc.get());
    assert!(state.sta.contains(StatusRegister::Z))
}

#[test]
fn test_lda_zpx_negative_flag() {
    let mut state = ComputerState::new();
    state.set_x(0x10);
    state.set_up_state(&vec![
        opcode_from_operation(lda_zpx),
        0x15
    ]);
    state.mem.set_byte_at_addr(0x25, 0xF5);
    state.execute_next();

    assert_eq!(0xF5, state.acc.get());
    assert!(state.sta.contains(StatusRegister::N))
}

#[test]
fn test_lda_ab() {
    let mut state = ComputerState::new();
    state.set_up_state(&vec![
        opcode_from_operation(lda_ab),
        0x55,
        0x66
    ]);
    state.mem.set_byte_at_addr(0x6655, 0x10);
    state.execute_next();

    assert_eq!(0x10, state.acc.get());
    assert!(state.sta.is_empty())
}

#[test]
fn test_lda_ab_zero_flag() {
    let mut state = ComputerState::new();
    state.set_up_state(&vec![
        opcode_from_operation(lda_ab),
        0x55,
        0x66
    ]);
    state.mem.set_byte_at_addr(0x6655, 0);
    state.execute_next();

    assert_eq!(0, state.acc.get());
    assert!(state.sta.contains(StatusRegister::Z))
}

#[test]
fn test_lda_ab_negative_flag() {
    let mut state = ComputerState::new();
    state.set_up_state(&vec![
        opcode_from_operation(lda_ab),
        0x55,
        0x66
    ]);
    state.mem.set_byte_at_addr(0x6655, 0xF6);
    state.execute_next();

    assert_eq!(0xF6, state.acc.get());
    assert!(state.sta.contains(StatusRegister::N))
}

#[test]
fn test_lda_abx() {
    let mut state = ComputerState::new();
    state.set_x(0x4);
    state.set_up_state(&vec![
        opcode_from_operation(lda_abx),
        0x55,
        0x66
    ]);
    state.mem.set_byte_at_addr(0x6659, 0x10);
    state.execute_next();

    assert_eq!(0x10, state.acc.get());
    assert!(state.sta.is_empty())
}

#[test]
fn test_lda_abx_zero_flag() {
    let mut state = ComputerState::new();
    state.set_x(0x4);
    state.set_up_state(&vec![
        opcode_from_operation(lda_abx),
        0x55,
        0x66
    ]);
    state.mem.set_byte_at_addr(0x6659, 0);
    state.execute_next();

    assert_eq!(0, state.acc.get());
    assert!(state.sta.contains(StatusRegister::Z))
}

#[test]
fn test_lda_abx_negative_flag() {
    let mut state = ComputerState::new();
    state.set_x(0x4);
    state.set_up_state(&vec![
        opcode_from_operation(lda_abx),
        0x55,
        0x66
    ]);
    state.mem.set_byte_at_addr(0x6659, 0xF6);
    state.execute_next();

    assert_eq!(0xF6, state.acc.get());
    assert!(state.sta.contains(StatusRegister::N))
}

#[test]
fn test_lda_aby() {
    let mut state = ComputerState::new();
    state.set_y(0x4);
    state.set_up_state(&vec![
        opcode_from_operation(lda_aby),
        0x55,
        0x66
    ]);
    state.mem.set_byte_at_addr(0x6659, 0x10);
    state.execute_next();

    assert_eq!(0x10, state.acc.get());
    assert!(state.sta.is_empty())
}

#[test]
fn test_lda_aby_zero_flag() {
    let mut state = ComputerState::new();
    state.set_y(0x4);
    state.set_up_state(&vec![
        opcode_from_operation(lda_aby),
        0x55,
        0x66
    ]);
    state.mem.set_byte_at_addr(0x6659, 0);
    state.execute_next();

    assert_eq!(0, state.acc.get());
    assert!(state.sta.contains(StatusRegister::Z))
}

#[test]
fn test_lda_aby_negative_flag() {
    let mut state = ComputerState::new();
    state.set_y(0x4);
    state.set_up_state(&vec![
        opcode_from_operation(lda_aby),
        0x55,
        0x66
    ]);
    state.mem.set_byte_at_addr(0x6659, 0xF6);
    state.execute_next();

    assert_eq!(0xF6, state.acc.get());
    assert!(state.sta.contains(StatusRegister::N))
}

#[test]
fn test_lda_inx() {
    let mut state = ComputerState::new();
    state.set_x(0x4);
    state.set_up_state(&vec![
        opcode_from_operation(lda_inx),
        0x55
    ]);
    // Pointer
    state.mem.set_nibble_at_addr(0x59, 0x4D3C);
    // Value
    state.mem.set_byte_at_addr(0x4D3C, 0x33);
    state.execute_next();

    assert_eq!(0x33, state.acc.get());
    assert!(state.sta.is_empty())
}

#[test]
fn test_lda_inx_wrap() {
    let mut state = ComputerState::new();
    state.set_x(0x6B);
    state.set_up_state(&vec![
        opcode_from_operation(lda_inx),
        0xFF
    ]);
    // Pointer
    state.mem.set_nibble_at_addr(0x6A, 0x4D3C);
    // Value
    state.mem.set_byte_at_addr(0x4D3C, 0x33);
    state.execute_next();

    assert_eq!(0x33, state.acc.get());
    assert!(state.sta.is_empty())
}

#[test]
fn test_lda_inx_zero_flag() {
    let mut state = ComputerState::new();
    state.set_x(0x4);
    state.set_up_state(&vec![
        opcode_from_operation(lda_inx),
        0x55
    ]);
    // Pointer
    state.mem.set_nibble_at_addr(0x59, 0x4D3C);
    // Value
    state.mem.set_byte_at_addr(0x4D3C, 0);
    state.execute_next();

    assert_eq!(0, state.acc.get());
    assert!(state.sta.contains(StatusRegister::Z))
}

#[test]
fn test_lda_inx_negative_flag() {
    let mut state = ComputerState::new();
    state.set_x(0x4);
    state.set_up_state(&vec![
        opcode_from_operation(lda_inx),
        0x55
    ]);
    // Pointer
    state.mem.set_nibble_at_addr(0x59, 0x4D3C);
    // Value
    state.mem.set_byte_at_addr(0x4D3C, 0xE3);
    state.execute_next();

    assert_eq!(0xE3, state.acc.get());
    assert!(state.sta.contains(StatusRegister::N))
}

#[test]
fn test_lda_iny() {
    let mut state = ComputerState::new();
    state.set_y(0x1A);
    state.set_up_state(&vec![
        opcode_from_operation(lda_iny),
        0x55
    ]);
    // Pointer
    state.mem.set_nibble_at_addr(0x55, 0x3412);
    // Value
    state.mem.set_byte_at_addr(0x342C, 0x33);
    state.execute_next();

    assert_eq!(0x33, state.acc.get());
    assert!(state.sta.is_empty())
}

#[test]
fn test_lda_iny_zero_flag() {
    let mut state = ComputerState::new();
    state.set_y(0x1A);
    state.set_up_state(&vec![
        opcode_from_operation(lda_iny),
        0x55
    ]);
    // Pointer
    state.mem.set_nibble_at_addr(0x55, 0x3412);
    // Value
    state.mem.set_byte_at_addr(0x342C, 0);
    state.execute_next();

    assert_eq!(0, state.acc.get());
    assert!(state.sta.contains(StatusRegister::Z))
}

#[test]
fn test_lda_iny_negative_flag() {
    let mut state = ComputerState::new();
    state.set_y(0x1A);
    state.set_up_state(&vec![
        opcode_from_operation(lda_iny),
        0x55
    ]);
    // Pointer
    state.mem.set_nibble_at_addr(0x55, 0x3412);
    // Value
    state.mem.set_byte_at_addr(0x342C, 0xF3);
    state.execute_next();

    assert_eq!(0xF3, state.acc.get());
    assert!(state.sta.contains(StatusRegister::N))
}

#[test]
fn test_ldx_imm() {
    let mut state = ComputerState::new();
    state.set_up_state(&vec![
        opcode_from_operation(ldx_im),
        36
    ]);
    state.execute_next();

    assert_eq!(36, state.get_x());
    assert!(state.sta.is_empty());
}

#[test]
fn test_ldx_imm_zero_flag() {
    let mut state = ComputerState::new();
    state.set_up_state(&vec![
        opcode_from_operation(ldx_im),
        0
    ]);
    state.execute_next();

    assert_eq!(0, state.get_x());
    assert!(state.sta.contains(StatusRegister::Z));
}

#[test]
fn test_ldx_imm_negative_flag() {
    let mut state = ComputerState::new();
    state.set_up_state(&vec![
        opcode_from_operation(ldx_im),
        0xFF
    ]);
    state.execute_next();

    assert_eq!(0xFF, state.get_x());
    assert!(state.sta.contains(StatusRegister::N));
}

#[test]
fn test_ldx_zp() {
    let mut state = ComputerState::new();
    state.set_up_state(&vec![
        opcode_from_operation(ldx_zp),
        123
    ]);
    state.mem.set_byte_at_addr(123, 78);
    state.execute_next();

    assert_eq!(78, state.get_x());
    assert!(state.sta.is_empty());
}

#[test]
fn test_ldx_zp_zero_flag() {
    let mut state = ComputerState::new();
    state.set_up_state(&vec![
        opcode_from_operation(ldx_zp),
        123
    ]);
    state.mem.set_byte_at_addr(123, 0);
    state.execute_next();

    assert_eq!(0, state.get_x());
    assert!(state.sta.contains(StatusRegister::Z));
}

#[test]
fn test_ldx_zp_negative_flag() {
    let mut state = ComputerState::new();
    state.set_up_state(&vec![
        opcode_from_operation(ldx_zp),
        123
    ]);
    state.mem.set_byte_at_addr(123, 150);
    state.execute_next();

    assert_eq!(150, state.get_x());
    assert!(state.sta.contains(StatusRegister::N));
}

#[test]
fn test_ldx_zpy() {
    let mut state = ComputerState::new();
    state.set_y(0x15);
    state.set_up_state(&vec![
        opcode_from_operation(ldx_zpy),
        0x10
    ]);
    state.mem.set_byte_at_addr(0x25, 78);
    state.execute_next();

    assert_eq!(78, state.get_x());
    assert!(state.sta.is_empty());
}

#[test]
fn test_ldx_zpy_wrap() {
    let mut state = ComputerState::new();
    state.set_y(0x80);
    state.set_up_state(&vec![
        opcode_from_operation(ldx_zpy),
        0xFF
    ]);
    state.mem.set_byte_at_addr(0x7F, 78);
    state.execute_next();

    assert_eq!(78, state.get_x());
    assert!(state.sta.is_empty());
}

#[test]
fn test_ldx_zpy_zero_flag() {
    let mut state = ComputerState::new();
    state.set_y(0x15);
    state.set_up_state(&vec![
        opcode_from_operation(ldx_zpy),
        0x10
    ]);
    state.mem.set_byte_at_addr(0x25, 0);
    state.execute_next();

    assert_eq!(0, state.get_x());
    assert!(state.sta.contains(StatusRegister::Z));
}

#[test]
fn test_ldx_zpy_negative_flag() {
    let mut state = ComputerState::new();
    state.set_y(0x15);
    state.set_up_state(&vec![
        opcode_from_operation(ldx_zpy),
        0x10
    ]);
    state.mem.set_byte_at_addr(0x25, 0xF5);
    state.execute_next();

    assert_eq!(0xF5, state.get_x());
    assert!(state.sta.contains(StatusRegister::N));
}

#[test]
fn test_ldx_ab() {
    let mut state = ComputerState::new();
    state.set_up_state(&vec![
        opcode_from_operation(ldx_ab),
        0x55,
        0x66
    ]);
    state.mem.set_byte_at_addr(0x6655, 0x10);
    state.execute_next();

    assert_eq!(0x10, state.get_x());
    assert!(state.sta.is_empty())
}

#[test]
fn test_ldx_ab_zero_flag() {
    let mut state = ComputerState::new();
    state.set_up_state(&vec![
        opcode_from_operation(ldx_ab),
        0x55,
        0x66
    ]);
    state.mem.set_byte_at_addr(0x6655, 0);
    state.execute_next();

    assert_eq!(0, state.get_x());
    assert!(state.sta.contains(StatusRegister::Z));
}

#[test]
fn test_ldx_ab_negative_flag() {
    let mut state = ComputerState::new();
    state.set_up_state(&vec![
        opcode_from_operation(ldx_ab),
        0x55,
        0x66
    ]);
    state.mem.set_byte_at_addr(0x6655, 0xF6);
    state.execute_next();

    assert_eq!(0xF6, state.get_x());
    assert!(state.sta.contains(StatusRegister::N));
}

#[test]
fn test_ldx_aby() {
    let mut state = ComputerState::new();
    state.set_y(0x4);
    state.set_up_state(&vec![
        opcode_from_operation(ldx_aby),
        0x55,
        0x66
    ]);
    state.mem.set_byte_at_addr(0x6659, 0x10);
    state.execute_next();

    assert_eq!(0x10, state.get_x());
    assert!(state.sta.is_empty())
}

#[test]
fn test_ldx_aby_zero_flag() {
    let mut state = ComputerState::new();
    state.set_y(0x4);
    state.set_up_state(&vec![
        opcode_from_operation(ldx_aby),
        0x55,
        0x66
    ]);
    state.mem.set_byte_at_addr(0x6659, 0);
    state.execute_next();

    assert_eq!(0, state.get_x());
    assert!(state.sta.contains(StatusRegister::Z));
}

#[test]
fn test_ldx_aby_negative_flag() {
    let mut state = ComputerState::new();
    state.set_y(0x4);
    state.set_up_state(&vec![
        opcode_from_operation(ldx_aby),
        0x55,
        0x66
    ]);
    state.mem.set_byte_at_addr(0x6659, 0xF6);
    state.execute_next();

    assert_eq!(0xF6, state.get_x());
    assert!(state.sta.contains(StatusRegister::N));
}

#[test]
fn test_ldy_im() {
    let mut state = ComputerState::new();
    state.set_up_state(&vec![
        opcode_from_operation(ldy_im),
        36
    ]);
    state.execute_next();

    assert_eq!(36, state.get_y());
    assert!(state.sta.is_empty())
}

#[test]
fn test_ldy_im_zero_flag() {
    let mut state = ComputerState::new();
    state.set_up_state(&vec![
        opcode_from_operation(ldy_im),
        0
    ]);
    state.execute_next();

    assert_eq!(0, state.get_y());
    assert!(state.sta.contains(StatusRegister::Z));
}

#[test]
fn test_ldy_im_negative_flag() {
    let mut state = ComputerState::new();
    state.set_up_state(&vec![
        opcode_from_operation(ldy_im),
        0xFF
    ]);
    state.execute_next();

    assert_eq!(0xFF, state.get_y());
    assert!(state.sta.contains(StatusRegister::N));
}

#[test]
fn test_ldy_zp() {
    let mut state = ComputerState::new();
    state.set_up_state(&vec![
        opcode_from_operation(ldy_zp),
        123
    ]);
    state.mem.set_byte_at_addr(123, 78);
    state.execute_next();

    assert_eq!(78, state.get_y());
    assert!(state.sta.is_empty())
}

#[test]
fn test_ldy_zp_zero_flag() {
    let mut state = ComputerState::new();
    state.set_up_state(&vec![
        opcode_from_operation(ldy_zp),
        123
    ]);
    state.mem.set_byte_at_addr(123, 0);
    state.execute_next();

    assert_eq!(0, state.get_y());
    assert!(state.sta.contains(StatusRegister::Z));
}

#[test]
fn test_ldy_zp_negative_flag() {
    let mut state = ComputerState::new();
    state.set_up_state(&vec![
        opcode_from_operation(ldy_zp),
        123
    ]);
    state.mem.set_byte_at_addr(123, 0xFF);
    state.execute_next();

    assert_eq!(0xFF, state.get_y());
    assert!(state.sta.contains(StatusRegister::N));
}

#[test]
fn test_ldy_zpx() {
    let mut state = ComputerState::new();
    state.set_x(0x15);
    state.set_up_state(&vec![
        opcode_from_operation(ldy_zpx),
        0x10
    ]);
    state.mem.set_byte_at_addr(0x25, 78);
    state.execute_next();

    assert_eq!(78, state.get_y());
    assert!(state.sta.is_empty())
}

#[test]
fn test_ldy_zpx_wrap() {
    let mut state = ComputerState::new();
    state.set_x(0x80);
    state.set_up_state(&vec![
        opcode_from_operation(ldy_zpx),
        0xFF
    ]);
    state.mem.set_byte_at_addr(0x7F, 78);
    state.execute_next();

    assert_eq!(78, state.get_y());
    assert!(state.sta.is_empty())
}

#[test]
fn test_ldy_zpx_zero_flag() {
    let mut state = ComputerState::new();
    state.set_x(0x15);
    state.set_up_state(&vec![
        opcode_from_operation(ldy_zpx),
        0x10
    ]);
    state.mem.set_byte_at_addr(0x25, 0);
    state.execute_next();

    assert_eq!(0, state.get_y());
    assert!(state.sta.contains(StatusRegister::Z));
}

#[test]
fn test_ldy_zpx_negative_flag() {
    let mut state = ComputerState::new();
    state.set_x(0x15);
    state.set_up_state(&vec![
        opcode_from_operation(ldy_zpx),
        0x10
    ]);
    state.mem.set_byte_at_addr(0x25, 0xFF);
    state.execute_next();

    assert_eq!(0xFF, state.get_y());
    assert!(state.sta.contains(StatusRegister::N));
}

#[test]
fn test_ldy_ab() {
    let mut state = ComputerState::new();
    state.set_up_state(&vec![
        opcode_from_operation(ldy_ab),
        0x55,
        0x66
    ]);
    state.mem.set_byte_at_addr(0x6655, 0x10);
    state.execute_next();

    assert_eq!(0x10, state.get_y());
    assert!(state.sta.is_empty())
}

#[test]
fn test_ldy_ab_zero_flag() {
    let mut state = ComputerState::new();
    state.set_up_state(&vec![
        opcode_from_operation(ldy_ab),
        0x55,
        0x66
    ]);
    state.mem.set_byte_at_addr(0x6655, 0);
    state.execute_next();

    assert_eq!(0, state.get_y());
    assert!(state.sta.contains(StatusRegister::Z));
}

#[test]
fn test_ldy_ab_negative_flag() {
    let mut state = ComputerState::new();
    state.set_up_state(&vec![
        opcode_from_operation(ldy_ab),
        0x55,
        0x66
    ]);
    state.mem.set_byte_at_addr(0x6655, 0xF6);
    state.execute_next();

    assert_eq!(0xF6, state.get_y());
    assert!(state.sta.contains(StatusRegister::N));
}

#[test]
fn test_ldy_abx() {
    let mut state = ComputerState::new();
    state.set_x(0x4);
    state.set_up_state(&vec![
        opcode_from_operation(ldy_abx),
        0x55,
        0x66
    ]);
    state.mem.set_byte_at_addr(0x6659, 0x10);
    state.execute_next();

    assert_eq!(0x10, state.get_y());
    assert!(state.sta.is_empty())
}

#[test]
fn test_ldy_abx_zero_flag() {
    let mut state = ComputerState::new();
    state.set_x(0x4);
    state.set_up_state(&vec![
        opcode_from_operation(ldy_abx),
        0x55,
        0x66
    ]);
    state.mem.set_byte_at_addr(0x6659, 0);
    state.execute_next();

    assert_eq!(0, state.get_y());
    assert!(state.sta.contains(StatusRegister::Z));
}

#[test]
fn test_ldy_abx_negative_flag() {
    let mut state = ComputerState::new();
    state.set_x(0x4);
    state.set_up_state(&vec![
        opcode_from_operation(ldy_abx),
        0x55,
        0x66
    ]);
    state.mem.set_byte_at_addr(0x6659, 0xF6);
    state.execute_next();

    assert_eq!(0xF6, state.get_y());
    assert!(state.sta.contains(StatusRegister::N));
}


#[test]
fn test_sta_zp() {
    let mut state = ComputerState::new();
    state.acc.set(0x9D);
    state.set_up_state(&vec![
        opcode_from_operation(sta_zp),
        0xE5
    ]);
    state.execute_next();

    assert_eq!(0x9D, state.mem.fetch_byte_from_addr(0xE5));
    assert!(state.sta.is_empty());
}

#[test]
fn test_sta_zp_flags_unaffected() {
    let mut state = ComputerState::new();
    state.acc.set(0x9D);
    // Every flag set
    state.sta |= StatusRegister::V |
                 StatusRegister::Z  |
                 StatusRegister::N  |
                 StatusRegister::C  |
                 StatusRegister::B  |
                 StatusRegister::D  |
                 StatusRegister::I;
    state.set_up_state(&vec![
        opcode_from_operation(sta_zp),
        0xE5
    ]);
    state.execute_next();

    assert_eq!(0x9D, state.mem.fetch_byte_from_addr(0xE5));
    assert!(state.sta.contains_only(
        StatusRegister::V |
            StatusRegister::Z  |
            StatusRegister::N  |
            StatusRegister::C  |
            StatusRegister::B  |
            StatusRegister::D  |
            StatusRegister::I
    ));
}

#[test]
fn test_sta_zpx() {
    let mut state = ComputerState::new();
    state.acc.set(0x4F);
    state.set_x(0x15);
    state.set_up_state(&vec![
        opcode_from_operation(sta_zpx),
        0x10
    ]);
    state.execute_next();

    assert_eq!(0x4F, state.mem.fetch_byte_from_addr(0x25));
    assert!(state.sta.is_empty());
}

#[test]
fn test_sta_ab() {
    let mut state = ComputerState::new();
    state.acc.set(0x10);
    state.set_up_state(&vec![
        opcode_from_operation(sta_ab),
        0x55,
        0x66
    ]);
    state.execute_next();

    assert_eq!(0x10, state.mem.fetch_byte_from_addr(0x6655));
    assert!(state.sta.is_empty());
}

#[test]
fn test_sta_abx() {
    let mut state = ComputerState::new();
    state.acc.set(0x10);
    state.set_x(0x4);
    state.set_up_state(&vec![
        opcode_from_operation(sta_abx),
        0x55,
        0x66
    ]);
    state.execute_next();

    assert_eq!(0x10, state.mem.fetch_byte_from_addr(0x6659));
    assert!(state.sta.is_empty());
}

#[test]
fn test_sta_aby() {
    let mut state = ComputerState::new();
    state.acc.set(0x10);
    state.set_y(0x4);
    state.set_up_state(&vec![
        opcode_from_operation(sta_aby),
        0x55,
        0x66
    ]);
    state.execute_next();

    assert_eq!(0x10, state.mem.fetch_byte_from_addr(0x6659));
    assert!(state.sta.is_empty());
}

#[test]
fn test_sta_inx() {
    let mut state = ComputerState::new();
    state.acc.set(0x33);
    state.set_x(0x4);
    state.set_up_state(&vec![
        opcode_from_operation(sta_inx),
        0x55
    ]);
    state.mem.set_nibble_at_addr(0x59, 0x4D3C);
    state.execute_next();

    assert_eq!(0x33, state.mem.fetch_byte_from_addr(0x4D3C));
    assert!(state.sta.is_empty());
}

#[test]
fn test_sta_inx_wrap() {
    let mut state = ComputerState::new();
    state.acc.set(0x33);
    state.set_x(0x6B);
    state.set_up_state(&vec![
        opcode_from_operation(sta_inx),
        0xFF
    ]);
    state.mem.set_nibble_at_addr(0x6A, 0x4D3C);
    state.execute_next();

    assert_eq!(0x33, state.mem.fetch_byte_from_addr(0x4D3C));
    assert!(state.sta.is_empty());
}

#[test]
fn test_sta_iny() {
    let mut state = ComputerState::new();
    state.acc.set(0x33);
    state.set_y(0x1A);
    state.set_up_state(&vec![
        opcode_from_operation(sta_iny),
        0x55
    ]);
    state.mem.set_nibble_at_addr(0x55, 0x3412);
    state.execute_next();

    assert_eq!(0x33, state.mem.fetch_byte_from_addr(0x342C));
    assert!(state.sta.is_empty());
}

#[test]
fn test_stx_zp() {
    let mut state = ComputerState::new();
    state.set_x(0x78);
    state.set_up_state(&vec![
        opcode_from_operation(stx_zp),
        123
    ]);
    state.execute_next();

    assert_eq!(0x78, state.mem.fetch_byte_from_addr(123));
    assert!(state.sta.is_empty());
}

#[test]
fn test_stx_zpy() {
    let mut state = ComputerState::new();
    state.set_x(0x78);
    state.set_y(0x15);
    state.set_up_state(&vec![
        opcode_from_operation(stx_zpy),
        0x10
    ]);
    state.execute_next();

    assert_eq!(0x78, state.mem.fetch_byte_from_addr(0x25));
    assert!(state.sta.is_empty());
}

#[test]
fn test_stx_zpy_wrap() {
    let mut state = ComputerState::new();
    state.set_x(0x78);
    state.set_y(0x80);
    state.set_up_state(&vec![
        opcode_from_operation(stx_zpy),
        0xFF
    ]);
    state.execute_next();

    assert_eq!(0x78, state.mem.fetch_byte_from_addr(0x7F));
    assert!(state.sta.is_empty());
}

#[test]
fn test_stx_ab() {
    let mut state = ComputerState::new();
    state.set_x(0x78);
    state.set_up_state(&vec![
        opcode_from_operation(stx_ab),
        0x55,
        0x66
    ]);
    state.execute_next();

    assert_eq!(0x78, state.mem.fetch_byte_from_addr(0x6655));
    assert!(state.sta.is_empty());
}

#[test]
fn test_sty_zp() {
    let mut state = ComputerState::new();
    state.set_y(0x78);
    state.set_up_state(&vec![
        opcode_from_operation(sty_zp),
        123
    ]);
    state.execute_next();

    assert_eq!(0x78, state.mem.fetch_byte_from_addr(123));
    assert!(state.sta.is_empty());
}

#[test]
fn test_sty_zpx() {
    let mut state = ComputerState::new();
    state.set_y(0x78);
    state.set_x(0x15);
    state.set_up_state(&vec![
        opcode_from_operation(sty_zpx),
        0x10
    ]);
    state.execute_next();

    assert_eq!(0x78, state.mem.fetch_byte_from_addr(0x25));
    assert!(state.sta.is_empty());
}

#[test]
fn test_sty_zpx_wrap() {
    let mut state = ComputerState::new();
    state.set_y(0x78);
    state.set_x(0x80);
    state.set_up_state(&vec![
        opcode_from_operation(sty_zpx),
        0xFF
    ]);
    state.execute_next();

    assert_eq!(0x78, state.mem.fetch_byte_from_addr(0x7F));
    assert!(state.sta.is_empty());
}

#[test]
fn test_sty_ab() {
    let mut state = ComputerState::new();
    state.set_y(0x78);
    state.set_up_state(&vec![
        opcode_from_operation(sty_ab),
        0x55,
        0x66
    ]);
    state.execute_next();

    assert_eq!(0x78, state.mem.fetch_byte_from_addr(0x6655));
    assert!(state.sta.is_empty());
}
