#![feature(generators)]

mod errors;

use std::io::Read;
pub use errors::{Error, Result};
// mod knights;
use gen_iter::GenIter;

/// A vertex in graph
pub trait Hamiltonian
    where Self: Sized + Clone
{
    fn count(&self) -> usize;
    fn current(&self) -> usize;
    fn get_visited(&self, node: usize) -> bool;
    fn set_visited(&mut self, node: usize);
    /// all valid moves
    fn possible_moves(&self, start: usize) -> Vec<usize>;
}

fn backtracking<T: Hamiltonian>(graph: &mut T) -> impl Iterator<Item=Vec<usize>> + '_ {
    let mut stack = vec![(vec![graph.current()], graph.current())];
    let mut paths: Vec<Vec<usize>> = vec![];
    GenIter(move || {
        while let Some((path, current)) = stack.pop() {
            if path.len() == graph.count() {
                yield path;
            } else {
                for next in graph.possible_moves(current) {
                    if !graph.get_visited(next) {
                        let mut new_path = path.clone();
                        new_path.push(next);
                        let mut new_self = graph.clone();
                        new_self.set_visited(next);
                        stack.push((new_path, next));
                    }
                }
            }
        }
    })
}

#[derive(Clone)]
pub struct KnightsTourState {
    size_x: isize,
    size_y: isize,
    current_x: isize,
    current_y: isize,
    visited: Vec<bool>,
    path: Vec<usize>,
}


const KNIGHTS_MOVES: &'static [(isize, isize)] = &[(-2, -1), (-2, 1), (-1, -2), (-1, 2), (1, -2), (1, 2), (2, -1), (2, 1)];

impl KnightsTourState {
    pub fn get_point(&self, index: usize) -> (isize, isize) {
        (index as isize / self.size_y, index as isize % self.size_y)
    }
    pub fn get_index(&self, x: isize, y: isize) -> usize {
        (x * self.size_y + y) as usize
    }
}

impl Hamiltonian for KnightsTourState {
    fn count(&self) -> usize {
        (self.size_x * self.size_y) as usize
    }

    fn current(&self) -> usize {
        self.get_index(self.current_x, self.current_y)
    }

    fn get_visited(&self, node: usize) -> bool {
        self.visited[node]
    }

    fn set_visited(&mut self, node: usize) {
        self.visited[node] = true;
    }

    fn possible_moves(&self, start: usize) -> Vec<usize> {
        let mut possible_moves: Vec<usize> = Vec::new();
        for (dx, dy) in KNIGHTS_MOVES {
            let p = self.get_point(start);
            let x = p.0 + dx;
            let y = p.1 + dy;
            let valid = x >= 0
                && x < self.size_x
                && y >= 0
                && y < self.size_y
                && !self.visited[self.get_index(x, y)];
            if valid
            {
                possible_moves.push(self.get_index(x, y));
            }
        }
        possible_moves
    }
}


#[test]
fn test_knights_tour() {
    let mut state = KnightsTourState {
        size_x: 5,
        size_y: 5,
        current_x: 1,
        current_y: 1,
        visited: vec![false; 25],
        path: vec![0],
    };
    for i in backtracking(&mut state).take(100) {
        println!("{:?}", i);
    }
}