
#[cfg(test)]
mod tests {
    use crate::computer_state::{ComputerState};
    use crate::computer_state::operations::arithmetic::*;

    #[test]
    fn test_adc_im() {
        let mut state: ComputerState = ComputerState::new();

        let old_pc = state.regs.pc;
        state.regs.acc = 36;
        state.insert_operation_at_pc(adc_im);
        state.insert_at_pc(36);
        state.regs.pc = old_pc;

        state.execute_next();

        assert_eq!(72, state.regs.acc);
        assert!(state.regs.sta.is_empty());
    }
}
