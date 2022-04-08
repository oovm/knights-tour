use std::iter;
use std::iter::from_generator;
use hamiltonian::backtracking;
use super::*;
use gen_iter::GenIter;

const KNIGHTS_MOVES: &'static [(isize, isize)] = &[(-2, -1), (-2, 1), (-1, -2), (-1, 2), (1, -2), (1, 2), (2, -1), (2, 1)];

impl KnightsTourState {
    pub fn initialize(&mut self) {
        self.visited = vec![false; self.count()];
        let index = self.get_index(self.current_x, self.current_y);
        self.visited[index] = true;
        self.stack = vec![(vec![self.current()], self.current())]
    }
    pub fn get_point(&self, index: usize) -> (isize, isize) {
        (index as isize / self.size_y, index as isize % self.size_y)
    }
    pub fn get_index(&self, x: isize, y: isize) -> usize {
        (x * self.size_y + y) as usize
    }
}

impl IntoIterator for KnightsTour {
    type Item = Vec<(usize, usize)>;
    type IntoIter = impl Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut state = KnightsTourState {
            size_x: self.size.0 as isize,
            size_y: self.size.0 as isize,
            current_x: self.start.0 as isize,
            current_y: self.start.1 as isize,
            visited: vec![],
            stack: vec![],
        };
        state.initialize();
        from_generator(move || {
            while let Some((path, current)) = state.stack.pop() {
                if path.len() == state.count() {
                    yield path.iter().map(|&p| {
                        let p = state.get_point(p);
                        (p.0 as usize, p.1 as usize)
                    }).collect::<Vec<_>>();
                } else {
                    for next in state.possible_moves(current) {
                        if !state.get_visited(next) {
                            let mut new_path = path.clone();
                            new_path.push(next);
                            let mut new_self = state.clone();
                            new_self.set_visited(next, true);
                            state.stack.push((new_path, next));
                        }
                    }
                }
            }
        })
    }
}

pub struct KnightsTourIter {
    state: KnightsTourState,
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

    fn set_visited(&mut self, node: usize, state: bool) {
        self.visited[node] = state;
    }

    fn walk_to(&mut self, node: usize) {
        let p = self.get_point(node);
        self.current_x = p.0;
        self.current_y = p.1;
        self.visited[node] = true;
        self.stack.push((vec![node], node));
    }

    fn back_to(&mut self, node: usize) {
        let p = self.get_point(node);
        self.current_x = p.0;
        self.current_y = p.1;
        self.visited[node] = false;
        self.stack.pop();
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
    let mut state = KnightsTour {
        size: (5, 5),
        start: (0, 0),
    };
    for i in state.into_iter().take(100) {
        println!("{:?}", i);
    }
}
