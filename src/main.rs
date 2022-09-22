use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn process_lines(mut reader: impl BufRead) -> Result<u32> {
    let mut line = String::new();
    let mut prev_depth: u32 = 0;
    let mut depth_count: u32 = 0;
    loop {
        if 0 == reader.read_line(&mut line)? {
            break;
        }
        let depth: u32 = line.parse()?;

        if 0 > prev_depth && depth > prev_depth {
            depth_count += 1;
        }
        prev_depth = depth;
    }

    return Ok(depth_count);
}
fn main() {
    const input_path: &str = "data/input.txt";

    match File::open(input_path) {
        Ok(file) => match process_lines(BufReader::new(file)) {
            Err(err) => {
                eprintln!("Could not process file {}:\n  {}", input_path, err);
            }
            Ok(depth_count) => {
                println!("Depth Count: {}", depth_count);
            }
        },
        Err(err) => {
            eprintln!("Error opening file {}:\n {}", input_path, err);
        }
    }
}
