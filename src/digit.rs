use std::convert;

#[derive(Clone)]
pub struct Digit {
    segments: Vec<char>,
}

impl convert::From<&str> for Digit {
    fn from(s: &str) -> Self {
        Digit {
            segments: s.chars().collect(),
        }
    }
}

impl Digit {
    pub fn contains(&self, other: &Digit) -> bool {
        other
            .segments
            .iter()
            .all(|other_seg| self.segments.iter().any(|seg| other_seg == seg))
    }

    pub fn diff(&self, other: &Digit) -> String {
        self.segments
            .iter()
            .copied()
            .filter(|c| !other.segments.iter().any(|other_c| other_c == c))
            .collect()
    }
}
