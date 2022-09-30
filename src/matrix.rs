use std::error;
use std::fmt;
use std::ops;

#[derive(Debug)]
pub struct InvalidMatrix {}

impl fmt::Display for InvalidMatrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Matrix is invalid")
    }
}

impl error::Error for InvalidMatrix {}

pub struct Matrix {
    raw: Vec<u8>,
    width: usize,
}

impl ops::Index<(usize, usize)> for Matrix {
    type Output = u8;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        self.get(index)
            .unwrap_or_else(|| panic!("matrix index {:#?} out of bounds", index))
    }
}

impl Matrix {
    pub fn try_from_raw(raw: &[u8], width: usize) -> Result<Self, InvalidMatrix> {
        if 0 != raw.len() % width {
            return Err(InvalidMatrix {});
        }

        Ok(Matrix {
            raw: Vec::from(raw),
            width,
        })
    }

    pub fn get_height(&self) -> usize {
        if 0 == self.width {
            0
        } else {
            self.raw.len() / self.width
        }
    }

    pub fn get(&self, index: (usize, usize)) -> Option<&u8> {
        self.raw.get(index.0 + index.1 * self.width)
    }
}
