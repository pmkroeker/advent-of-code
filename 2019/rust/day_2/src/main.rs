use std::fs::File;
use std::error::Error;
use std::io::{prelude::*};
use std::convert::TryInto;
use std::process::exit;

/*
    Position 0: 1, 2, 99
        1 add numbers from postions together
        2 multiply numbers from positions together
        99 program is finished and should immediately halt
    [opcode, input 1 pos, input 2 pos, output pos]

    Step forward 4 positions then repeat
*/

fn main() {
    // Open file
    let intcode_file = parse_file("input.txt");
    let mut intcode = match intcode_file {
        Ok(v) => v,
        Err(e) => {
            println!("Error parsing file: {:?}", e);
            exit(1);
        }
    };
    // Perform operations!
    let mut current_position: usize = 0;
    let code_length = intcode.len();
    // Problem stats to changes theses values!
    intcode[1] = 12;
    intcode[2] = 2;
    loop {
        let opcode = intcode[current_position];
        // println!("{}\n", current_position);
        if opcode == 99 || current_position >= code_length - 1 {
            break;
        }
        let pos_1: usize = current_position + 1;
        let pos_2: usize = current_position + 2;
        let pos_3: usize = current_position + 3;

        let pos_first: usize = intcode[pos_1].try_into().unwrap();
        let pos_second: usize = intcode[pos_2].try_into().unwrap();
        let res_pos: usize = intcode[pos_3].try_into().unwrap();
        let first_val = intcode[pos_first];
        let second_val = intcode[pos_second];
        if opcode == 1 {
            intcode[res_pos] = first_val + second_val;
        } else if opcode == 2 {
            intcode[res_pos] = first_val * second_val;
        }
        current_position += 4;
    }
    println!("{}", intcode[0]);
}

/// Parses file and returns comma separated int list as a Vector of u16
fn parse_file(filename: &'static str) -> Result<Vec<u64>, Box<dyn Error>> {
    let mut file = File::open(filename)?;
    let mut intcode_str = String::new();
    // Read file contents to string
    file.read_to_string(&mut intcode_str)?;
    // Split contents into vector from splitting on commas
    let code_split: Vec<&str> = intcode_str.split(',').collect();
    // Initialize int vector from string vector
    let mut intcode: Vec<u64> = Vec::new();
    for code in code_split {
        let parsed_att = code.parse::<u64>();
        let parsed = match parsed_att {
            Ok(v) => v,
            Err(..) => 0
        };
        intcode.push(parsed);
    }
    Ok(intcode)
}
