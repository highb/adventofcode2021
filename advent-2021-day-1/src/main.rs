use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

fn main() -> std::io::Result<()> {
    let mut file = File::open("./input")?;
    let mut puzzle_input = String::new();
    file.read_to_string(&mut puzzle_input)?;

    let mut measurements: Vec<i32> = Vec::new();
    for input in puzzle_input.lines() {
        measurements.push(i32::from_str(input).unwrap());
    }

    let mut last = -1;
    let mut increasing = 0;
    for window in measurements.windows(3) {
        println!("{:#?}", window);
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
        println!("{} ({})", measurement, change);
        last = measurement;
    }

    println!("Increased {} times", increasing);

    Ok(())
}
