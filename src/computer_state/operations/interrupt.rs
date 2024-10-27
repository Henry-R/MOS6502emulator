
use crate::computer_state::{ComputerState, StatusFlags, INTERRUPT_REQUEST_HANDLER};

pub fn nop(_state: &mut ComputerState) {}

/// BRK (Force Break)
pub fn brk(state: &mut ComputerState) {
    // Set break flag
    state.regs.sta |= StatusFlags::b;
    // Push return address and status onto stack
    state.stk_push_pc(state.regs.pc + 2);
    state.stk_push_frame((state.regs.sta.bits(), 0));
    // Move PC to interrupt on request handler
    state.regs.pc = INTERRUPT_REQUEST_HANDLER;
}
