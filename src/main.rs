use crate::computer_state::ComputerState;

mod computer_state;

fn main() {
    use crate::computer_state::operations::*;

    let mut state = ComputerState::new();
    INSTRUCTION_TABLE[0](&mut state);
    let acc = state.regs.acc;

    println!("Hello, world! {acc}");
}
