mod kings;
mod knights;
mod solve_walk;
mod states;

mod solve_tour;

pub use self::knights::KnightsTour;
use crate::{utils::format_point, SvgRender};
use std::{
    collections::BTreeMap,
    fmt::{Debug, Display, Formatter},
    iter::from_generator,
};

/// The state of the chess tour, which is a chess walk with a back-to-start constraint
#[derive(Clone)]
pub struct ChessTourState {
    size_x: isize,
    size_y: isize,
    current_x: isize,
    current_y: isize,
    back_to_start: bool,
    visited: BTreeMap<(isize, isize), bool>,
    path: Vec<(isize, isize)>,
    available_moves: Vec<(isize, isize)>,
}

impl ChessTourState {
    pub fn new(width: usize, height: usize, x: usize, y: usize) -> Self {
        let mut state = ChessTourState {
            size_x: width as isize,
            size_y: height as isize,
            current_x: x as isize,
            current_y: y as isize,
            back_to_start: true,
            visited: Default::default(),
            path: vec![],
            available_moves: vec![],
        };
        state.initialize();
        state
    }
    pub fn with_start(mut self) -> Self {
        let x = if x < self.size_x as usize { x } else { self.size_x as usize - 1 };
        let y = if y < self.size_y as usize { y } else { self.size_y as usize - 1 };
        self.current_x = x as isize;
        self.current_y = y as isize;
        self.initialize();
        self
    }
    pub fn with_moves(mut self, moves: Vec<(isize, isize)>) -> Self {
        self.available_moves = moves;
        self
    }
    pub fn with_back_to_start(mut self, back_to_start: bool) -> Self {
        self.back_to_start = back_to_start;
        self
    }
    fn initialize(&mut self) {
        self.visited.clear();
        self.visited.insert((self.current_x, self.current_y), true);
        self.path.clear();
        self.path.push((self.current_x, self.current_y));
    }
    pub fn count(&self) -> usize {
        (self.size_x * self.size_y) as usize
    }
}
