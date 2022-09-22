use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn process_lines(reader: impl BufRead) -> Result<u32> {
    let mut depth_window: [u32; 3] = [0, 0, 0];
    let mut depth_count: u32 = 0;
    for line_result in reader.lines() {
        let line = line_result?;
        let depth: u32 = line.parse()?;

        let new_window = [depth_window[1], depth_window[2], depth];

        let prev_sum: u32 = depth_window.iter().sum();
        let new_sum: u32 = new_window.iter().sum();
        let is_window_saturated = depth_window.into_iter().all(|d| 0 < d);
        if is_window_saturated && new_sum > prev_sum {
            depth_count += 1;
        }

        depth_window = new_window;
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
