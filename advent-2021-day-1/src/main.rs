use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

#[macro_use]
extern crate log;

fn main() -> std::io::Result<()> {
    env_logger::init();

    // Accept a file path as the only CLI arg
    let path = std::env::args().nth(1).expect("USAGE: cmd <file>");

    // Read in file
    let mut file = File::open(path)?;
    let mut puzzle_input = String::new();
    file.read_to_string(&mut puzzle_input)?;

    let measurements: Vec<i32> = input_to_measurements(puzzle_input);
    let increased = measurement_statistics(measurements);

    println!("Increased {} times", increased);

    Ok(())
}

fn input_to_measurements(input: String) -> Vec<i32> {
    let mut measurements: Vec<i32> = Vec::new();

    for line in input.lines() {
        measurements.push(i32::from_str(line).unwrap());
    }

    return measurements
}

fn measurement_statistics(measurements: Vec<i32>) -> i32 {
    let mut last = -1;
    let mut increasing = 0;
    for window in measurements.windows(3) {
        debug!("{:#?}", window);
        let mut change = "";
        let measurement = window.iter().sum();
        if last < 0 {
            change = "N/A - no previous measurement";
        } else {
            if measurement > last {
                change = "increased";
                increasing += 1;
            } else if measurement < last {
                change = "decreased";
            }
        }
        debug!("{} ({})", measurement, change);
        last = measurement;
    }

    return increasing
}
