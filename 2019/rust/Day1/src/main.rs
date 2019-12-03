use std::fs::File;
use std::str::FromStr;
use std::io::{prelude::BufRead, BufReader};
use std::f32;

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut total_fuel: f32 = 0.0;
    for line in reader.lines() {
        let value = line.unwrap();
        let immut_value = &value;
        let to_parse: &str = &immut_value;
        let num = f32::from_str(to_parse).unwrap();
        let fuel = calc_fuel(num);
        total_fuel += fuel;
    }
    println!("{}", total_fuel);
    Ok(())
}

/// Fuel required is mass / 3 rounded down then subtract 2
fn calc_fuel(mass: f32) -> f32 {
    let halved = mass / 3.0;
    let floored = halved.floor();
    floored - 2.0
}
