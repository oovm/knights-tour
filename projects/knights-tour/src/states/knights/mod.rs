use super::*;

pub struct KnightsTour {
    size: (usize, usize),
    start: (usize, usize),
    back_to_start: bool,
}

impl KnightsTour {
    pub fn start_walk(&self) -> ChessPathState {
        let size_x = self.size.0 as isize;
        let size_y = self.size.1 as isize;
        let current_x = self.start.0 as isize;
        let current_y = self.start.1 as isize;
        let mut state = ChessPathState {
            size_x,
            size_y,
            current_x,
            current_y,
            visited: Default::default(),
            path: vec![],
            available_moves: vec![(-2, -1), (-2, 1), (-1, -2), (-1, 2), (1, -2), (1, 2), (2, -1), (2, 1)],
        };
        state.initialize();
        state
    }
    pub fn start_tour(&self) -> ChessTourState {
        ChessTourState { wrapped: self.start_walk() }
    }
}

impl KnightsTour {
    pub fn new(width: usize, height: usize) -> Self {
        KnightsTour { size: (width, height), start: (0, 0), back_to_start: false }
    }
    pub fn with_start(mut self, x: usize, y: usize) -> Self {
        let x = if x < self.size.0 { x } else { self.size.0 - 1 };
        let y = if y < self.size.1 { y } else { self.size.1 - 1 };
        self.start = (x, y);
        self
    }
    pub fn with_back_to_start(mut self, back_to_start: bool) -> Self {
        self.back_to_start = back_to_start;
        self
    }
}

impl IntoIterator for KnightsTour {
    type Item = ChessPathState;
    type IntoIter = Box<dyn Iterator<Item = Self::Item>>;
    fn into_iter(self) -> Self::IntoIter {
        match self.back_to_start {
            true => Box::new(self.start_tour().solving()),
            false => Box::new(self.start_walk().solving()),
        }
    }
}
