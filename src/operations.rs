use crate::ComputerState;

// MOS6502 operation. Mutates the computer's state
type MosOp = fn(&mut ComputerState);
const INSTRUCTION_TABLE: [[MosOp; 16]; 16] = {
    {
        // BRK
        MosOp {}
    }
};