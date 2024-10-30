use crate::computer_state::ComputerState;
use crate::computer_state::operations::arithmetic::*;
use crate::computer_state::operations::bitwise::*;
use crate::computer_state::operations::interrupt::*;

mod arithmetic;
mod interrupt;
mod branch;
mod bitwise;

/// Helper function that reinterprets an u8 to an i8
fn u8_to_i8(n: u8) -> i8 {
    (i16::from(n) - i16::from(i8::MAX)) as i8
}
/// Helper function that reinterprets an i8 to an u8
fn i8_to_u8(n: i8) -> u8 {
    (i16::from(n) + i16::from(i8::MAX)) as u8
}

#[cfg(test)]
mod tests {
    use crate::computer_state::operations::{i8_to_u8, u8_to_i8};

    #[test]
    fn test_u8_to_i8() {
        assert_eq!(i8_to_u8(u8_to_i8(64)), 64);
        assert_eq!(i8_to_u8(u8_to_i8(0)), 0);
    }
}

pub const INSTRUCTION_TABLE: [fn (&mut ComputerState); 16 * 16] = [
 //      0         2         4         6         8         A         C         E
 //           1         3         5         7         9         B         D         F
 /* 0 */ brk,      nop,      nop,      asl_zp,   nop,      asl_acc,  nop,      asl_ab,
              or_inx,   nop,      or_zp,    nop,      or_im,    nop,      or_ab,    nop,
 /* 1 */ brk,      nop,      nop,      asl_zpx,  nop,      nop,      nop,      asl_abx,
              or_iny,   nop,      or_zpx,   nop,      or_aby,   nop,      or_abx,   nop,
 /* 2 */ brk,      nop,      nop,      nop,      nop,      nop,      bit_ab,   nop,
              and_inx,  nop,      and_zp,   nop,      and_im,   nop,      and_ab,   nop,
 /* 3 */ brk,      nop,      bit_zp,   nop,      nop,      nop,      nop,      nop,
              and_iny,  nop,      and_zpx,  nop,      and_aby,  nop,      and_abx,  nop,
 /* 4 */ brk,      nop,      nop,      lsr_zp,   nop,      lsr_acc,  nop,      lsr_ab,
              eor_inx,  nop,      eor_zp,   nop,      eor_im,   nop,      eor_ab,   nop,
 /* 5 */ brk,      nop,      nop,      lsr_zpx,  nop,      nop,      nop,      lsr_abx,
              eor_iny,  nop,      eor_zpx,  nop,      eor_aby,  nop,      eor_abx,  nop,
 /* 6 */ brk,      nop,      nop,      nop,      nop,      nop,      nop,      nop,
              add_inx,  nop,      add_zp,   nop,      add_im,   nop,      add_ab,   nop,
 /* 7 */ brk,      nop,      nop,      nop,      nop,      nop,      nop,      nop,
              add_iny,  nop,      add_zpx,  nop,      add_aby,  nop,      add_abx,  nop,
 /* 8 */ brk,      nop,      nop,      nop,      nop,      nop,      nop,      nop,
              nop,      nop,      nop,      nop,      nop,      nop,      nop,      nop,
 /* 9 */ brk,      nop,      nop,      nop,      nop,      nop,      nop,      nop,
              nop,      nop,      nop,      nop,      nop,      nop,      nop,      nop,
 /* A */ brk,      nop,      nop,      nop,      nop,      nop,      nop,      nop,
              nop,      nop,      nop,      nop,      nop,      nop,      nop,      nop,
 /* B */ brk,      nop,      nop,      nop,      nop,      nop,      nop,      nop,
              nop,      nop,      nop,      nop,      nop,      nop,      nop,      nop,
 /* C */ brk,      nop,      nop,      nop,      nop,      nop,      nop,      nop,
              nop,      nop,      nop,      nop,      nop,      nop,      nop,      nop,
 /* D */ brk,      nop,      nop,      nop,      nop,      nop,      nop,      nop,
              nop,      nop,      nop,      nop,      nop,      nop,      nop,      nop,
 /* E */ brk,      nop,      nop,      nop,      nop,      nop,      nop,      nop,
              sub_inx,  nop,      sub_zp,   nop,      sub_im,   nop,      sub_ab,   nop,
 /* F */ brk,      nop,      nop,      nop,      nop,      nop,      nop,      nop,
              sub_iny,  nop,      sub_zpx,  nop,      sub_aby,  nop,      sub_abx,  nop,
];