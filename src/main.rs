use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str;

fn process_lines(mut reader: impl BufRead) -> anyhow::Result<usize> {
    let mut line = String::new();
    reader.read_to_string(&mut line)?;
    let mut school: Vec<u32> = line
        .trim()
        .split(',')
        .map(|token| token.trim().parse())
        .collect::<Result<Vec<u32>, _>>()?;
    for _ in 0..80 {
        let mut num_new_fish = 0u32;
        for fish in school.iter_mut() {
            if 0 == *fish {
                *fish = 6;
                num_new_fish += 1;
            } else {
                *fish -= 1;
            }
        }
        (0..num_new_fish).for_each(|_| school.push(8));
    }
    Ok(school.len())
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
