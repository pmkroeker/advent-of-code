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
        let fuel = fuel_per_module(num);
        total_fuel += fuel;
    }
    println!("{}", total_fuel);
    Ok(())
}

/// Fuel required is mass / 3 rounded down then subtract 2
/// Part 1
fn calc_fuel(mass: f32) -> f32 {
    let halved = mass / 3.0;
    let floored = halved.floor();
    floored - 2.0
}

/// Part 2
fn fuel_per_module(mass: f32) -> f32 {
    let fuel_for_mass = calc_fuel(mass);
    let mut fuel_for_fuel: f32 = 0.0;
    let mut prev_fuel: f32 = fuel_for_mass;
    loop {
        let fuel_req = calc_fuel(prev_fuel);
        if fuel_req <= 0.0 {
            break;
        }
        prev_fuel = fuel_req;
        fuel_for_fuel += fuel_req;
    }
    fuel_for_mass + fuel_for_fuel
}
