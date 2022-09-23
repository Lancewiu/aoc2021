use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn big_endian_bool_slice_to_num(array: &[bool]) -> u32 {
    array
        .into_iter()
        .rev()
        .enumerate()
        .map(|(i_place, bit_value)| (*bit_value as u32) << i_place)
        .sum()
}

fn process_lines(reader: impl BufRead) -> Result<u64> {
    // big endian count
    let mut one_count = [0u32; 12];
    let mut num_lines = 0u32;
    for line_result in reader.lines() {
        let line = line_result?;
        for (i, c) in line.chars().enumerate() {
            if '1' == c {
                one_count[i] += 1;
            }
        }
        num_lines += 1;
    }
    let mut majority = [false; 12];
    let mut minority = [false; 12];
    for (i, num_ones) in one_count.into_iter().enumerate() {
        let common_bit = num_lines - num_ones < num_ones;
        majority[i] = common_bit;
        minority[i] = !common_bit;
    }
    let gamma_rate = big_endian_bool_slice_to_num(&majority);
    let epsilon_rate = big_endian_bool_slice_to_num(&minority);
    Ok((gamma_rate as u64) * (epsilon_rate as u64))
}
fn main() {
    const INPUT_PATH: &str = "data/input.txt";

    match File::open(INPUT_PATH) {
        Ok(file) => match process_lines(BufReader::new(file)) {
            Err(err) => {
                eprintln!("Could not process file {}:\n  {}", INPUT_PATH, err);
            }
            Ok(power) => {
                println!("power consumption: {}", power);
            }
        },
        Err(err) => {
            eprintln!("Error opening file {}:\n  {}", INPUT_PATH, err);
        }
    }
}
