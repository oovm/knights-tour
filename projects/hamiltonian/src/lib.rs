#![feature(generators)]

use std::io::Read;

// mod states;
use gen_iter::GenIter;

/// A vertex in graph
pub trait Hamiltonian
where
    Self: Sized + Clone,
{
    fn count(&self) -> usize;
    fn current(&self) -> usize;
    /// get visited
    fn get_visited(&self, node: usize) -> bool;
    /// set visited
    fn set_visited(&mut self, node: usize, state: bool);
    /// walk to node
    fn walk_to(&mut self, node: usize);
    /// back to node
    fn back_to(&mut self, node: usize);
    /// all valid moves
    fn possible_moves(&self, start: usize) -> Vec<usize>;
}

pub fn backtracking<T: Hamiltonian>(mut graph: T) -> impl Iterator<Item = Vec<usize>> {
    let mut stack: Vec<(Vec<usize>, usize)> = vec![(vec![graph.current()], graph.current())];
    // let mut paths: Vec<Vec<usize>> = vec![];
    GenIter(move || {
        while let Some((path, current)) = stack.pop() {
            if path.len() == graph.count() {
                yield path;
            }
            else {
                for next in graph.possible_moves(current) {
                    if !graph.get_visited(next) {
                        let mut new_path = path.clone();
                        new_path.push(next);
                        let mut new_self = graph.clone();
                        new_self.set_visited(next, true);
                        stack.push((new_path, next));
                    }
                }
            }
        }
    })
}
