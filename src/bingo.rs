use std::convert;
use std::error;
use std::fmt;

#[derive(Debug)]
pub struct TryFromSliceError(String);

impl fmt::Display for TryFromSliceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "error generating board from slice ({})", self.0)
    }
}

impl error::Error for TryFromSliceError {}

pub struct Board {
    buf: [u32; 25],
    is_marked_buf: [bool; 25],
}

impl convert::TryFrom<&[u32]> for Board {
    type Error = TryFromSliceError;

    fn try_from(slice: &[u32]) -> Result<Self, Self::Error> {
        let buf = slice
            .get(..25)
            .ok_or(TryFromSliceError(format!(
                "could not convert from slice. Len must be >= 25, received len {}",
                slice.len()
            )))
            .and_then(|slice| {
                <[u32; 25]>::try_from(slice)
                    .map_err(|from_slice_err| TryFromSliceError(from_slice_err.to_string()))
            })?;

        Ok(Board {
            buf,
            is_marked_buf: [false; 25],
        })
    }
}

impl Board {
    pub fn mark(&mut self, value: u32) {
        self.buf
            .iter()
            .copied()
            .zip(self.is_marked_buf.iter_mut())
            .filter(|(v, _)| *v == value)
            .for_each(|(_, is_marked)| {
                *is_marked = true;
            });
    }

    pub fn score(&self) -> u32 {
        self.buf
            .iter()
            .copied()
            .zip(self.is_marked_buf.iter().copied())
            .map(|(value, is_marked)| if is_marked { 0u32 } else { value })
            .sum()
    }

    pub fn has_won(&self) -> bool {
        // diagonal
        let is_row_win = self
            .is_marked_buf
            .chunks(5)
            .any(|row| row.iter().copied().all(|is_marked| is_marked));

        #[rustfmt::skip]
        let is_col_win = [
            0usize, 5, 10, 15, 20,
            1,      6, 11, 16, 21,
            2,      7, 12, 17, 22,
            3,      8, 13, 18, 23,
            4,      9, 14, 19, 24,
        ]
        .chunks(5)
        .any(|i_col| i_col.iter().copied().all(|i| self.is_marked_buf[i]));
        is_row_win || is_col_win
    }
}
