use crate::computer_state::{ComputerState, StatusFlags};

struct AddResult {
    value: u8,
    carry: bool
}

/// ADd with carry
fn add(state: &mut ComputerState, value: i8) {
    const MAX_VALUE: i16 = i8::MAX as i16;

    // Addition with remainder
    let raw_add: i16 = state.regs.acc as i16 + value as i16;
    let (quot, rem) = (raw_add / MAX_VALUE, raw_add % MAX_VALUE);

    // REGISTERS
    state.regs.acc = rem as i8;

    // FLAGS
    // Carry bit set
    if quot > 0 { state.regs.sta.insert(StatusFlags::c); }
    // Result was zero
    if rem == 0 { state.regs.sta.insert(StatusFlags::z) }
}

/// ADc (intermediate addressing mode)
fn add_imd(state: &mut ComputerState) {
    // Load value from operand

}

#[cfg(test)]
mod tests {
    use crate::computer_state::{ComputerState, StatusFlags};
    use crate::computer_state::operations::add::add;

    #[test]
    fn test_add() {
        let mut state = ComputerState::new();

        add(&mut state, 100);
        assert_eq!(state.regs.acc, 100);
        assert!(state.regs.sta.is_empty());

        add(&mut state, 100);
        assert_eq!(state.regs.acc, 73);
        assert!(state.regs.sta.contains(StatusFlags::c));

        add(&mut state, 54);
        assert_eq!(state.regs.acc, 0);
        assert!(state.regs.sta.contains(StatusFlags::z));
    }
}