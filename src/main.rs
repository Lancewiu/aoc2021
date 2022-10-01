mod basin;
mod matrix;

use matrix::Matrix;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str;

fn process_lines(reader: impl BufRead) -> anyhow::Result<usize> {
    const WIDTH: usize = 100;
    let mut raw_map: Vec<u8> = Vec::new();
    for line_result in reader.lines() {
        let line = line_result?;
        let mut row = line
            .chars()
            .map(|height_char| height_char.to_digit(10).map(|h| h as u8))
            .collect::<Option<Vec<u8>>>()
            .expect("invalid digit encountered");
        raw_map.append(&mut row);
    }
    let m = basin::Map::from_matrix(Matrix::try_from_raw(&raw_map[..], WIDTH)?);
    let mut sizes = basin::find_sizes(&m);
    sizes.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());
    Ok(sizes[0] * sizes[1] * sizes[2])
}
fn main() {
    const INPUT_PATH: &str = "data/input.txt";

    match File::open(INPUT_PATH) {
        Ok(file) => match process_lines(BufReader::new(file)) {
            Err(err) => {
                eprintln!("Could not process file {}:\n  {}", INPUT_PATH, err);
            }
            Ok(basin_mult) => {
                println!("triple basin size: {}", basin_mult);
            }
        },
        Err(err) => {
            eprintln!("Error opening file {}:\n  {}", INPUT_PATH, err);
        }
    }
}
