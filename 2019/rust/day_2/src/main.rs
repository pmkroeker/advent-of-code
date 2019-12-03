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
    let intcode = match intcode_file {
        Ok(v) => v,
        Err(e) => {
            println!("Error parsing file: {:?}", e);
            exit(1);
        }
    };
    // Perform operations!
    let code_length = intcode.len();
    // Problem stats to changes theses values!
    // Part 2 these are dynamic to produce a final result of 19690720
    // Will be between 0 & 99
    let mut final_noun = 0;
    let mut final_verb = 0;
    'noun: for noun in 00..100 {
        for verb in 0..100 {
            let mut mut_intcode = intcode.clone();
            let mut current_position: usize = 0;
            mut_intcode[1] = noun;
            mut_intcode[2] = verb;
            loop {
                let opcode = mut_intcode[current_position];
                // println!("{}\n", current_position);
                if opcode == 99 || current_position >= code_length - 1 {
                    break;
                }
                let param_1: usize = current_position + 1;
                let param_2: usize = current_position + 2;
                let param_3: usize = current_position + 3;

                let pos_first: usize = mut_intcode[param_1].try_into().unwrap();
                let pos_second: usize = mut_intcode[param_2].try_into().unwrap();
                let res_pos: usize = mut_intcode[param_3].try_into().unwrap();
                let first_val = mut_intcode[pos_first];
                let second_val = mut_intcode[pos_second];
                if opcode == 1 {
                    mut_intcode[res_pos] = first_val + second_val;
                } else if opcode == 2 {
                    mut_intcode[res_pos] = first_val * second_val;
                }
                current_position += 4;
            }
            if mut_intcode[0] == 19_690_720 {
                final_noun = noun;
                final_verb = verb;
                break 'noun;
            }
        }
    }
    println!("Noun:{} | Verb:{}", final_noun, final_verb);
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
