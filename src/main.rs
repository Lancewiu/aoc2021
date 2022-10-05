use std::collections::HashMap;
use std::fs::File;
use std::hint;
use std::io::{BufRead, BufReader};
use std::str;

fn verify_token(line: String) -> Option<char> {
    let mut prev_open_brackets: Vec<char> = Vec::new();
    for token in line.chars() {
        match token {
            '(' | '[' | '{' | '<' => {
                prev_open_brackets.push(token);
            }
            ')' | ']' | '}' | '>' => {
                let opposite = match token {
                    ')' => '(',
                    ']' => '[',
                    '}' => '{',
                    '>' => '<',
                    _ => unsafe {
                        hint::unreachable_unchecked();
                    },
                };
                if Some(opposite) != prev_open_brackets.pop() {
                    return Some(token);
                }
            }
            _ => unreachable!(),
        }
    }
    None
}

fn process_lines(reader: impl BufRead) -> anyhow::Result<u64> {
    let score_table: HashMap<char, u64> =
        HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    let mut scoreboard = HashMap::from([(')', 0u64), (']', 0), ('}', 0), ('>', 0)]);
    for line_result in reader.lines() {
        if let Some(error_bracket) = verify_token(line_result?) {
            scoreboard.entry(error_bracket).and_modify(|count| {
                *count += 1;
            });
        }
    }
    Ok(scoreboard
        .into_iter()
        .map(|(bracket, count)| score_table[&bracket] * count)
        .sum())
}
fn main() {
    const INPUT_PATH: &str = "data/input.txt";

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
