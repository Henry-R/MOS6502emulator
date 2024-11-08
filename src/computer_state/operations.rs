use super::ComputerState;

use arithmetic::*;
use bitwise::*;
use flags::*;
use interrupt::*;
use branch::*;
use load_store::*;
use register_transfers::*;

pub mod arithmetic;
pub mod interrupt;
pub mod branch;
pub mod bitwise;
pub mod load_store;
pub mod flags;
pub mod register_transfers;

macro_rules! add_op {
    ($fn_ptr:expr, $address:expr) => {{
        ($fn_ptr, stringify!($fn_ptr), $address)
    }};
}

pub type MosOp = fn (&mut ComputerState);

const INSTRUCTION_COUNT: usize = 256;
const INSTRUCTION_LIST: [(MosOp, &str, usize); 146] = [
    // LOAD / STORE OPERATIONS
    // load accumulator
    add_op!(lda_im, 0xA9),
    add_op!(lda_zp, 0xA5),
    add_op!(lda_zpx, 0xB5),
    add_op!(lda_ab, 0xAD),
    add_op!(lda_abx, 0xBD),
    add_op!(lda_aby, 0xB9),
    add_op!(lda_inx, 0xA1),
    add_op!(lda_iny, 0xB1),

    // load x register
    add_op!(ldx_im, 0xA2),
    add_op!(ldx_zp, 0xA6),
    add_op!(ldx_zpy, 0xB6),
    add_op!(ldx_ab, 0xAE),
    add_op!(ldx_aby, 0xBE),

    // load y register
    add_op!(ldy_im, 0xA0),
    add_op!(ldy_zp, 0xA4),
    add_op!(ldy_zpy, 0xB4),
    add_op!(ldy_ab, 0xAC),
    add_op!(ldy_aby, 0xBC),

    // store accumulator
    add_op!(sta_zp, 0x85),
    add_op!(sta_zpx, 0x95),
    add_op!(sta_ab, 0x8D),
    add_op!(sta_abx, 0x9D),
    add_op!(sta_aby, 0x99),
    add_op!(sta_inx, 0x81),
    add_op!(sta_iny, 0x91),

    // store x register
    add_op!(stx_zp, 0x86),
    add_op!(stx_zpy, 0x96),
    add_op!(stx_ab, 0x8E),

    // store y register
    add_op!(sty_zp, 0x84),
    add_op!(sty_zpx, 0x94),
    add_op!(sty_ab, 0x8C),


    // REGISTER TRANSFERS
    // transfer accumulator to x register
    add_op!(tax, 0xAA),

    // transfer accumulator to y register
    add_op!(tay, 0xA8),

    // transfer x register to accumulator
    add_op!(txa, 0x8A),

    // transfer y register to accumulator
    add_op!(tya, 0x98),


    // STACK OPERATIONS
    // transfer stack pointer to x
    add_op!(nop, 0xBA),

    // transfer x to stack pointer
    add_op!(nop, 0x9A),

    // push accumulator onto stack
    add_op!(nop, 0x48),

    // push processor status onto stack
    add_op!(nop, 0x08),

    // pull accumulator from stack
    add_op!(nop, 0x68),

    // pull processor status from stack
    add_op!(nop, 0x28),


    // LOGICAL
    // logical and
    add_op!(and_im, 0x29),
    add_op!(and_zp, 0x25),
    add_op!(and_zpx, 0x35),
    add_op!(and_ab, 0x2D),
    add_op!(and_abx, 0x3D),
    add_op!(and_aby, 0x39),
    add_op!(and_inx, 0x21),
    add_op!(and_iny, 0x31),

    // exclusive or
    add_op!(eor_im, 0x49),
    add_op!(eor_zp, 0x45),
    add_op!(eor_zpx, 0x55),
    add_op!(eor_ab, 0x4D),
    add_op!(eor_abx, 0x5D),
    add_op!(eor_aby, 0x59),
    add_op!(eor_inx, 0x41),
    add_op!(eor_iny, 0x51),

    // logical inclusive or
    add_op!(or_im, 0x09),
    add_op!(or_zp, 0x05),
    add_op!(or_zpx, 0x15),
    add_op!(or_ab, 0x0D),
    add_op!(or_abx, 0x1D),
    add_op!(or_aby, 0x19),
    add_op!(or_inx, 0x01),
    add_op!(or_iny, 0x11),

    // bit test
    add_op!(bit_zp, 0x24),
    add_op!(bit_ab, 0x2C),


    // ARITHMETIC
    // add with carry
    add_op!(adc_im, 0x69),
    add_op!(adc_zp, 0x65),
    add_op!(adc_zpx, 0x75),
    add_op!(adc_ab, 0x6D),
    add_op!(adc_abx, 0x7D),
    add_op!(adc_aby, 0x79),
    add_op!(adc_inx, 0x61),
    add_op!(adc_iny, 0x71),

    // subtract with carry
    add_op!(sbc_im, 0xE9),
    add_op!(sbc_zp, 0xE5),
    add_op!(sbc_zpx, 0xF5),
    add_op!(sbc_ab, 0xED),
    add_op!(sbc_abx, 0xFD),
    add_op!(sbc_aby, 0xF9),
    add_op!(sbc_inx, 0xE1),
    add_op!(sbc_iny, 0xF1),

    // compare accumulator
    add_op!(nop, 0xC9),
    add_op!(nop, 0xC5),
    add_op!(nop, 0xD5),
    add_op!(nop, 0xCD),
    add_op!(nop, 0xDD),
    add_op!(nop, 0xD9),
    add_op!(nop, 0xC1),
    add_op!(nop, 0xD1),

    // compare x register
    add_op!(nop, 0xE0),
    add_op!(nop, 0xE4),
    add_op!(nop, 0xEC),

    // compare y register
    add_op!(nop, 0xC0),
    add_op!(nop, 0xC4),
    add_op!(nop, 0xCC),


    // INCREMENTS & DECREMENTS
    // increment a memory location
    add_op!(inc_zp, 0xE6),
    add_op!(inc_zpx, 0xF6),
    add_op!(inc_ab, 0xEE),
    add_op!(inc_abx, 0xFE),

    // increment x register
    add_op!(inx, 0xE8),

    // increment y register
    add_op!(iny, 0xC8),

    // decrement a memory location
    add_op!(dec_zp, 0xC6),
    add_op!(dec_zpx, 0xD6),
    add_op!(dec_ab, 0xCE),
    add_op!(dec_abx, 0xDE),

    // decrement x register
    add_op!(dex, 0xCA),

    // decrement y register
    add_op!(dey, 0x88),


    // SHIFTS
    // arithmetic shift left
    add_op!(asl_acc, 0x0A),
    add_op!(asl_zp, 0x06),
    add_op!(asl_zpx, 0x16),
    add_op!(asl_ab, 0x0E),
    add_op!(asl_abx, 0x1A),

    // logical shift right
    add_op!(lsr_acc, 0x4A),
    add_op!(lsr_zp, 0x46),
    add_op!(lsr_zpx, 0x56),
    add_op!(lsr_ab, 0x4E),
    add_op!(lsr_abx, 0x5A),

    // rotate left
    add_op!(rol_acc, 0x2A),
    add_op!(rol_zp, 0x26),
    add_op!(rol_zpx, 0x36),
    add_op!(rol_ab, 0x2E),
    add_op!(rol_abx, 0x3E),

    // rotate right
    add_op!(ror_acc, 0x6A),
    add_op!(ror_zp, 0x66),
    add_op!(ror_zpx, 0x76),
    add_op!(ror_ab, 0x6E),
    add_op!(ror_abx, 0x7E),


    // JUMPS & CALLS
    // jump to another location

    // jump to a subroutine

    // return from subroutine


    // BRANCHES
    // branch if carry flag clear
    add_op!(bcc, 0x90),

    // branch if carry flag set
    add_op!(bcs, 0xB0),

    // branch if zero flag set
    add_op!(beq, 0xF0),

    // branch if negative flag set
    add_op!(bmi, 0x30),

    // branch if zero flag clear
    add_op!(bne, 0xD0),

    // branch if negative flag clear
    add_op!(bpl, 0x10),

    // branch if overflow flag clear
    add_op!(bvs, 0x70),

    // branch if overflow flag set
    add_op!(bvc, 0x50),


    // STATUS FLAG CHANGES
    // clear carry flag
    add_op!(clr, 0x18),

    // clear decimal mode flag
    add_op!(cld, 0xD8),

    // clear interrupt disable flag
    add_op!(cli, 0x58),

    // clear overflow flag
    add_op!(clv, 0xB8),

    // set carry flag
    add_op!(sec, 0xB8),

    // set decimal mode flag
    add_op!(sed, 0xF8),

    // set interrupt disable flag
    add_op!(sei, 0x78),


    // SYSTEM FUNCTIONS
    // force an interrupt
    add_op!(brk, 0x00),

    // no operation
    add_op!(nop, 0xEA),

    // return from interrupt
];

const INSTRUCTION_DATA_TABLE: [(MosOp, &str); INSTRUCTION_COUNT] = {
    let mut tmp_data_table: [(MosOp, &str); INSTRUCTION_COUNT] = [(nop, "NOP"); INSTRUCTION_COUNT];
    let mut i = 0;

    while i < INSTRUCTION_LIST.len() {
        let func      = INSTRUCTION_LIST[i].0;
        let name      = INSTRUCTION_LIST[i].1;
        let opcode    = INSTRUCTION_LIST[i].2;
        tmp_data_table[opcode] = (func, name);
        i += 1;
    }

    tmp_data_table
};


pub const fn decode(opcode: u8) -> MosOp {
    INSTRUCTION_DATA_TABLE[opcode as usize].0
}

pub const fn decode_operation_name(opcode: u8) -> &'static str {
    INSTRUCTION_DATA_TABLE[opcode as usize].1
}

pub fn opcode_from_operation(op: MosOp) -> u8 {
    let op_index = INSTRUCTION_DATA_TABLE.iter().position(|&f| f.0 == op);
    // If given a correct function, this will always give a result,
    // and the index will fit inside an u8
    op_index.unwrap() as u8
}
