use std::fs::File;
use std::io::{prelude::*};

/*
    Position 0: 1, 2, 99
        1 add numbers from postions together
        2 multiply numbers from positions together
        99 program is finished and should immediately halt
    [opcode, input 1 pos, input 2 pos, output pos]

    Step forward 4 positions then repeat
*/

fn main() -> std::io::Result<()> {
    // Open file
    let mut file = File::open("input.txt").unwrap();
    let mut intcode_str = String::new();
    // Read file contents to string
    file.read_to_string(&mut intcode_str)?;
    // Split contents into vector from splitting on commas
    let code_split: Vec<&str> = intcode_str.split("','").collect();
    // Initialize int vector from string vector
    let mut intcode: Vec<u16> = Vec::new();
    for code in code_split {
        let parsed: u16 = code.parse().unwrap();
        intcode.push(parsed);
    }
    // Perform operations!

    Ok(())
}
