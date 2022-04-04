mod errors;

pub use errors::{Error, Result};
// mod knights;


/// A vertex in graph
pub trait Hamiltonian
    where Self: Sized
{

    /// is node start connect to node end
    fn is_safe(&self, start: usize, end: usize);
    /// all valid moves
    fn valid_move(&self, start: usize) -> Vec<usize> {

    }
}


pub struct KnightsTourState {
    current_x: isize,
    current_y: isize,
    visited: Array2<bool>,
    path: Vec<(isize, isize)>,
    /// if true, the path will back to start point
    back_to_start: bool,
}