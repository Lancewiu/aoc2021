
use std::cmp;
use std::error;
use std::fmt;
use std::iter;
use std::str;

#[derive(Debug)]
pub struct ParsePointError(String);

impl fmt::Display for ParsePointError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed to parse point from string.\n  {}", self.0)
    }
}

impl error::Error for ParsePointError {}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl str::FromStr for Point {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split(',');
        let x = tokens
            .next()
            .ok_or_else(|| ParsePointError("missing x in \"x,y\"".to_string()))
            .and_then(|x_str| {
                x_str.trim().parse().map_err(|parse_err| {
                    ParsePointError(format!("failed to parse x.\n  {}", parse_err))
                })
            })?;
        let y = tokens
            .next()
            .ok_or_else(|| ParsePointError("missing y in \"x,y\"".to_string()))
            .and_then(|y_str| {
                y_str.trim().parse().map_err(|parse_err| {
                    ParsePointError(format!("failed to parse y.\n  {}", parse_err))
                })
            })?;
        Ok(Point { x, y })
    }
}

pub struct Trace {
    cursor: Point,
    end: Point,
    is_done: bool,
}

impl iter::Iterator for Trace {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_done {
            return None;
        }

        let temp = self.cursor.clone();

        if self.end == self.cursor {
            self.is_done = true;
        } else {
            match self.cursor.x.cmp(&self.end.x) {
                cmp::Ordering::Equal => {}
                cmp::Ordering::Greater => {
                    self.cursor.x -= 1;
                }
                cmp::Ordering::Less => {
                    self.cursor.x += 1;
                }
            }

            match self.cursor.y.cmp(&self.end.y) {
                cmp::Ordering::Equal => {}
                cmp::Ordering::Greater => {
                    self.cursor.y -= 1;
                }
                cmp::Ordering::Less => {
                    self.cursor.y += 1;
                }
            }
        }
        Some(temp)
    }
}

impl Trace {
    fn new(start: Point, end: Point) -> Self {
        Self {
            cursor: start,
            end,
            is_done: false,
        }
    }
}

#[derive(Debug)]
pub struct ParseLineError(String);

impl fmt::Display for ParseLineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed to parse line from string.\n  {}", self.0)
    }
}

impl error::Error for ParseLineError {}

pub struct Line {
    pub from: Point,
    pub to: Point,
}

impl str::FromStr for Line {
    type Err = ParseLineError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_whitespace();
        let from = tokens
            .next()
            .ok_or_else(|| ParseLineError("could not find \"from\" point.".to_string()))
            .and_then(|from_str| {
                from_str.parse().map_err(|parse_err| {
                    ParseLineError(format!("could not parse \"from\" point.\n  {}", parse_err))
                })
            })?;
        tokens.next(); // -> symbol which is ignored
        let to = tokens
            .next()
            .ok_or_else(|| ParseLineError("could not find \"to\" point.".to_string()))
            .and_then(|to_str| {
                to_str.parse().map_err(|parse_err| {
                    ParseLineError(format!("could not parse \"to\" point.\n  {}", parse_err))
                })
            })?;
        Ok(Line { from, to })
    }
}

impl Line {
    pub fn trace(&self) -> Trace {
        Trace::new(self.from.clone(), self.to.clone())
    }
}
