mod digit;
mod solver;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str;

fn merge_digits(digits: &[u8]) -> u16 {
    digits
        .iter()
        .rev()
        .copied()
        .enumerate()
        .map(|(i, d)| 10u16.pow(i as u32) * (d as u16))
        .sum()
}

fn process_lines(reader: impl BufRead) -> anyhow::Result<u64> {
    let mut sum = 0u64;
    for line_res in reader.lines() {
        let line = line_res?;
        let mut line_io = line.split('|');
        let input = line_io.next().expect("Malformed line input").trim();
        let output = line_io.next().expect("Malformed line output").trim();
        let input_vec: Vec<&str> = input.split_whitespace().collect();
        let key = solver::Key::try_from_input(&input_vec[..])?;
        let number_vec = output
            .split_whitespace()
            .map(|digit| key.solve(digit))
            .collect::<Option<Vec<u8>>>()
            .expect("failed to process output digits");
        sum += merge_digits(&number_vec[..]) as u64;
    }
    Ok(sum)
}
fn main() {
    const INPUT_PATH: &str = "data/input.txt";

    match File::open(INPUT_PATH) {
        Ok(file) => match process_lines(BufReader::new(file)) {
            Err(err) => {
                eprintln!("Could not process file {}:\n  {}", INPUT_PATH, err);
            }
            Ok(output) => {
                println!("output sum: {}", output);
            }
        },
        Err(err) => {
            eprintln!("Error opening file {}:\n  {}", INPUT_PATH, err);
        }
    }
}
