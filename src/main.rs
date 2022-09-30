mod matrix;

use matrix::Matrix;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str;

fn process_lines(reader: impl BufRead) -> anyhow::Result<u64> {
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
    let map = Matrix::try_from_raw(&raw_map[..], WIDTH)?;
    let mut risks = 0u64;
    let map_height = map.get_height();
    for h in 0..map_height {
        for w in 0..WIDTH {
            let cave_height = map[(w, h)];
            let is_north_higher = 0 == h || cave_height < map[(w, h - 1)];
            let is_south_higher = map_height == h + 1 || cave_height < map[(w, h + 1)];
            let is_west_higher = 0 == w || cave_height < map[(w - 1, h)];
            let is_east_higher = WIDTH == w + 1 || cave_height < map[(w + 1, h)];
            if is_north_higher && is_south_higher && is_west_higher && is_east_higher {
                risks += (cave_height as u64) + 1;
            }
        }
    }

    Ok(risks)
}
fn main() {
    const INPUT_PATH: &str = "data/input.txt";

    match File::open(INPUT_PATH) {
        Ok(file) => match process_lines(BufReader::new(file)) {
            Err(err) => {
                eprintln!("Could not process file {}:\n  {}", INPUT_PATH, err);
            }
            Ok(risks) => {
                println!("total risk: {}", risks);
            }
        },
        Err(err) => {
            eprintln!("Error opening file {}:\n  {}", INPUT_PATH, err);
        }
    }
}
