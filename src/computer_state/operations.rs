use crate::computer_state::ComputerState;
use crate::computer_state::operations::interrupt::*;

mod add;
mod interrupt;

pub const INSTRUCTION_TABLE: [fn (&mut ComputerState); 16 * 16] = [
    brk, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop,
    nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop,
    nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop,
    nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop,
    nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop,
    nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop,
    nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop,
    nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop,
    nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop,
    nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop,
    nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop,
    nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop,
    nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop,
    nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop,
    nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop,
    nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop,
];