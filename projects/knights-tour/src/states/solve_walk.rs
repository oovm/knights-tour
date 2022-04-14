use super::*;

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
        self.back_to_start && self.path.len() == self.count() - 1
    }
    pub fn is_traversed(&self) -> bool {
        self.path.len() == self.count()
    }
    pub fn is_traversed_back(&self) -> bool {
        self.is_traversed() && self.path[0] == (self.current_x, self.current_y)
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
}

impl<'i> IntoIterator for &'i KnightsTour {
    type Item = ChessTourState;
    type IntoIter = impl Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut stack = vec![self.initial_state()];
        from_generator(move || {
            while let Some(mut state) = stack.pop() {
                match state.back_to_start {
                    true if state.is_traversed_back() => {
                        yield state;
                        continue;
                    }
                    false if state.is_traversed() => {
                        yield state;
                        continue;
                    }
                    _ => {}
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
