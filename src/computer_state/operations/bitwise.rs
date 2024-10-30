use crate::computer_state::{ComputerState, StatusFlags};
use crate::computer_state::operations::u8_to_i8;

// AND
/// AND (bitwise and)
fn and(state: &mut ComputerState, value: i8) {
    // REGISTERS
    state.regs.acc &= value;

    // FLAGS
    if state.regs.acc == 0 { state.regs.sta.insert(StatusFlags::z); }
    if state.regs.acc < 0 { state.regs.sta.insert(StatusFlags::n); }
}

/// AND (intermediate addressing mode)
fn and_im(state: &mut ComputerState) {
    let value = u8_to_i8(state.fetch_intermediate());
    and(state, value);
}
/// AND (zero-page addressing mode)
fn and_zp(state: &mut ComputerState) {
    let value = u8_to_i8(state.fetch_zero_page());
    and(state, value);
}
/// AND (zero-page X addressing mode)
fn and_zpx(state: &mut ComputerState) {
    let value = u8_to_i8(state.fetch_zero_page_x());
    and(state, value);
}
/// AND (zero-page Y addressing mode)
fn and_zpy(state: &mut ComputerState) {
    let value = u8_to_i8(state.fetch_zero_page_y());
    and(state, value);
}
/// AND (absolute addressing mode)
fn and_ab(state: &mut ComputerState) {
    let value = u8_to_i8(state.fetch_absolute());
    and(state, value);
}
/// AND (absolute X addressing mode)
fn and_abx(state: &mut ComputerState) {
    let value = u8_to_i8(state.fetch_absolute_x());
    and(state, value);
}
/// AND (absolute Y addressing mode)
fn and_aby(state: &mut ComputerState) {
    let value = u8_to_i8(state.fetch_absolute_y());
    and(state, value);
}
/// AND (indirect X addressing mode)
fn and_inx(state: &mut ComputerState) {
    let value = u8_to_i8(state.fetch_indexed_indirect());
    and(state, value);
}
/// AND (indirect Y addressing mode)
fn and_iny(state: &mut ComputerState) {
    let value = u8_to_i8(state.fetch_indirect_indexed());
    and(state, value);
}


// OR
/// OR (logical bitwise inclusive or)
fn or(state: &mut ComputerState, value: i8) {
    // REGISTERS
    state.regs.acc |= value;

    // FLAGS
    if state.regs.acc == 0 { state.regs.sta.insert(StatusFlags::z); }
    if state.regs.acc < 0 { state.regs.sta.insert(StatusFlags::n); }
}


/// AND (intermediate addressing mode)
fn or_im(state: &mut ComputerState) {
    let value = u8_to_i8(state.fetch_intermediate());
    or(state, value);
}
/// AND (zero-page addressing mode)
fn or_zp(state: &mut ComputerState) {
    let value = u8_to_i8(state.fetch_zero_page());
    or(state, value);
}
/// AND (zero-page X addressing mode)
fn or_zpx(state: &mut ComputerState) {
    let value = u8_to_i8(state.fetch_zero_page_x());
    or(state, value);
}
/// AND (zero-page Y addressing mode)
fn or_zpy(state: &mut ComputerState) {
    let value = u8_to_i8(state.fetch_zero_page_y());
    or(state, value);
}
/// AND (absolute addressing mode)
fn or_ab(state: &mut ComputerState) {
    let value = u8_to_i8(state.fetch_absolute());
    or(state, value);
}
/// AND (absolute X addressing mode)
fn or_abx(state: &mut ComputerState) {
    let value = u8_to_i8(state.fetch_absolute_x());
    or(state, value);
}
/// AND (absolute Y addressing mode)
fn or_aby(state: &mut ComputerState) {
    let value = u8_to_i8(state.fetch_absolute_y());
    or(state, value);
}
/// AND (indirect X addressing mode)
fn or_inx(state: &mut ComputerState) {
    let value = u8_to_i8(state.fetch_indexed_indirect());
    or(state, value);
}
/// AND (indirect Y addressing mode)
fn or_iny(state: &mut ComputerState) {
    let value = u8_to_i8(state.fetch_indirect_indexed());
    or(state, value);
}


#[cfg(test)]
mod tests {
    use crate::computer_state::ComputerState;
    use crate::computer_state::operations::bitwise::{and, or};

    #[test]
    fn test_and() {
        let mut state = ComputerState::new();

        and(&mut state, 0b0000_0000);
        assert_eq!(state.regs.acc, 0);

        state.regs.acc = 0b0101_0101;
        and(&mut state, 0b0000_1111);
        assert_eq!(state.regs.acc, 0b0000_0101);
        and(&mut state, 0b0011_0001);
        assert_eq!(state.regs.acc, 0b0000_0001);
    }

    #[test]
    fn test_or() {
        let mut state = ComputerState::new();

        or(&mut state, 0b0101_0101);
        assert_eq!(state.regs.acc, 0b0101_0101);

        or(&mut state, 0b0000_1111);
        assert_eq!(state.regs.acc, 0b0101_1111);
        or(&mut state, 0b0010_0000);
        assert_eq!(state.regs.acc, 0b0111_1111);
    }
}