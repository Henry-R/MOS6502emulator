use super::ComputerState;
use arithmetic::*;
use bitwise::*;
use flags::*;
use interrupt::*;

mod signed_arithmetic;

mod arithmetic;
mod interrupt;
mod branch;
mod bitwise;
mod flags;

pub const INSTRUCTION_TABLE: [fn (&mut ComputerState); 256] = [
 //      0         2         4         6         8         A         C         E
 //           1         3         5         7         9         B         D         F
 /* 0 */ brk,      nop,      nop,      asl_zp,   nop,      asl_acc,  nop,      asl_ab,
              or_inx,   nop,      or_zp,    nop,      or_im,    nop,      or_ab,    nop,
 /* 1 */ brk,      nop,      nop,      asl_zpx,  clr,      nop,      nop,      asl_abx,
              or_iny,   nop,      or_zpx,   nop,      or_aby,   nop,      or_abx,   nop,
 /* 2 */ brk,      nop,      nop,      nop,      nop,      nop,      bit_ab,   nop,
              and_inx,  nop,      and_zp,   nop,      and_im,   nop,      and_ab,   nop,
 /* 3 */ brk,      nop,      bit_zp,   nop,      sec,      nop,      nop,      nop,
              and_iny,  nop,      and_zpx,  nop,      and_aby,  nop,      and_abx,  nop,
 /* 4 */ brk,      nop,      nop,      lsr_zp,   nop,      lsr_acc,  nop,      lsr_ab,
              eor_inx,  nop,      eor_zp,   nop,      eor_im,   nop,      eor_ab,   nop,
 /* 5 */ brk,      nop,      nop,      lsr_zpx,  cli,      nop,      nop,      lsr_abx,
              eor_iny,  nop,      eor_zpx,  nop,      eor_aby,  nop,      eor_abx,  nop,
 /* 6 */ brk,      nop,      nop,      nop,      nop,      nop,      nop,      nop,
              add_inx,  nop,      add_zp,   nop,      add_im,   nop,      add_ab,   nop,
 /* 7 */ brk,      nop,      nop,      nop,      sei,      nop,      nop,      nop,
              add_iny,  nop,      add_zpx,  nop,      add_aby,  nop,      add_abx,  nop,
 /* 8 */ brk,      nop,      nop,      nop,      dey,      nop,      nop,      nop,
              nop,      nop,      nop,      nop,      nop,      nop,      nop,      nop,
 /* 9 */ brk,      nop,      nop,      nop,      nop,      nop,      nop,      nop,
              nop,      nop,      nop,      nop,      nop,      nop,      nop,      nop,
 /* A */ brk,      nop,      nop,      nop,      nop,      nop,      nop,      nop,
              nop,      nop,      nop,      nop,      nop,      nop,      nop,      nop,
 /* B */ brk,      nop,      nop,      nop,      clo,      nop,      nop,      nop,
              nop,      nop,      nop,      nop,      nop,      nop,      nop,      nop,
 /* C */ brk,      nop,      nop,      dec_zp,   iny,      dex,      nop,      dec_ab,
              nop,      nop,      nop,      nop,      nop,      nop,      nop,      nop,
 /* D */ brk,      nop,      nop,      dec_zpx,  cld,      nop,      nop,      dec_abx,
              nop,      nop,      nop,      nop,      nop,      nop,      nop,      nop,
 /* E */ brk,      nop,      nop,      inc_zp,   inx,      nop,      nop,      inc_ab,
              sub_inx,  nop,      sub_zp,   nop,      sub_im,   nop,      sub_ab,   nop,
 /* F */ brk,      nop,      nop,      inc_zpx,  sed,      nop,      nop,      inc_abx,
              sub_iny,  nop,      sub_zpx,  nop,      sub_aby,  nop,      sub_abx,  nop,
];