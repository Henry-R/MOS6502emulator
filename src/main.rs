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


struct Registers {
    // (A) Accumulator
    pub acc: u8,
    // (P) Status register
    pub sta: StatusFlags,
    // (PC) Program counter
    pub pc: u16,
    // (S) Stack pointer
    pub stk: u8,
    // (X) Index register
    pub x: u8,
    // (Y) Index register
    pub y: u8
}

struct ComputerState {
    // MEMORY
    // Each page is 256 bytes
    // First page is reserved for the Zero-Page ($0000-$00FF)
    // Second page is reserved for system stack ($0100-$01FF)
    // Last 6 bytes are reserved for interrupts ($FFFA-$FFFF)
    mem: [u8; 2^16],

    regs: Registers,
}

impl ComputerState {
    // Important memory locations
    const ZERO_PAGE: u16 = 0x0000;
    const STACK_PAGE: u16 = 0x0100;
    
    fn new() -> ComputerState {
        ComputerState {
            mem: [0u8; 2^16],
            regs: Registers {
                acc: 0,
                sta: StatusFlags {
                    n: false,
                    v: false,
                    b: false,
                    d: false,
                    i: false,
                    z: false,
                    c: false,
                },
                pc: 0,
                stk: 0xFF,  // Stack grows downwards, so initialise stack to top of memory
                x: 0,
                y: 0,
            },
        }
    }

    // STACK INSTRUCTIONS
    /// Returns the byte at the top of the stack without mutating memory
    fn stk_peek(& self) -> u8 {
        self.mem[Self::STACK_PAGE + self.regs.stk as u16]
    }

    /// Returns the byte at the top of the stack and reduces the stack pointer by one
    fn stk_pop(&mut self) -> u8 {
        let ret = self.stk_peek();
        self.regs.stk += 1;
        ret
    }

    /// Increases the stack pointer by one, and pushed the supplied byte onto this stack location
    fn stk_push(&mut self, byte: u8){
        self.regs.stk -= 1;
        self.mem[Self::STACK_PAGE + self.regs.stk as u16] = byte;
    }
}

fn main() {


    println!("Hello, world!");
}
