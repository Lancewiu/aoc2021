use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str;

const fn invert_bracket(bracket: char) -> Option<char> {
    match bracket {
        ')' => Some('('),
        '(' => Some(')'),
        '[' => Some(']'),
        ']' => Some('['),
        '{' => Some('}'),
        '}' => Some('{'),
        '<' => Some('>'),
        '>' => Some('<'),
        _ => None,
    }
}

const fn score_bracket(bracket: char) -> Option<u64> {
    match bracket {
        ')' => Some(1),
        ']' => Some(2),
        '}' => Some(3),
        '>' => Some(4),
        _ => None,
    }
}

fn complete_token(line: &str) -> Option<Vec<char>> {
    let mut prev_open_brackets: Vec<char> = Vec::new();
    for token in line.chars() {
        match token {
            '(' | '[' | '{' | '<' => {
                prev_open_brackets.push(token);
            }
            ')' | ']' | '}' | '>' => {
                let opposite = unsafe { invert_bracket(token).unwrap_unchecked() };
                if Some(opposite) != prev_open_brackets.pop() {
                    return None;
                }
            }
            _ => unreachable!(),
        }
    }
    prev_open_brackets
        .into_iter()
        .rev()
        .map(invert_bracket)
        .collect::<Option<Vec<char>>>()
}

fn score_chunk(chunk: &[char]) -> Option<u64> {
    let mut score = 0u64;
    for token in chunk.iter().copied() {
        score *= 5;
        if let Some(score_value) = score_bracket(token) {
            score += score_value;
        } else {
            return None;
        }
    }

    Some(score)
}

fn process_lines(reader: impl BufRead) -> anyhow::Result<u64> {
    let mut scores: Vec<u64> = Vec::new();
    for line_result in reader.lines() {
        if let Some(complete_chunk) = complete_token(line_result?.as_str()) {
            scores.push(score_chunk(&complete_chunk[..]).expect("invalid chunk given"));
        }
    }
    scores.sort();
    Ok(scores[scores.len() / 2])
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
