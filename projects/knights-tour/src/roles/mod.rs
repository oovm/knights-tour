use crate::ChessTourState;

pub struct Chessboard {
    pub role: ChessRole,
    pub size: (usize, usize),
    pub start: (usize, usize),
    pub back_to_start: bool,
}

pub enum ChessRole {
    King,
    Knight,
    Pawn,
}

impl ChessRole {
    #[allow(unused_variables)]
    pub fn get_moves(&self, width: usize, height: usize) -> Vec<(isize, isize)> {
        match self {
            ChessRole::King => vec![(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)],
            ChessRole::Knight => vec![(-2, -1), (-2, 1), (-1, -2), (-1, 2), (1, -2), (1, 2), (2, -1), (2, 1)],
            ChessRole::Pawn => {
                vec![(-1, 0), (0, -1), (1, 0), (0, 1)]
            }
        }
    }
}

impl Chessboard {
    pub fn initial_state(&self) -> ChessTourState {
        ChessTourState::new(self.size.0, self.size.1, self.start.0, self.start.1)
            .with_back_to_start(self.back_to_start)
            .with_moves(self.role.get_moves(self.size.0, self.size.1))
    }
}

impl Chessboard {
    pub fn new(width: usize, height: usize) -> Self {
        Chessboard { role: ChessRole::Knight, size: (width, height), start: (0, 0), back_to_start: true }
    }
    pub fn with_start(mut self, x: usize, y: usize) -> Self {
        let x = if x < self.size.0 { x } else { self.size.0 - 1 };
        let y = if y < self.size.1 { y } else { self.size.1 - 1 };
        self.start = (x, y);
        self
    }
    pub fn with_role(mut self, role: ChessRole) -> Self {
        self.role = role;
        self
    }
    pub fn with_back_to_start(mut self, back_to_start: bool) -> Self {
        self.back_to_start = back_to_start;
        self
    }
    pub fn walk(mut self, back_to_start: bool) -> Self {
        self.back_to_start = back_to_start;
        self
    }
}

impl IntoIterator for Chessboard {
    type Item = ChessTourState;
    type IntoIter = impl Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.initial_state().backtrack()
    }
}
