mod operations;

struct ComputerState {
    // REGISTERS
    // (A) Accumulator
    pub acc: i8,
    // (P) Status register
    pub sta: i8,
    // (PC) Program counter
    pub pc: i16,
    // (S) Stack pointer
    pub stk: i8,
    // (X) Index register
    pub x: i8,
    // (Y) Index register
    pub y: i8
}

fn main() {


    println!("Hello, world!");
}
