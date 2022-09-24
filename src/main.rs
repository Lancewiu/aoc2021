mod bingo;
mod draw;

use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str;

fn play_bingo(draws: &[u32], mut boards: Vec<bingo::Board>) -> u64 {
    for v in draws.iter().copied() {
        boards.iter_mut().for_each(|b| b.mark(v));
        if 1 == boards.len() {
            if boards[0].has_won() {
                return (boards[0].score() as u64) * (v as u64);
            }
        } else {
            boards = boards.into_iter().filter(|b| !b.has_won()).collect();
        }
    }
    unreachable!();
}

fn process_lines(reader: impl BufRead) -> Result<(Vec<u32>, Vec<bingo::Board>)> {
    let mut lines_iter = reader.lines();
    let draw_str = lines_iter.next().ok_or(draw::ParseDrawError::from(
        "could not find draw buffer in file.",
    ))??;

    let draw_buffer: Vec<u32> = draw_str
        .split(',')
        .map(|token| token.parse::<u32>())
        .collect::<Result<Vec<u32>, _>>()?;

    let mut boards: Vec<bingo::Board> = Vec::new();
    let mut board_buffer: Vec<u32> = Vec::new();
    lines_iter.next(); // ignore first empty line
    for lines_res in lines_iter {
        let line = lines_res?;
        if line.is_empty() {
            boards.push(bingo::Board::try_from(&board_buffer[..])?);
            board_buffer.clear();
        } else {
            for value in line.split_whitespace() {
                board_buffer.push(value.parse()?);
            }
        }
    }
    //the last board line is not delimited by an empty space.
    boards.push(bingo::Board::try_from(&board_buffer[..])?);

    Ok((draw_buffer, boards))
}
fn main() {
    const INPUT_PATH: &str = "data/input.txt";

    match File::open(INPUT_PATH) {
        Ok(file) => match process_lines(BufReader::new(file)) {
            Err(err) => {
                eprintln!("Could not process file {}:\n  {}", INPUT_PATH, err);
            }
            Ok((draws, boards)) => {
                println!("Final score: {}", play_bingo(&draws[..], boards));
            }
        },
        Err(err) => {
            eprintln!("Error opening file {}:\n  {}", INPUT_PATH, err);
        }
    }
}
