mod canvas;
mod geometry;

use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str;

fn process_lines(reader: impl BufRead) -> Result<u64> {
    let mut board = canvas::Canvas::new();
    for line_res in reader.lines() {
        let line_str = line_res?;
        let line: geometry::Line = line_str.parse()?;
        if !line.is_diagonal() {
            board.count_line(&line);
        }
    }
    Ok(board.iter().filter(|(_, counts)| 1 < **counts).count() as u64)
}
fn main() {
    const INPUT_PATH: &str = "data/input.txt";

    match File::open(INPUT_PATH) {
        Ok(file) => match process_lines(BufReader::new(file)) {
            Err(err) => {
                eprintln!("Could not process file {}:\n  {}", INPUT_PATH, err);
            }
            Ok(count) => {
                println!("# overlapping points: {}", count);
            }
        },
        Err(err) => {
            eprintln!("Error opening file {}:\n  {}", INPUT_PATH, err);
        }
    }
}
