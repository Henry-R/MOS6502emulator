use crate::ComputerState;

#[cfg(test)]
mod tests {
    #[test]
    fn test_stk_peek_byte() {
        state: ComputerState::new();
        let result = state.stk_peek_byte();
        assert_eq!(0u8, result);
    }
}