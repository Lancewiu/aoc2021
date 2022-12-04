use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str;

fn debug_octopi(octopi: &[u32]) {
    for row in 0..10 {
        let start = row * 10;
        let end = start + 10;
        eprintln!("{}: {:?}", row, &octopi[start..end]);
    }
    eprintln!();
}

fn empower_adjacent(octopi: &mut [u32], index: usize) {
    let x = (index % 10) as i32;
    let y = (index / 10) as i32;
    for row in -1..2 {
        for col in -1..2 {
            let adj_x = x + col;
            let adj_y = y + row;
            let adjacent_i = adj_x + adj_y * 10;
            if index as i32 == adjacent_i
                || !(0..100).contains(&adjacent_i)
                || adj_x > 9
                || adj_y > 9
            {
                continue;
            }
            octopi[adjacent_i as usize] += 1;
        }
    }
}

fn cascade_flashes(octopi: &mut [u32]) -> u64 {
    let mut flashed: Vec<usize> = Vec::new();
    let mut num_flashes: u64 = 0;

    loop {
        let new_flashes: Vec<usize> = octopi
            .iter()
            .enumerate()
            .filter(|(_, octopus)| 10 <= **octopus)
            .filter_map(|(i, _)| match flashed.binary_search(&i) {
                Ok(_) => None,
                Err(_) => Some(i),
            })
            .collect();
        if new_flashes.is_empty() {
            break;
        }
        num_flashes += new_flashes.len() as u64;
        flashed.extend_from_slice(&new_flashes[..]);
        flashed.sort();

        for flash_index in new_flashes.into_iter() {
            empower_adjacent(octopi, flash_index);
        }
    }

    num_flashes
}

fn process_lines(reader: impl BufRead) -> anyhow::Result<u64> {
    let mut octopi: Vec<u32> = Vec::new();
    for line_result in reader.lines() {
        let line = line_result?;
        let mut powers: Vec<u32> = line
            .chars()
            .map(|digit| digit.to_digit(10))
            .collect::<Option<Vec<u32>>>()
            .ok_or_else(|| anyhow::Error::msg("invalid digit encountered"))?;
        octopi.append(&mut powers);
    }

    let mut num_flashes: u64 = 0;
    for step in 0..10 {
        eprintln!("Step {}", step);
        debug_octopi(&octopi[..]);
        octopi.iter_mut().for_each(|octopus| {
            *octopus += 1;
        });

        num_flashes += cascade_flashes(&mut octopi[..]);

        octopi
            .iter_mut()
            .filter(|octopus| 10 <= **octopus)
            .for_each(|flashed| {
                *flashed = 0;
            });
    }

    Ok(num_flashes)
}
fn main() {
    //const INPUT_PATH: &str = "data/input.txt";
    const INPUT_PATH: &str = "data/test_input.txt";

    match File::open(INPUT_PATH) {
        Ok(file) => match process_lines(BufReader::new(file)) {
            Err(err) => {
                eprintln!("Could not process file {}:\n  {}", INPUT_PATH, err);
            }
            Ok(error_score) => {
                println!("error score total: {}", error_score);
            }
        },
        Err(err) => {
            eprintln!("Error opening file {}:\n  {}", INPUT_PATH, err);
        }
    }
}
