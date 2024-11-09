
use crate::computer_state::ComputerState;
use crate::computer_state::status_register::StatusRegister;

pub fn nop(_: &mut ComputerState) {}

/// BRK (Force Break)
/// Opcode: 00
pub fn brk(state: &mut ComputerState) {
    // Push program counter
    state.mem.push_nibble_on_stack(state.mem.pc.get() as u16);
    // Then push status with break
    state.sta |= StatusRegister::B;
    state.mem.push_on_stack(state.sta.as_byte());
    // Set interrupt status
    state.sta |= StatusRegister::I;
    // Set PC to interrupt vector
    let interrupt_vector = state.mem.fetch_nibble_from_addr(0xFFFE);
    state.mem.pc.set(usize::from(interrupt_vector));
}

/// RTI (Return from interrupt)
/// Opcode: 40
pub fn rti(state: &mut ComputerState) {
    state.sta |= StatusRegister::from_byte(state.mem.pop_from_stack());
    state.mem.pc.set(usize::from(state.mem.pop_nibble_from_stack()));
}

