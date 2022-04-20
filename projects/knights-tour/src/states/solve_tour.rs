use super::*;
use itertools::Itertools;
use rand::{prelude::SliceRandom, rngs::SmallRng, SeedableRng};

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
    pub fn available_moves(&self) -> Vec<(isize, isize)> {
        let mut moves = vec![];
        for (x, y) in &self.available_moves {
            let x = self.current_x + x;
            let y = self.current_y + y;
            if x < 0 || y < 0 || x >= self.size_x || y >= self.size_y {
                continue;
            }
            if self.must_back_to_start() {
                if (x, y) != self.path[0] {
                    continue;
                }
            }
            else if self.get_visited(x, y) {
                continue;
            }
            moves.push((x, y));
        }
        moves
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
                for (x, y) in state.available_moves() {
                    state.go_grid(x, y);
                    stack.push(state.clone());
                    state.go_back();
                }
            }
        })
    }
}

impl ChessTourState {
    pub fn best_moves(&self) -> Vec<(isize, isize)> {
        let mut moves = vec![];
        let mut min_score = usize::MAX;
        for (x, y) in self.available_moves() {
            let mut score = 0;
            for (dx, dy) in self.available_moves.iter() {
                let nx = x + dx;
                let ny = y + dy;
                if nx >= 0 && ny >= 0 && nx < self.size_x && ny < self.size_y && !self.get_visited(nx, ny) {
                    score += 1;
                }
            }
            if score < min_score {
                min_score = score;
            }
            moves.push((x, y, score));
        }
        moves.into_iter().filter(|(_, _, score)| *score == min_score).map(|(x, y, _)| (x, y)).collect_vec()
    }

    pub fn warnsdorff_rule(&self) -> impl Iterator<Item = Self> {
        let state = self.clone();
        let mut rng = SmallRng::from_entropy();
        from_generator(move || {
            'outer: loop {
                let mut state = state.clone();
                while !state.is_traversed_back() {
                    let best_moves = state.best_moves();
                    match best_moves.choose(&mut rng) {
                        Some((x, y)) => state.go_grid(*x, *y),
                        // 最佳走法为空，说明已经走到死胡同，回溯也没用, 直接推倒重来
                        None => continue 'outer,
                    }
                }
                yield state;
            }
        })
    }
}
