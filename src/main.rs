use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str;

fn process_lines(mut reader: impl BufRead) -> anyhow::Result<usize> {
    let mut line = String::new();
    reader.read_to_string(&mut line)?;
    let mut histogram = [0usize; 9];
    let initial_state = line
        .trim()
        .split(',')
        .map(|token| token.trim().parse::<usize>())
        .collect::<Result<Vec<usize>, _>>()?;
    initial_state.into_iter().for_each(|i| {
        histogram[i] += 1;
    });

    for _ in 0..256 {
        let num_reset_fish = histogram[0];
        for i in 0usize..8 {
            histogram[i] = histogram[i + 1];
        }
        histogram[6] += num_reset_fish;
        histogram[8] = num_reset_fish;
    }
    Ok(histogram.into_iter().sum())
}
fn main() {
    const INPUT_PATH: &str = "data/input.txt";

    match File::open(INPUT_PATH) {
        Ok(file) => match process_lines(BufReader::new(file)) {
            Err(err) => {
                eprintln!("Could not process file {}:\n  {}", INPUT_PATH, err);
            }
            Ok(num_fish) => {
                println!("# fish: {}", num_fish);
            }
        },
        Err(err) => {
            eprintln!("Error opening file {}:\n  {}", INPUT_PATH, err);
        }
    }
}
