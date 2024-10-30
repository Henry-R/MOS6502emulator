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
    // Carry and overflow bits set TODO maybe subtle carry logic could create bugs
    if quot > 0 {
        state.regs.sta.insert(StatusFlags::c);
        state.regs.sta.insert(StatusFlags::v);
    }
    // Result was zero
    if rem == 0 { state.regs.sta.insert(StatusFlags::z) }
}

/// Helper function that converts an u8 byte to the i8 represented by its bits
const fn u8_to_i8(n: u8) -> i8 {
    (n as i16 - i8::MAX as i16) as i8
}

/// ADc (intermediate addressing mode)
fn add_im(state: &mut ComputerState) { add(state, u8_to_i8(state.fetch_intermediate())); }
/// ADc (zero-page addressing mode)
fn add_zp(state: &mut ComputerState) { add(state, u8_to_i8(state.fetch_zero_page())); }
/// ADc (zero-page X addressing mode)
fn add_zpx(state: &mut ComputerState) { add(state, u8_to_i8(state.fetch_zero_page_x())); }
/// ADc (zero-page Y addressing mode)
fn add_zpy(state: &mut ComputerState) { add(state, u8_to_i8(state.fetch_zero_page_y())); }
/// ADc (absolute addressing mode)
fn add_ab(state: &mut ComputerState) { add(state, u8_to_i8(state.fetch_absolute())); }
/// ADc (absolute X addressing mode)
fn add_abx(state: &mut ComputerState) { add(state, u8_to_i8(state.fetch_absolute_x())); }
/// ADc (absolute Y addressing mode)
fn add_aby(state: &mut ComputerState) { add(state, u8_to_i8(state.fetch_absolute_y())); }
/// ADc (indirect X addressing mode)
fn add_inx(state: &mut ComputerState) { add(state, u8_to_i8(state.fetch_indexed_indirect())); }
/// ADc (indirect Y addressing mode)
fn add_iny(state: &mut ComputerState) { add(state, u8_to_i8(state.fetch_indirect_indexed())); }

#[cfg(test)]
mod tests {
    use crate::computer_state::{ComputerState, StatusFlags};
    use crate::computer_state::operations::arithmetic::add;

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