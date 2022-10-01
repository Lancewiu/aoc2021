use crate::matrix::Matrix;

use std::iter;
use std::ops;

pub struct Map {
    map: Matrix,
}

impl Map {
    pub fn from_matrix(map: Matrix) -> Self {
        Map { map }
    }

    pub fn low_points(&self) -> LowPointIter {
        LowPointIter {
            map: self,
            current_point: (0, 0),
        }
    }
}

impl ops::Deref for Map {
    type Target = Matrix;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

pub struct LowPointIter<'m> {
    map: &'m Map,
    current_point: (usize, usize),
}

impl<'m> LowPointIter<'m> {
    fn is_current_point_low(&self) -> bool {
        let (width, height) = self.map.get_dimensions();
        let (x, y) = self.current_point;
        let current_height = self.map[self.current_point];

        (0 == x || current_height < self.map[(x - 1, y)])
            && (0 == y || current_height < self.map[(x, y - 1)])
            && (width == x + 1 || current_height < self.map[(x + 1, y)])
            && (height == y + 1 || current_height < self.map[(x, y + 1)])
    }

    fn shift_current_point(&mut self) -> Option<(usize, usize)> {
        let (mut i_w, mut i_h) = self.current_point;
        if i_w + 1 == self.map.get_width() {
            if i_h + 1 == self.map.get_height() {
                self.current_point = self.map.get_dimensions();
                return None;
            }
            i_w = 0;
            i_h += 1;
        } else {
            i_w += 1;
        }
        self.current_point = (i_w, i_h);
        Some(self.current_point)
    }

    fn is_done(&self) -> bool {
        self.current_point == self.map.get_dimensions()
    }
}

impl<'m> iter::Iterator for LowPointIter<'m> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_done() {
            return None;
        }
        while !self.is_current_point_low() {
            self.shift_current_point()?;
        }
        let low_point = self.current_point;
        self.shift_current_point();
        Some(low_point)
    }
}

pub fn find_sizes(height_map: &Map) -> Vec<usize> {
    let (map_width, map_height) = height_map.get_dimensions();
    let mut sizes: Vec<usize> = Vec::new();

    for (x, y) in height_map.low_points() {
        let mut next_points = vec![(x, y)];
        let mut visited: Vec<(usize, usize)> = Vec::new();
        while let Some((x, y)) = next_points.pop() {
            let height = height_map[(x, y)];
            let mut candidates: Vec<(usize, usize)> = Vec::with_capacity(4);
            if 0 < y {
                candidates.push((x, y - 1));
            }
            if 0 < x {
                candidates.push((x - 1, y));
            }
            if map_width > x + 1 {
                candidates.push((x + 1, y));
            }
            if map_height > y + 1 {
                candidates.push((x, y + 1));
            }

            candidates = candidates
                .into_iter()
                .filter(|p| {
                    const MAX_HEIGHT: u8 = 9;
                    let value = height_map[p];
                    height < value
                        && MAX_HEIGHT != value
                        && !visited.iter().any(|visited_p| visited_p == p)
                        && !next_points.iter().any(|candid_p| candid_p == p)
                })
                .collect();

            next_points.append(&mut candidates);
            visited.push((x, y));
        }

        sizes.push(visited.len());
    }

    sizes
}
