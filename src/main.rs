use std::io::Read;
use std::io;

mod computer_state;
mod test;

fn main() {
    let args: Vec<_> = std::env::args().collect();

    // Get input file from command line arguments
    match args.iter().position(|arg| arg.starts_with("-f")) {
        Some(path_index) => {
            let path = &args[path_index + 1];

            // Read the program from file
            match std::fs::read(path) {
                Ok(input) => {
                    let mut computer = computer_state::ComputerState::new();
                    computer.set_up_state(&input);

                    loop {
                        // Clears screen and moves cursor to the top of the terminal
                        print!("\x1B[2J\x1B[1;1H");
                        // Print the computer's current state, including registers and current instruction
                        println!("{}", computer.get_state_str());

                        // Read a single byte and discard
                        // Waits for user before stepping the program
                        let _ = io::stdin().read(&mut [0u8]).unwrap();

                        computer.execute_next();
                    }
                }
                Err(error) => {
                    eprintln!("Error while trying to read from stdin. Error: {error}");
                }
            }
        }
        None => {
            eprintln!("Error! Input file not provided");
        }
    }
}
