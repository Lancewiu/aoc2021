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
    let _one_count = [0u32; 12];
    let mut numbers: Vec<[bool; 12]> = Vec::new();
    for line_result in reader.lines() {
        let line = line_result?;
        let mut num = [false; 12];
        for (i, c) in line.chars().enumerate() {
            num[i] = '1' == c;
        }
        numbers.push(num);
    }

    let mut majority_set = numbers.clone();
    let mut minority_set = numbers.clone();
    for i in 0..12 {
        if 1 < majority_set.len() {
            let mut count = 0u32;
            for bits in majority_set.iter() {
                count += bits[i] as u32;
            }
            let major_bit = (majority_set.len() as u32) - count <= count;
            majority_set = majority_set
                .into_iter()
                .filter(|bits| bits[i] == major_bit)
                .collect();
        }

        if 1 < minority_set.len() {
            let mut count = 0u32;
            for bits in minority_set.iter() {
                count += bits[i] as u32;
            }
            let minor_bit = !((minority_set.len() as u32) - count <= count);
            minority_set = minority_set
                .into_iter()
                .filter(|bits| bits[i] == minor_bit)
                .collect();
        }
    }
    assert!(
        1 == majority_set.len() && 1 == minority_set.len(),
        "majority set length: {}, minority set length: {}",
        majority_set.len(),
        minority_set.len()
    );
    let oxygen_rating = big_endian_bool_slice_to_num(&majority_set[0]);
    let co2_rating = big_endian_bool_slice_to_num(&minority_set[0]);
    Ok((oxygen_rating as u64) * (co2_rating as u64))
}
fn main() {
    const INPUT_PATH: &str = "data/input.txt";

    match File::open(INPUT_PATH) {
        Ok(file) => match process_lines(BufReader::new(file)) {
            Err(err) => {
                eprintln!("Could not process file {}:\n  {}", INPUT_PATH, err);
            }
            Ok(life_support) => {
                println!("life support rating: {}", life_support);
            }
        },
        Err(err) => {
            eprintln!("Error opening file {}:\n  {}", INPUT_PATH, err);
        }
    }
}
