use std::boxed;
use std::error;
use std::fmt;
use std::iter;
use std::str;
use std::cmp;

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
    increment_call: boxed::Box<dyn Fn(&mut Point)>,
    is_done: bool,
}

impl iter::Iterator for Trace {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        eprintln!("cursor: {:?}, end: {:?}, is_done: {}", self.cursor, self.end, self.is_done);
        if self.is_done {
            return None;
        }

        let temp = self.cursor.clone();

        if self.end == self.cursor {
            self.is_done = true;
        } else {
            (self.increment_call)(&mut self.cursor);
        }
        Some(temp)
    }
}

impl Trace {
    fn new(start: Point, end: Point, increment_call: boxed::Box<dyn Fn(&mut Point)>) -> Self {
        Self {
            cursor: start,
            end,
            increment_call,
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
        let shift_by_x = match self.from.x.cmp(&self.to.x) {
            cmp::Ordering::Equal => boxed::Box::new(|x: &mut u32| {}),
            cmp::Ordering::Greater => boxed::Box::new(|x: &mut u32| { *x -= 1; }),
            cmp::Ordering::Less => boxed::Box::new(|x: &mut u32| { *x += 1; }),
        };
        let shift_by_y = if self.from.y < self.to.y {
            |y: &mut u32| {
                *y += 1;
            }
        } else {
            |y: &mut u32| {
                *y -= 1;
            }
        };

        if self.from.x == self.to.x {
            Trace::new(
                self.from.clone(),
                self.to.clone(),
                boxed::Box::new(move |p: &mut Point| shift_by_y(&mut p.x)),
            )
        } else if self.from.y == self.to.y {
            Trace::new(
                self.from.clone(),
                self.to.clone(),
                boxed::Box::new(move |p: &mut Point| shift_by_x(&mut p.y)),
            )
        } else {
            Trace::new(
                self.from.clone(),
                self.to.clone(),
                boxed::Box::new(move |p: &mut Point| {
                    shift_by_x(&mut p.x);
                    shift_by_y(&mut p.y);
                }),
            )
        }
    }
}
