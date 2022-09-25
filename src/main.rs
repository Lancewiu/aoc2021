use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str;

fn process_lines(reader: impl BufRead) -> anyhow::Result<usize> {
    let mut digit_count: HashMap<usize, usize> = HashMap::with_capacity(4);
    digit_count.insert(2, 0);
    digit_count.insert(3, 0);
    digit_count.insert(4, 0);
    digit_count.insert(7, 0);
    for line_res in reader.lines() {
        let line = line_res?;
        line.split('|')
            .next_back()
            .map(|s| s.trim())
            .expect("malformed input")
            .split_whitespace()
            .map(|digit| digit.chars().count())
            .for_each(|count| {
                digit_count.entry(count).and_modify(|v| *v += 1);
            });
    }
    Ok(digit_count.into_values().sum())
}
fn main() {
    const INPUT_PATH: &str = "data/input.txt";

    match File::open(INPUT_PATH) {
        Ok(file) => match process_lines(BufReader::new(file)) {
            Err(err) => {
                eprintln!("Could not process file {}:\n  {}", INPUT_PATH, err);
            }
            Ok(digit_count) => {
                println!("# special digits: {}", digit_count);
            }
        },
        Err(err) => {
            eprintln!("Error opening file {}:\n  {}", INPUT_PATH, err);
        }
    }
}
