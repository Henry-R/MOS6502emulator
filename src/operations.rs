use crate::ComputerState;

// MOS6502 operation. Mutates the computer's state
type MosOp = fn(&mut ComputerState);

/// BRK (Force Break)
///
/// BRK initiates a software interrupt similar to a hardware
/// interrupt (IRQ). The return address pushed to the stack is
/// PC+2, providing an extra byte of spacing for a break mark
/// (identifying a reason for the break.)
/// The status register will be pushed to the stack with the break
/// flag set to 1. However, when retrieved during RTI or by a PLP
/// instruction, the break flag will be ignored.
/// The interrupt disable flag is not set automatically.
fn force_break(state: &mut ComputerState) {
    // state.stk = state.pc + 2; RESEARCH TYPE OF STACK
    state.sta.b = true;
}

const INSTRUCTION_TABLE: [[MosOp; 1]; 1] = [
    [
        force_break
    ]
];