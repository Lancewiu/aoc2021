use std::convert;
use std::error;
use std::fmt;

#[derive(Debug)]
pub struct ParseDrawError(String);

impl fmt::Display for ParseDrawError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "error parsing card drawer ({})", self.0)
    }
}

impl error::Error for ParseDrawError {}

impl convert::From<&str> for ParseDrawError {
    fn from(s: &str) -> Self {
        ParseDrawError(format!("{}", s))
    }
}
