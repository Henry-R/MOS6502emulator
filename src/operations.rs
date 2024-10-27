use crate::{ComputerState, StatusFlags, INTERRUPT_REQUEST_HANDLER};

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

fn nop(state: &mut ComputerState) {

}

const INSTRUCTION_TABLE: [fn (&mut ComputerState); 16 * 16] = [
        force_break, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop, nop,
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