use super::ComputerState;
use arithmetic::*;
use bitwise::*;
use flags::*;
use interrupt::*;

pub(crate) mod arithmetic;
mod interrupt;
mod branch;
pub(crate) mod bitwise;
mod flags;

pub type MosOp = fn (&mut ComputerState);

const INSTRUCTION_TABLE: [MosOp; 256] = [
 //      0         2         4         6         8         A         C         E
 //           1         3         5         7         9         B         D         F
 /* 0 */ brk,      nop,      nop,      asl_zp,   nop,      asl_acc,  nop,      asl_ab,
              or_inx,   nop,      or_zp,    nop,      or_im,    nop,      or_ab,    nop,
 /* 1 */ brk,      nop,      nop,      asl_zpx,  clr,      nop,      nop,      asl_abx,
              or_iny,   nop,      or_zpx,   nop,      or_aby,   nop,      or_abx,   nop,
 /* 2 */ brk,      nop,      nop,      rol_zp,   nop,      rol_acc,  bit_ab,   rol_ab,
              and_inx,  nop,      and_zp,   nop,      and_im,   nop,      and_ab,   nop,
 /* 3 */ brk,      nop,      bit_zp,   rol_zpx,  sec,      nop,      nop,      rol_abx,
              and_iny,  nop,      and_zpx,  nop,      and_aby,  nop,      and_abx,  nop,
 /* 4 */ brk,      nop,      nop,      lsr_zp,   nop,      lsr_acc,  nop,      lsr_ab,
              eor_inx,  nop,      eor_zp,   nop,      eor_im,   nop,      eor_ab,   nop,
 /* 5 */ brk,      nop,      nop,      lsr_zpx,  cli,      nop,      nop,      lsr_abx,
              eor_iny,  nop,      eor_zpx,  nop,      eor_aby,  nop,      eor_abx,  nop,
 /* 6 */ brk,      nop,      nop,      nop,      nop,      nop,      nop,      nop,
              adc_inx,  nop,      adc_zp,   nop,      adc_im,   nop,      adc_ab,   nop,
 /* 7 */ brk,      nop,      nop,      nop,      sei,      nop,      nop,      nop,
              adc_iny,  nop,      adc_zpx,  nop,      adc_aby,  nop,      adc_abx,  nop,
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
              sbc_inx,  nop,      sbc_zp,   nop,      sbc_im,   nop,      sbc_ab,   nop,
 /* F */ brk,      nop,      nop,      inc_zpx,  sed,      nop,      nop,      inc_abx,
         sbc_iny,       nop,      sbc_zpx,  nop,      sbc_aby,  nop,      sbc_abx,  nop,
];

pub const fn decode(opcode: u8) -> MosOp {
    INSTRUCTION_TABLE[opcode as usize]
}

pub fn opcode_from_operation(op: MosOp) -> u8 {
    let op_index = INSTRUCTION_TABLE.iter().position(|&f| f == op);
    // If given a correct function, this will always give a result,
    // and the index will fit inside an u8
    op_index.unwrap() as u8
}
