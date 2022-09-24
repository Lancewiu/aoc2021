use std::cmp;
use std::error;
use std::fmt;
use std::str;

#[derive(Debug)]
pub struct ParsePointError(String);

impl fmt::Display for ParsePointError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed to parse point from string.\n  {}", self.0)
    }
}

impl error::Error for ParsePointError {}

#[derive(Eq, Hash, PartialEq)]
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
            .ok_or(ParsePointError(format!("missing x in \"x,y\"")))
            .and_then(|x_str| {
                x_str.trim().parse().map_err(|parse_err| {
                    ParsePointError(format!("failed to parse x.\n  {}", parse_err))
                })
            })?;
        let y = tokens
            .next()
            .ok_or(ParsePointError(format!("missing y in \"x,y\"")))
            .and_then(|y_str| {
                y_str.trim().parse().map_err(|parse_err| {
                    ParsePointError(format!("failed to parse y.\n  {}", parse_err))
                })
            })?;
        Ok(Point { x, y })
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
            .ok_or(ParseLineError(format!("could not find \"from\" point.")))
            .and_then(|from_str| {
                from_str.parse().map_err(|parse_err| {
                    ParseLineError(format!("could not parse \"from\" point.\n  {}", parse_err))
                })
            })?;
        tokens.next(); // -> symbol which is ignored
        let to = tokens
            .next()
            .ok_or(ParseLineError(format!("could not find \"to\" point.")))
            .and_then(|to_str| {
                to_str.parse().map_err(|parse_err| {
                    ParseLineError(format!("could not parse \"to\" point.\n  {}", parse_err))
                })
            })?;
        Ok(Line { from, to })
    }
}

impl Line {
    pub fn is_diagonal(&self) -> bool {
        self.from.x != self.to.x && self.from.y != self.to.y
    }

    pub fn trace(&self) -> Vec<Point> {
        if self.is_diagonal() {
            unimplemented!();
        }
        let is_vertical = self.from.x == self.to.x;
        let ((a, b), pivot_value) = if is_vertical {
            ((self.from.y, self.to.y), self.from.x)
        } else {
            ((self.from.x, self.to.x), self.from.y)
        };

        (cmp::min(a, b)..=cmp::max(a, b))
            .map(|value| {
                if is_vertical {
                    Point {
                        x: pivot_value,
                        y: value,
                    }
                } else {
                    Point {
                        x: value,
                        y: pivot_value,
                    }
                }
            })
            .collect()
    }
}
