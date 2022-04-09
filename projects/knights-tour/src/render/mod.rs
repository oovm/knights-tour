use svg::{
    node::element::{Line, Rectangle},
    Document,
};

pub struct SvgRender {
    pub grid_size: f32,
    pub board_white: String,
    pub board_black: String,
    pub path_color: String,
    pub path_width: f32,
}

impl Default for SvgRender {
    fn default() -> Self {
        Self {
            grid_size: 50.0,
            board_white: "#f0f0f0".to_string(),
            board_black: "#c0c0c0".to_string(),
            path_color: "#0000ff".to_string(),
            path_width: 5.0,
        }
    }
}

impl SvgRender {
    pub fn document(&self, width: f32, height: f32) -> Document {
        Document::new().set("viewBox", (0, 0, width * self.grid_size, height * self.grid_size))
    }
    pub fn grid_color(&self, x: usize, y: usize) -> &str {
        if (x + y) % 2 == 0 { self.board_white.as_str() } else { self.board_black.as_str() }
    }
    pub fn draw_square(&self, x: usize, y: usize) -> Rectangle {
        Rectangle::new()
            .set("x", x as f32 * self.grid_size)
            .set("y", y as f32 * self.grid_size)
            .set("width", self.grid_size)
            .set("height", self.grid_size)
            .set("fill", self.grid_color(x, y))
    }
    pub fn draw_path(&self, x1: usize, y1: usize, x2: usize, y2: usize) -> Line {
        Line::new()
            .set("x1", x1 as f32 * self.grid_size + self.grid_size / 2.0)
            .set("y1", y1 as f32 * self.grid_size + self.grid_size / 2.0)
            .set("x2", x2 as f32 * self.grid_size + self.grid_size / 2.0)
            .set("y2", y2 as f32 * self.grid_size + self.grid_size / 2.0)
            .set("stroke", self.path_color.as_str())
            .set("stroke-width", self.path_width)
    }
}
