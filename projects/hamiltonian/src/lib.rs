mod errors;

pub use errors::{Error, Result};
// mod knights;


/// A vertex in graph
pub trait Hamiltonian
    where Self: Sized
{
    fn current(&self) -> usize;
    /// is node start connect to node end
    fn is_safe(&self, start: usize, end: usize);
    /// all valid moves
    fn valid_move(&self, start: usize) -> Vec<usize>;
}


pub struct KnightsTourState {
    size: (usize, usize),
    current: usize,
    visited: Vec<bool>,
    path: Vec<usize>,
}

impl Hamiltonian for KnightsTourState {
    fn current(&self) -> usize {
        self.current
    }

    fn is_safe(&self, start: usize, end: usize) {

    }

    fn valid_move(&self, start: usize) -> Vec<usize> {
        todo!()
    }
}