use crate::computer_state::status_register::StatusRegister;

pub const fn get_zero_neg(n: u8) -> StatusRegister {
    StatusRegister::new() |
    if n == 0            { StatusRegister::Z } else {StatusRegister::EMPTY} |
    if n < i8::MAX as u8 { StatusRegister::N } else {StatusRegister::EMPTY}
}