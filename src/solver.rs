use crate::digit::Digit;

use std::collections::HashMap;

use std::fmt;

#[derive(Debug)]
pub struct InvalidKeyInput(String);

impl fmt::Display for InvalidKeyInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid key input string.\n {}", self.0)
    }
}

impl std::error::Error for InvalidKeyInput {}

pub struct Key {
    four: Digit,
    seven: Digit,
}

impl Key {
    pub fn try_from_input(input: &[&str]) -> Result<Self, InvalidKeyInput> {
        let mut key_map: HashMap<u8, Digit> = HashMap::with_capacity(2);

        for digit in input {
            match digit.chars().count() {
                3 => {
                    key_map.insert(7, Digit::from(*digit));
                }
                4 => {
                    key_map.insert(4, Digit::from(*digit));
                }
                _ => {}
            }
        }

        for expected_digit in [4, 7] {
            if !key_map.contains_key(&expected_digit) {
                return Err(InvalidKeyInput(format!("missing digit {}", expected_digit)));
            }
        }

        Ok(Key {
            four: key_map.remove(&4).unwrap(),
            seven: key_map.remove(&7).unwrap(),
        })
    }

    pub fn solve(&self, s: &str) -> Option<u8> {
        let digit = Digit::from(s);
        match s.chars().count() {
            2 => Some(1),
            3 => Some(7),
            4 => Some(4),
            5 => {
                if digit.contains(&self.seven) {
                    Some(3)
                } else if 3 == digit.diff(&self.four).chars().count() {
                    Some(2)
                } else {
                    Some(5)
                }
            }
            6 => {
                if digit.contains(&self.four) {
                    Some(9)
                } else if digit.contains(&self.seven) {
                    Some(0)
                } else {
                    Some(6)
                }
            }
            7 => Some(8),
            _ => None,
        }
    }
}
