use super::*;

pub struct KingTour {
    size: (usize, usize),
    start: (usize, usize),
    back_to_start: bool,
}

impl KingTour {
    pub fn initial_state(&self) -> ChessTourState {
        let size_x = self.size.0 as isize;
        let size_y = self.size.1 as isize;
        let current_x = self.start.0 as isize;
        let current_y = self.start.1 as isize;
        let mut state = ChessTourState {
            size_x,
            size_y,
            current_x,
            current_y,
            back_to_start: self.back_to_start,
            visited: Default::default(),
            path: vec![],
            available_moves: vec![(-2, -1), (-2, 1), (-1, -2), (-1, 2), (1, -2), (1, 2), (2, -1), (2, 1)],
        };
        state.initialize();
        state
    }
}

impl KingTour {
    pub fn new(width: usize, height: usize) -> Self {
        KingTour { size: (width, height), start: (0, 0), back_to_start: false }
    }
    pub fn with_start(mut self, x: usize, y: usize) -> Self {
        let x = if x < self.size.0 { x } else { self.size.0 - 1 };
        let y = if y < self.size.1 { y } else { self.size.1 - 1 };
        self.start = (x, y);
        self
    }

    pub fn walk(mut self, back_to_start: bool) -> Self {
        self.back_to_start = back_to_start;
        self
    }
}
