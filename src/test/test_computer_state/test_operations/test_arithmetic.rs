
#[cfg(test)]
mod tests {
    use crate::computer_state::{ComputerState};
    use crate::computer_state::operations::arithmetic::*;

    #[test]
    fn test_adc_im() {
        let mut state: ComputerState = ComputerState::new();
        state.regs.acc = 36;
        state.set_addr(36, usize::from(state.regs.pc) + 1);

        add_im(&mut state);

        assert_eq!(72, state.regs.acc);
        assert!(state.regs.sta.is_empty());
    }
}