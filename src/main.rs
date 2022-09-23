mod ship;

use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn process_lines(reader: impl BufRead) -> Result<ship::Pilot> {
    let mut pilot = ship::Pilot::new();
    for line_result in reader.lines() {
        pilot.process(&line_result?.parse::<ship::Command>()?);
    }
    return Ok(pilot);
}
fn main() {
    const INPUT_PATH: &str = "data/input.txt";

    match File::open(INPUT_PATH) {
        Ok(file) => match process_lines(BufReader::new(file)) {
            Err(err) => {
                eprintln!("Could not process file {}:\n  {}", INPUT_PATH, err);
            }
            Ok(ship) => {
                println!("Depth multiple: {}", ship.get_depth() * ship.get_forward());
            }
        },
        Err(err) => {
            eprintln!("Error opening file {}:\n  {}", INPUT_PATH, err);
        }
    }
}
