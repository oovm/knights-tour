

pub struct KnightsTour {
    size: (usize, usize),
    start: (usize, usize),
}

impl KnightsTour {
    pub fn new(width: usize, height: usize) -> Self {
        KnightsTour { size: (width, height), start: (0, 0), back_to_start: true }
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
    type Item = Vec<(usize, usize)>;
    type IntoIter = KnightsTourState;

    fn into_iter(self) -> Self::IntoIter {
        let mut visited = Array2::default((self.size.0, self.size.1));
        visited[self.start] = true;
        let current_x = self.start.0 as isize;
        let current_y = self.start.1 as isize;
        let path = vec![(current_x, current_y)];
        KnightsTourState { current_x, current_y, visited, path, back_to_start: self.back_to_start }
    }
}

pub struct KnightsTourState {
    current_x: isize,
    current_y: isize,
    visited: Array2<bool>,
    path: Vec<(isize, isize)>,
    /// if true, the path will back to start point
    back_to_start: bool,
}

const KNIGHTS_MOVES: &'static [(isize, isize)] = &[(-2, -1), (-2, 1), (-1, -2), (-1, 2), (1, -2), (1, 2), (2, -1), (2, 1)];

impl KnightsTourState {
    pub fn valid_move(&self) -> Vec<(isize, isize)> {
        let mut possible_moves: Vec<(isize, isize)> = Vec::new();
        for (dx, dy) in KNIGHTS_MOVES {
            let x = self.current_x + dx;
            let y = self.current_y + dy;
            if x >= 0
                && x < self.visited.shape()[0] as isize
                && y >= 0
                && y < self.visited.shape()[1] as isize
                && !self.visited[(x as usize, y as usize)]
            {
                possible_moves.push((x, y));
            }
        }
        possible_moves
    }
}

impl Iterator for KnightsTourState {
    /// solution path
    type Item = Vec<(usize, usize)>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.visited.iter().all(|&visited| visited) {
            if self.back_to_start {
                if self.valid_move().contains(&(self.path[0].0, self.path[0].1)) {
                    self.path.push((self.path[0].0, self.path[0].1));
                    return Some(self.path.iter().map(|&(x, y)| (x as usize, y as usize)).collect());
                }
            }
            else {
                return Some(self.path.iter().map(|&(x, y)| (x as usize, y as usize)).collect());
            }
        }
        for (x, y) in self.valid_move() {
            self.visited[(x as usize, y as usize)] = true;
            self.path.push((x, y));
            self.current_x = x;
            self.current_y = y;
            if let Some(solution) = self.next() {
                return Some(solution);
            }
            self.visited[(x as usize, y as usize)] = false;
            self.path.pop();
        }
        None
    }
}

#[test]
fn test_knights_tour() {
    let tour = KnightsTour::new(6, 6).with_back_to_start(false);
    for sol in tour.into_iter().take(2) {
        println!("{:?}", sol);
    }
}
