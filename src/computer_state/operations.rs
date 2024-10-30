use crate::computer_state::ComputerState;
use crate::computer_state::operations::arithmetic::*;
use crate::computer_state::operations::bitwise::*;
use crate::computer_state::operations::interrupt::*;

mod arithmetic;
mod interrupt;
mod branch;
mod bitwise;

/// Helper function that converts an u8 byte to the i8 represented by its bits
const fn u8_to_i8(n: u8) -> i8 {
    (n as i16 - i8::MAX as i16) as i8
}

pub const INSTRUCTION_TABLE: [fn (&mut ComputerState); 16 * 16] = [
 //      0         2         4         6         8         A         C         E
 //           1         3         5         7         9         B         D         F
 /* 0 */ brk,      nop,      nop,      nop,      nop,      nop,      nop,      nop,
              nop,      nop,      nop,      nop,      nop,      nop,      nop,      nop,
 /* 1 */ brk,      nop,      nop,      nop,      nop,      nop,      nop,      nop,
              nop,      nop,      nop,      nop,      nop,      nop,      nop,      nop,
 /* 2 */ brk,      nop,      nop,      nop,      nop,      nop,      nop,      nop,
              and_inx,  nop,      and_zp,   nop,      and_im,   nop,      and_ab,   nop,
 /* 3 */ brk,      nop,      nop,      nop,      nop,      nop,      nop,      nop,
              and_iny,  nop,      and_zpx,  nop,      and_aby,  nop,      and_abx,  nop,
 /* 4 */ brk,      nop,      nop,      nop,      nop,      nop,      nop,      nop,
              nop,      nop,      nop,      nop,      nop,      nop,      nop,      nop,
 /* 5 */ brk,      nop,      nop,      nop,      nop,      nop,      nop,      nop,
              nop,      nop,      nop,      nop,      nop,      nop,      nop,      nop,
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