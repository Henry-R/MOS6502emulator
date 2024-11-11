# Emulator for the MOS6502 written in Rust. 

I do not recommend using this emulator. I am still working on it and it does not yet function perfectly. 
I would say I am 80% of the way towards making this emulator complete a large integration test such as https://github.com/Klaus2m5/6502_65C02_functional_tests

This was a personal project which gave me an excuse to get into the details of microcontrollers and finally write something substantial in Rust.

# Features
* Implements all legal opcodes with every addressing mode
* Every opcode is unit tested
* Command line interface with all registers and current instruction

# To-do
* Implement integration tests into the testing routine
* Track CPU cycles
* Implement illegal opcodes

# Some of the Details
This emulator uses a jump-table to decode instructions, which is generated at compile time. Each instruction is a function which mutates the computer's state. 
I have made an effort to make as much of the emulator use compile time functions as possible. This was done as an interesting programming challenge but also to make the emulator just a little more efficient :)
I made heavy use of Rust's modules to partition each category of instruction and section of the CPU into a hierarchy to help organise the program.

This emulator only supports illegal opcodes. Currently, if the emulator hits an illegal opcode, it will exit with an error message.

# Running the Emulator
I used Rust 1.82.0.
I wrote this emulator in RustRover, so it should compile easily in that. 

When running the emulator, you must specify a path to a file containing the binary data of the program using the ```-f``` flag
```
\.emulator -f <path to your file>
```
