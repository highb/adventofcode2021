use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

fn main() -> std::io::Result<()> {
    let mut file = File::open("./input")?;
    let mut puzzle_input = String::new();
    file.read_to_string(&mut puzzle_input)?;

    println!("Puzzle Input:");
    let mut last = -1;
    let mut increasing = 0;
    for input in puzzle_input.lines() {
        let measurement: i64 = i64::from_str(input).unwrap();
        let mut change = "";
        if last < 0 {
            change = "N/A - no previous measurement";
        } else {
            if measurement > last {
                change = "INCREASING";
                increasing += 1;
            } else if measurement < last {
                change = "DECREASING";
            }
        }
        println!("{} ({})", measurement, change);
        last = measurement;
    }

    println!("Increased {} times", increasing);

    Ok(())
}
