mod solve;
mod states;
use std::{collections::BTreeMap, fmt::Display, iter::from_generator};

pub struct KnightsTour {
    size: (usize, usize),
    start: (usize, usize),
    back_to_start: bool,
}

#[derive(Clone)]
pub struct KnightsTourState {
    size_x: isize,
    size_y: isize,
    current_x: isize,
    current_y: isize,
    back_to_start: bool,
    visited: BTreeMap<(isize, isize), bool>,
    path: Vec<(isize, isize)>,
}

impl KnightsTour {
    pub fn new(width: usize, height: usize) -> Self {
        KnightsTour { size: (width, height), start: (0, 0), back_to_start: false }
    }
    pub fn with_start(mut self, x: usize, y: usize) -> Self {
        let x = if x < self.size.0 { x } else { self.size.0 - 1 };
        let y = if y < self.size.1 { y } else { self.size.1 - 1 };
        self.start = (x, y);
        self
    }
    pub fn with_back_to_start(mut self, back_to_start: bool) -> Self {
        self.back_to_start = back_to_start;
        self
    }
}
