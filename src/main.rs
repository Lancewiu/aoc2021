use std::cmp;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str;

fn process_lines(mut reader: impl BufRead) -> anyhow::Result<usize> {
    let mut line = String::new();
    reader.read_to_string(&mut line)?;
    let initial_state = line
        .trim()
        .split(',')
        .map(|token| token.trim().parse::<u32>())
        .collect::<Result<Vec<u32>, _>>()?;
    let pos_offset = *initial_state.iter().min().unwrap();
    let pos_max = ((*initial_state.iter().max().unwrap()) - pos_offset) as usize;
    let mut histogram = vec![0u32; pos_max + 1];
    initial_state.into_iter().for_each(|i| {
        histogram[(i - pos_offset) as usize] += 1;
    });

    (0..=pos_max)
        .map(|pos| {
            histogram
                .iter()
                .copied()
                .enumerate()
                .map(|(i, count)| (count as usize) * (cmp::max(i, pos) - cmp::min(i, pos)))
                .sum()
        })
        .min()
        .ok_or_else(|| unreachable!())
}
fn main() {
    const INPUT_PATH: &str = "data/input.txt";

    match File::open(INPUT_PATH) {
        Ok(file) => match process_lines(BufReader::new(file)) {
            Err(err) => {
                eprintln!("Could not process file {}:\n  {}", INPUT_PATH, err);
            }
            Ok(gas_use) => {
                println!("min gas use: {}", gas_use);
            }
        },
        Err(err) => {
            eprintln!("Error opening file {}:\n  {}", INPUT_PATH, err);
        }
    }
}
