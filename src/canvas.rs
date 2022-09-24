use crate::geometry;
use std::collections::hash_map;
use std::collections::HashMap;

pub struct Canvas {
    mapping: HashMap<geometry::Point, u32>,
}

impl Canvas {
    pub fn new() -> Self {
        Canvas {
            mapping: HashMap::new(),
        }
    }

    fn count_point(&mut self, point: geometry::Point) {
        self.mapping
            .entry(point)
            .and_modify(|count| {
                *count += 1;
            })
            .or_insert(1);
    }

    pub fn count_line(&mut self, line: &geometry::Line) {
        line.trace().into_iter().for_each(|p| self.count_point(p));
    }

    pub fn iter(&self) -> hash_map::Iter<'_, geometry::Point, u32> {
        self.mapping.iter()
    }
}
