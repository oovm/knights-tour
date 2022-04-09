use super::*;

const KNIGHTS_MOVES: &'static [(isize, isize)] = &[(-2, -1), (-2, 1), (-1, -2), (-1, 2), (1, -2), (1, 2), (2, -1), (2, 1)];

impl KnightsTour {
    pub fn initial_state(&self) -> KnightsTourState {
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
    type Item = KnightsTourState;
    type IntoIter = impl Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut stack = vec![self.initial_state()];
        from_generator(move || {
            while let Some(mut state) = stack.pop() {
                if state.path.len() == state.count() {
                    match state.back_to_start {
                        true => {
                            let (x, y) = self.start;
                            if state.current_x == x as isize && state.current_y == y as isize {
                                yield state;
                            }
                        }
                        false => {
                            yield state;
                        }
                    }
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
