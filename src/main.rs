use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn process_lines(reader: impl BufRead) -> Result<u32> {
    let mut prev_depth: u32 = 0;
    let mut depth_count: u32 = 0;
    for line_result in reader.lines() {
        let line = line_result?;
        let depth: u32 = line.parse()?;
        if 0 < prev_depth && depth > prev_depth {
            depth_count += 1;
        }
        prev_depth = depth;
    }
    return Ok(depth_count);
}
fn main() {
    const INPUT_PATH: &str = "data/input.txt";

    match File::open(INPUT_PATH) {
        Ok(file) => match process_lines(BufReader::new(file)) {
            Err(err) => {
                eprintln!("Could not process file {}:\n  {}", INPUT_PATH, err);
            }
            Ok(depth_count) => {
                println!("Depth Count: {}", depth_count);
            }
        },
        Err(err) => {
            eprintln!("Error opening file {}:\n  {}", INPUT_PATH, err);
        }
    }
}
