use super::*;
use crate::{utils::format_point, SvgRender};
use std::fmt::{Debug, Formatter};
use svg::{
    node::element::{path::Data, Circle, Line, Rectangle, Text},
    Document,
};

impl Debug for KnightsTourState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let start = self.path.first().unwrap_or(&(0, 0));
        f.debug_struct("KnightsTourState")
            .field("size", &format_point(self.size_x, self.size_y))
            .field("start", &format_point(start.0, start.1))
            .field("end", &format_point(self.current_x, self.current_y))
            .field("back_to_start", &self.back_to_start)
            .field("path", &self.path)
            .finish()
    }
}

impl Display for KnightsTourState {
    // write a board, number is the step
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.size_y {
            for x in 0..self.size_x {
                let visited = self.visited.get(&(x, y)).unwrap_or(&false);
                let current = self.current_x == x && self.current_y == y;
                let step = self.path.iter().position(|&p| p == (x, y)).map(|i| i + 1);
                let step_str = match step {
                    Some(s) => format!("{:2}", s),
                    None => "  ".to_string(),
                };
                let symbol = if *visited {
                    "X"
                }
                else if current {
                    "K"
                }
                else {
                    "."
                };
                write!(f, "{} ", symbol)?;
                write!(f, "{} ", step_str)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl KnightsTourState {
    /// return a iterator of steps, each step is a tuple of two points
    pub fn steps(&self) -> impl Iterator<Item = ((usize, usize), (usize, usize))> + '_ {
        self.path.windows(2).map(|w| ((w[0].0 as usize, w[0].1 as usize), (w[1].0 as usize, w[1].1 as usize)))
    }
}

impl KnightsTourState {
    pub fn draw_svg(&self, render: &SvgRender) -> String {
        let mut board = render.document(self.size_x as f32, self.size_y as f32);
        // Draw the board squares
        for x in 0..self.size_x {
            for y in 0..self.size_y {
                board = board.add(render.draw_square(x as usize, y as usize));
            }
        }
        // Draw the path
        for ((x1, y1), (x2, y2)) in self.steps() {
            let line = render.draw_path();
            board = board.add(line);
        }

        // Draw the step numbers
        for (i, &(x, y)) in self.path.iter().enumerate() {
            let text = Text::new()
                .set("x", x * 50 + 25)
                .set("y", y * 50 + 35)
                .set("text-anchor", "middle")
                .set("font-size", 20)
                .set("fill", "#000000")
                .add(svg::node::Text::new((i + 1).to_string()));
            board = board.add(text);
        }

        board.to_string()
    }
}
