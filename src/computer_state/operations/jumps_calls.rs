use crate::computer_state::ComputerState;

/// JMP (absolute addressing mode)
/// Opcode: 4C
pub fn jmp_ab(state: &mut ComputerState) {
    let addr = state.mem.fetch_next_nibble();
    state.mem.pc.set(usize::from(addr))
}

/// JMP (indirect addressing mode)
/// Opcode: 6C
pub fn jmp_in(state: &mut ComputerState) {
    let addr = state.mem.fetch_indirect();
    state.mem.pc.set(usize::from(addr))
}


/// JSR (jump to subroutine)
/// Opcode: 20
pub fn jsr(state: &mut ComputerState) {
    let sub_addr = state.mem.fetch_next_nibble();
    let ret_addr = state.mem.pc.get();
    state.mem.push_nibble_on_stack(ret_addr as u16);
    state.mem.pc.set(sub_addr as usize)
}

/// RTS (return from subroutine)
/// Opcode: 60
pub fn rts(state: &mut ComputerState) {
    let ret_addr = state.mem.pop_nibble_from_stack();
    state.mem.pc.set(ret_addr as usize);
}
