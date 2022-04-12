use super::*;

impl ChessPathState {
    pub fn initialize(&mut self) {
        self.visited.insert((self.current_x, self.current_y), true);
        self.path.push((self.current_x, self.current_y));
    }
    pub fn count(&self) -> usize {
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
    pub fn is_traversed(&self) -> bool {
        self.path.len() == self.count()
    }
}

impl ChessPathState {
    fn possible_moves(&self) -> Vec<(isize, isize)> {
        self.available_moves
            .iter()
            .filter_map(|(dx, dy)| {
                let x = self.current_x + dx;
                let y = self.current_y + dy;
                if x < 0 || y < 0 || x >= self.size_x || y >= self.size_y {
                    return None;
                }
                else if self.get_visited(x, y) {
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
                if state.is_traversed() {
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
