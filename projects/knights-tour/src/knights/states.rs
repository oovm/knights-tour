use super::*;
use crate::utils::format_point;
use std::fmt::{Debug, Formatter};
use svg::{
    node::element::{path::Data, Circle, Line, Text},
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
    pub fn draw_svg(&self) -> String {
        let mut doc = Document::new().set("viewBox", (0, 0, self.size_x, self.size_y));
        let mut data = Data::new();
        for i in 0..self.size_x {
            for j in 0..self.size_y {
                let x = i as f64 + 0.5;
                let y = j as f64 + 0.5;
                let circle = Circle::new().set("cx", x).set("cy", y).set("r", 0.4).set("fill", "white");
                doc = doc.add(circle);
                if let Some(step) = self.path.iter().position(|&(a, b)| a == i && b == j) {
                    let text = Text::new()
                        .set("x", x)
                        .set("y", y)
                        .set("text-anchor", "middle")
                        .set("dominant-baseline", "central")
                        .set("font-size", 0.4)
                        .set("fill", "black")
                        .add(step.to_string());
                    doc = doc.add(text);
                }
                if let Some(&(prev_x, prev_y)) = self.path.get(self.path.len() - 2) {
                    if (prev_x, prev_y) == (i, j) {
                        data = data.move_to((prev_x as f64 + 0.5, prev_y as f64 + 0.5));
                        data = data.line_to((x, y));
                    }
                }
            }
        }
        let path = Line::new().set("stroke", "black").set("stroke-width", 0.1).set("fill", "none").set("d", data);
        doc = doc.add(path);
        doc.to_string()
    }
}
