use super::*;
use rand::prelude::SliceRandom;

impl ChessTourState {
    pub fn get_visited(&self, x: isize, y: isize) -> bool {
        *self.visited.get(&(x, y)).unwrap_or(&false)
    }
    pub fn set_visited(&mut self, x: isize, y: isize, visited: bool) {
        self.visited.insert((x, y), visited);
    }
    pub fn go_grid(&mut self, x: isize, y: isize) {
        self.set_visited(x, y, true);
        self.current_x = x;
        self.current_y = y;
        self.path.push((x, y));
    }
    pub fn go_back(&mut self) {
        if let Some((x, y)) = self.path.pop() {
            self.set_visited(self.current_x, self.current_y, false);
            self.current_x = x;
            self.current_y = y;
        }
    }
    pub fn must_back_to_start(&self) -> bool {
        self.back_to_start && self.path.len() == self.count()
    }
    pub fn is_traversed(&self) -> bool {
        !self.back_to_start && self.path.len() == self.count()
    }
    pub fn is_traversed_back(&self) -> bool {
        self.back_to_start && self.path.len() == self.count() + 1 // && self.path[0] == (self.current_x, self.current_y)
    }
}

impl ChessTourState {
    fn possible_moves(&self) -> Vec<(isize, isize)> {
        self.available_moves
            .iter()
            .filter_map(|(dx, dy)| {
                let x = self.current_x + dx;
                let y = self.current_y + dy;
                if x < 0 || y < 0 || x >= self.size_x || y >= self.size_y {
                    return None;
                }
                if self.must_back_to_start() {
                    // println!("must_back_to_start: {} {} {} {}", x, y, self.path[0].0, self.path[0].1);
                    if (x, y) != self.path[0] {
                        return None;
                    }
                }
                else if self.get_visited(x, y) {
                    return None;
                }
                Some((x, y))
            })
            .collect()
    }
    pub fn backtrack(self) -> impl Iterator<Item = Self> {
        let mut stack = vec![self];
        from_generator(move || {
            while let Some(mut state) = stack.pop() {
                if state.is_traversed() {
                    yield state;
                    continue;
                }
                if state.is_traversed_back() {
                    yield state;
                    continue;
                }
                for (x, y) in state.possible_moves() {
                    state.go_grid(x, y);
                    stack.push(state.clone());
                    state.go_back();
                }
            }
        })
    }
}

impl ChessTourState {
    pub fn warnsdorff_rule(self) -> impl Iterator<Item = Self> {
        let mut state = self;
        let mut rng = rand::thread_rng();
        while !state.is_traversed() {
            let moves = state.possible_moves();
            if moves.is_empty() {
                break;
            }
            let mut moves_scores: Vec<(isize, isize, usize)> = moves
                .iter()
                .map(|(x, y)| {
                    (
                        *x,
                        *y,
                        state
                            .available_moves
                            .iter()
                            .filter(|(dx, dy)| {
                                let nx = x + dx;
                                let ny = y + dy;
                                if state.must_back_to_start() {
                                    if (nx, ny) == state.path[0] {
                                        return true;
                                    }
                                    return false;
                                }
                                nx >= 0 && ny >= 0 && nx < state.size_x && ny < state.size_y && !state.get_visited(nx, ny)
                            })
                            .count(),
                    )
                })
                .collect();
            moves_scores.sort_by_key(|(_, _, score)| *score);
            let (_, _, min_score) = moves_scores[0];
            let best_moves: Vec<(isize, isize)> =
                moves_scores.into_iter().filter(|(_, _, score)| *score == min_score).map(|(x, y, _)| (x, y)).collect();
            let (x, y) = *best_moves.choose(&mut rng).unwrap();
            state.go_grid(x, y);
        }
        if state.is_traversed_back() {
            state.go_back();
        }
        std::iter::once(state)
    }
}
