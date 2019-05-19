use vek::{Vec2, Rgba};

type Point = Vec2<f64>;
type Color = Rgba<f64>;

mod color {
    use super::Color;

    pub const BLACK: Color = Color {r: 0.0, g: 0.0, b: 0.0, a: 1.0};
    pub const WHITE: Color = Color {r: 255.0, g: 255.0, b: 255.0, a: 1.0};

    pub const RED: Color = Color {r: 255.0, g: 0.0, b: 0.0, a: 1.0};
    pub const BLUE: Color = Color {r: 0.0, g: 255.0, b: 0.0, a: 1.0};

    pub const CYAN: Color = Color {r: 0.0, g: 255.0, b: 255.0, a: 1.0};
}

struct Pen {
    enabled: bool,
    color: Color,
    stroke_width: f64,
}

impl Default for Pen {
    fn default() -> Self {
        Self {
            enabled: true,
            color: color::BLACK,
            stroke_width: 1.0,
        }
    }
}

struct Turtle {
    pen: Pen,
    fill_color: Color,
    /// The current path animated by the turtle
    animated_path: Path,
}

impl Turtle {
    pub fn new() -> Self {
        Self {
            pen: Pen::default(),
            fill_color: color::BLACK,
        }
    }

    pub fn set_pen_size(&mut self, size: f64) {
        unimplemented!()
    }

    pub fn set_pen_color(&mut self, color: Color) {
        unimplemented!()
    }

    pub fn set_fill_color(&mut self, color: Color) {
        unimplemented!()
    }

    pub fn forward(&mut self, distance: f64) {
        unimplemented!()
    }

    pub fn right(&mut self, angle: f64) {
        unimplemented!()
    }

    /// `radius` - radius left of the turtle
    /// `extent` - angle of the circle to draw
    pub fn arc(&mut self, radius: f64, extent: f64) {
        unimplemented!();
    }

    pub fn begin_fill(&mut self) {
        unimplemented!()
    }

    pub fn end_fill(&mut self) {
        unimplemented!()
    }
}
