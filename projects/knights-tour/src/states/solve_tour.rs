use super::*;

impl ChessTourState {
    pub fn is_travelled_back(&self) -> bool {
        self.wrapped.path[0] == (self.wrapped.current_x, self.wrapped.current_y)
    }
    pub fn must_back(&self) -> bool {
        self.wrapped.path.len() == self.wrapped.count()
    }
}

impl ChessTourState {
    fn possible_moves(&self) -> Vec<(isize, isize)> {
        self.wrapped
            .available_moves
            .iter()
            .filter_map(|(dx, dy)| {
                let x = self.wrapped.current_x + dx;
                let y = self.wrapped.current_y + dy;
                if x < 0 || y < 0 || x >= self.wrapped.size_x || y >= self.wrapped.size_y {
                    return None;
                }
                else if self.must_back() {
                    if (x, y) != self.wrapped.path[0] {
                        return None;
                    }
                }
                else if self.wrapped.get_visited(x, y) {
                    return None;
                }
                Some((x, y))
            })
            .collect()
    }
    pub fn solving(&self) -> impl Iterator<Item = ChessPathState> {
        let mut stack = vec![self.clone()];
        from_generator(move || {
            while let Some(mut state) = stack.pop() {
                if state.is_travelled_back() {
                    yield state;
                    continue;
                }
                for (x, y) in state.possible_moves() {
                    state.wrapped.go_grid(x, y);
                    stack.push(state.clone());
                    state.wrapped.go_back();
                }
            }
        })
    }
}
