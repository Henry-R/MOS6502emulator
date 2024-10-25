mod operations;

struct StatusFlags {
    // (n) Negative
    pub n: bool,

    // (v) Overflow
    pub v: bool,

    // (b) Break
    pub b: bool,

    // (d) Decimal
    pub d: bool,

    // (i) Interrupt disable
    pub i: bool,

    // (z) Zero
    pub z: bool,

    // (c) Carry
    pub c: bool,
}

struct ComputerState {
    // REGISTERS
    // (A) Accumulator
    pub acc: i8,
    // (P) Status register
    pub sta: StatusFlags,
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
