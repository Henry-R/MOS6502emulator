use crate::{ComputerState, StatusFlags, INTERRUPT_REQUEST_HANDLER};

// MOS6502 operation. Mutates the computer's state
type MosOp = fn(&mut ComputerState);

/// BRK (Force Break)
fn force_break(state: &mut ComputerState) {
    // Set break flag
    state.regs.sta |= StatusFlags::b;
    // Push return address and status onto stack
    state.stk_push_pc(state.regs.pc + 2);
    state.stk_push_frame((state.regs.sta.bits(), 0));
    // Move PC to interrupt on request handler
    state.regs.pc = INTERRUPT_REQUEST_HANDLER;
}

const _INSTRUCTION_TABLE: [[MosOp; 1]; 1] = [
    [
        force_break
    ]
];