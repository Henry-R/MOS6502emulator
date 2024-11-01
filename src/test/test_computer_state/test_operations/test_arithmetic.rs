
#[cfg(test)]
mod tests {
    use crate::computer_state::{ComputerState};
    use crate::computer_state::operations::arithmetic::*;
    use crate::computer_state::operations::opcode_from_operation;
    use crate::computer_state::status_register::StatusRegister;
    // Unit tests shamelessly taken from https://github.com/tdinucci/6502-emulator
    // All credit for coming up with the test cases goes to tdinucci

    #[test]
    fn test_adc_im() {
        let mut state: ComputerState = ComputerState::new();

        // Initialise computer state
        state.regs.acc = 36;
        state.set_up_state(vec![
            opcode_from_operation(adc_im),
            36
        ]);

        state.execute_next();

        assert_eq!(72, state.regs.acc);
        assert!(state.regs.sta.is_empty());
    }

    #[test]
    fn test_adc_im_zero_flag() {
        let mut state: ComputerState = ComputerState::new();

        // Initialise computer state
        state.regs.acc = 0;
        state.set_up_state(vec![
            opcode_from_operation(adc_im),
            0
        ]);

        state.execute_next();

        assert_eq!(0, state.regs.acc);
        assert!(state.regs.sta.contains_only(StatusRegister::Z));
    }

    #[test]
    fn test_adc_im_carry_flag() {
        let mut state: ComputerState = ComputerState::new();

        // Initialise computer state
        state.regs.acc = 72;
        state.set_up_state(vec![
            opcode_from_operation(adc_im),
            200
        ]);

        state.execute_next();

        assert_eq!(16, state.regs.acc);
        assert!(state.regs.sta.contains_only(StatusRegister::C));
    }

    #[test]
    fn test_adc_im_zero_and_carry_flag() {
        let mut state: ComputerState = ComputerState::new();

        // Initialise computer state
        state.regs.acc = 1;
        state.set_up_state(vec![
            opcode_from_operation(adc_im),
            0xFF
        ]);

        state.execute_next();

        assert_eq!(0, state.regs.acc);
        assert!(state.regs.sta.contains_only(StatusRegister::Z | StatusRegister::C));
    }

    #[test]
    fn test_adc_im_negative_flag() {
        let mut state: ComputerState = ComputerState::new();

        // Initialise computer state
        state.regs.acc = 32;
        state.set_up_state(vec![
            opcode_from_operation(adc_im),
            200
        ]);

        state.execute_next();

        assert_eq!(232, state.regs.acc);
        assert!(state.regs.sta.contains_only(StatusRegister::N));
    }

    #[test]
    fn test_adc_im_overflow_positive() {
        let mut state: ComputerState = ComputerState::new();

        // Initialise computer state
        state.regs.acc = 32;
        state.set_up_state(vec![
            opcode_from_operation(adc_im),
            120
        ]);

        state.execute_next();

        assert_eq!(152, state.regs.acc);
        assert!(state.regs.sta.contains_only(StatusRegister::N | StatusRegister::V));
    }

    #[test]
    fn test_adc_im_overflow_negative() {
        let mut state: ComputerState = ComputerState::new();

        // Initialise computer state
        state.regs.acc = 144;
        state.set_up_state(vec![
            opcode_from_operation(adc_im),
            208
        ]);

        state.execute_next();

        assert_eq!(96, state.regs.acc);
        assert!(state.regs.sta.contains_only(StatusRegister::C | StatusRegister::V));
    }

    #[test]
    fn test_adc_zp() {
        let mut state: ComputerState = ComputerState::new();

        // Initialise computer state
        state.regs.acc = 36;
        state.set_up_state(vec![
            opcode_from_operation(adc_zp),
            0xF1
        ]);
        state.set_addr(20, 0xF1);

        state.execute_next();

        assert_eq!(56, state.regs.acc);
        assert!(state.regs.sta.is_empty());
    }

    #[test]
    fn test_adc_zpx() {
        let mut state: ComputerState = ComputerState::new();

        // Initialise computer state
        state.regs.acc = 36;
        state.regs.x = 5;
        state.set_up_state(vec![
            opcode_from_operation(adc_zpx),
            0xF1
        ]);
        state.set_addr(20, 0xF6);

        state.execute_next();

        assert_eq!(56, state.regs.acc);
        assert!(state.regs.sta.is_empty());
    }

    #[test]
    fn test_adc_ab() {
        let mut state: ComputerState = ComputerState::new();

        // Initialise computer state
        state.regs.acc = 36;
        state.set_up_state(vec![
            opcode_from_operation(adc_ab),
            0xF1,
            0x36
        ]);
        state.set_addr(20, 0x36F1);

        state.execute_next();

        assert_eq!(56, state.regs.acc);
        assert!(state.regs.sta.is_empty());
    }

    #[test]
    fn test_adc_abx() {
        let mut state: ComputerState = ComputerState::new();

        // Initialise computer state
        state.regs.acc = 36;
        state.regs.x = 8;
        state.set_up_state(vec![
            opcode_from_operation(adc_abx),
            0xF1,
            0x36
        ]);
        state.set_addr(20, 0x36F9);

        state.execute_next();

        assert_eq!(56, state.regs.acc);
        assert!(state.regs.sta.is_empty());
    }

    #[test]
    fn test_adc_aby() {
        let mut state: ComputerState = ComputerState::new();

        // Initialise computer state
        state.regs.acc = 36;
        state.regs.y = 8;
        state.set_up_state(vec![
            opcode_from_operation(adc_aby),
            0xF1,
            0x36
        ]);
        state.set_addr(20, 0x36F9);

        state.execute_next();

        assert_eq!(56, state.regs.acc);
        assert!(state.regs.sta.is_empty());
    }

    #[test]
    fn test_adc_inx() {
        let mut state: ComputerState = ComputerState::new();

        // Initialise computer state
        state.regs.acc = 36;
        state.regs.x = 0x22;
        state.set_up_state(vec![
            opcode_from_operation(adc_inx),
            0x41
        ]);
        state.set_addr(0x34, 0x63);
        state.set_addr(0x12, 0x64);
        state.set_addr(20, 0x1234);

        state.execute_next();

        assert_eq!(56, state.regs.acc);
        assert!(state.regs.sta.is_empty());
    }

    #[test]
    fn test_adc_iny() {
        let mut state: ComputerState = ComputerState::new();

        // Initialise computer state
        state.regs.acc = 36;
        state.regs.y = 0x22;
        state.set_up_state(vec![
            opcode_from_operation(adc_iny),
            0x41
        ]);
        state.set_addr(0x31, 0x41);
        state.set_addr(0x32, 0x42);
        state.set_addr(20, 0x1256);

        state.execute_next();

        assert_eq!(56, state.regs.acc);
        assert!(state.regs.sta.is_empty());
    }
}
