
#[cfg(test)]
mod tests {
    use crate::computer_state::{ComputerState};
    use crate::computer_state::operations::arithmetic::*;

    // Unit tests shamelessly taken from https://github.com/tdinucci/6502-emulator
    // All credit for coming up with the test cases goes to tdinucci

    #[test]
    fn test_adc_im() {
        let mut state: ComputerState = ComputerState::new();

        // Initialise computer state
        state.regs.acc = 36;
        state.insert_operation_at_pc(adc_im);
        state.insert_at_pc(36);

        // Reset pc for execution
        state.regs.pc = 0;

        state.execute_next();

        assert_eq!(72, state.regs.acc);
        assert!(state.regs.sta.is_empty());
    }
}
