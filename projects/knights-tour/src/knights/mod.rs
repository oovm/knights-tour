mod solve;
use hamiltonian::Hamiltonian;
pub struct KnightsTour {
    size: (usize, usize),
    start: (usize, usize),
}

impl KnightsTour {
    pub fn new(width: usize, height: usize) -> Self {
        KnightsTour { size: (width, height), start: (0, 0) }
    }
    pub fn with_start(mut self, x: usize, y: usize) -> Self {
        let x = if x < self.size.0 { x } else { self.size.0 - 1 };
        let y = if y < self.size.1 { y } else { self.size.1 - 1 };
        self.start = (x, y);
        self
    }
}



#[derive(Clone)]
pub struct KnightsTourState {
    size_x: isize,
    size_y: isize,
    current_x: isize,
    current_y: isize,
    visited: Vec<bool>,
    stack: Vec<(Vec<usize>, usize)>,
}