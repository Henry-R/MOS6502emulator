use crate::ComputerState;

// MOS6502 operation. Mutates the computer's state
type MosOp = fn(&mut ComputerState);

/// BRK (Force Break)
fn force_break(state: &mut ComputerState) {

    //state.regs.sta.b = true;
}

const INSTRUCTION_TABLE: [[MosOp; 1]; 1] = [
    [
        force_break
    ]
];