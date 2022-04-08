use super::*;
use std::{iter, iter::from_generator};

const KNIGHTS_MOVES: &'static [(isize, isize)] = &[(-2, -1), (-2, 1), (-1, -2), (-1, 2), (1, -2), (1, 2), (2, -1), (2, 1)];

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
    pub fn get_state(&self) -> KnightsTourState {
        let size_x = self.size.0 as isize;
        let size_y = self.size.1 as isize;
        let current_x = self.start.0 as isize;
        let current_y = self.start.1 as isize;
        let mut state = KnightsTourState {
            size_x,
            size_y,
            current_x,
            current_y,
            back_to_start: self.back_to_start,
            visited: Default::default(),
            path: vec![],
        };
        state.initialize();
        state
    }
}

impl KnightsTourState {
    pub fn initialize(&mut self) {
        self.visited.insert((self.current_x, self.current_y), true);
        self.path.push((self.current_x, self.current_y));
    }
    fn count(&self) -> usize {
        (self.size_x * self.size_y) as usize
    }
    pub fn get_visited(&self, x: isize, y: isize) -> bool {
        *self.visited.get(&(x, y)).unwrap_or(&false)
    }
    pub fn set_visited(&mut self, x: isize, y: isize, visited: bool) {
        self.visited.insert((x, y), visited);
    }
}

impl KnightsTourState {
    fn possible_moves(&self) -> Vec<(isize, isize)> {
        KNIGHTS_MOVES
            .iter()
            .filter_map(|(dx, dy)| {
                let x = self.current_x + dx;
                let y = self.current_y + dy;
                if x < 0 || x >= self.size_x || y < 0 || y >= self.size_y {
                    return None;
                }
                if self.get_visited(x, y) {
                    return None;
                }
                Some((x, y))
            })
            .collect()
    }
}

impl<'i> IntoIterator for &'i KnightsTour {
    type Item = Vec<(usize, usize)>;
    type IntoIter = impl Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut stack: Vec<KnightsTourState> = vec![self.get_state()];
        from_generator(move || {
            while let Some(mut state) = stack.pop() {
                if state.path.len() == state.count() {
                    if state.back_to_start {
                        if state.possible_moves().contains(&(state.current_x - state.size_x, state.current_y - state.size_y)) {
                            state.path.push((state.current_x - state.size_x, state.current_y - state.size_y));
                            yield state.path.iter().map(|(x, y)| (*x as usize, *y as usize)).collect();
                            state.path.pop();
                        }
                    }
                    else {
                        yield state.path.iter().map(|(x, y)| (*x as usize, *y as usize)).collect();
                    }
                }
                else {
                    for (x, y) in state.possible_moves() {
                        state.set_visited(x, y, true);
                        state.path.push((x, y));
                        state.current_x = x;
                        state.current_y = y;
                        stack.push(state.clone());
                        state.set_visited(x, y, false);
                        state.path.pop();
                        state.current_x = state.path.last().unwrap().0;
                        state.current_y = state.path.last().unwrap().1;
                    }
                }
            }
        })
    }
}

#[test]
fn test_knights_tour() {
    let mut state = KnightsTour { size: (5, 5), start: (0, 0), back_to_start: true };
    for i in state.into_iter().take(2) {
        println!("{:?}", i);
    }
}
