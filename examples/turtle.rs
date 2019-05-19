use vek::{Vec2, Rgba};

mod radians {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    struct Radians(f64);

    impl Radians {
        pub fn from_degrees(angle: f64) -> Self {
            Radians(angle.to_radians())
        }

        pub fn get(self) -> f64 {
            self.0
        }
    }
}

use radians::Radians;

type Point = Vec2<f64>;
type Color = Rgba<f64>;

type ID = u32;
type FontID = u32;

const BLACK: Color = Color {r: 0.0, g: 0.0, b: 0.0, a: 1.0};
const WHITE: Color = Color {r: 255.0, g: 255.0, b: 255.0, a: 1.0};

const RED: Color = Color {r: 255.0, g: 0.0, b: 0.0, a: 1.0};
const BLUE: Color = Color {r: 0.0, g: 255.0, b: 0.0, a: 1.0};

const CYAN: Color = Color {r: 0.0, g: 255.0, b: 255.0, a: 1.0};

#[derive(Debug)]
enum RendererCommand {
    /// Push a draw command onto the display list and associate it with this ID.
    ///
    /// Clears the redo stack for this ID.
    Push {id: ID, command: PathCommand},
    /// Replace the last draw command pushed onto the display list with this ID.
    ///
    /// Clears the redo stack for this ID.
    Replace {id: ID, command: PathCommand},
    /// Designates the current point in the display list as the beginning of a shape that will be
    /// filled with the given color. If either Undo/Redo are used after this command, it will clear
    /// this designation.
    ///
    /// Subsequent BeginFill commands do not do anything. Only the first will be applied.
    BeginFill(Color),
    /// Completes the filled shape started by BeginFill.
    ///
    /// If no shape was being filled, this does nothing.
    EndFill,
    /// Remove the last draw command pushed onto the display list with this ID.
    ///
    /// Places the removed draw command onto the redo stack for this ID.
    /// Does nothing if there is no path command to remove.
    Undo {id: ID},
    /// Removes the last path command placed on the redo stack for this ID and adds it to the
    /// display list. The command remains associated with this ID.
    Redo {id: ID},
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Align {
    Left,
    Center,
    Right,
}

#[derive(Debug)]
struct Text {
    font: FontID,
    /// The string to render as text
    content: String,
    /// The vertical size of font to use
    font_size: f64,
    /// The position of the bottom of the drawn text
    start: Point,
    /// Dictates whether `start` represents the bottom-left, bottom-center, or bottom-right
    alignment: Align,
    /// The angle to draw the text
    angle: Radians, //TODO: Can we support this?
}

#[derive(Debug)]
struct Image {
    /// Buffer must be exactly `width * height` size
    buffer: Vec<Color>, //TODO
    /// Width of the buffer in pixels
    width: usize,
    /// Height of the buffer in pixels
    height: usize,
    /// The position of the bottom of the drawn image
    start: Point,
    /// Dictates whether `start` represents the bottom-left, bottom-center, or bottom-right
    alignment: Align,
    /// The angle to draw the image
    angle: Radians, //TODO: Can we support this?
}

#[derive(Debug)]
enum DrawCommand {
    /// If the pen is not enabled, any subsequent draw commands will move to their destination
    /// position without drawing anything.
    SetPen(Pen),
    /// Draw a line to the given point. Only moves the path if the current pen is not enabled.
    LineTo(Point),
    /// Draw an arc with the given parameters. Only moves the path if the current pen is not enabled.
    Arc {
        /// radius left of the turtle
        radius: f64,
        /// angle of the circle to draw in radians
        extent: Radians,
    },
    /// Draws text using current pen.
    ///
    /// This will draw a line to the bottom-right of the drawn text. Note that if the pen is not
    /// enabled, no text will be drawn and no line will be produced, but the path will still move an
    /// adequate amount to draw the text. (Consistent with the behaviour of LineTo)
    Text(Text),
    /// Draws an image.
    ///
    /// This will draw a line to the bottom-right of the drawn image. Note that if the pen is not
    /// enabled, no image will be drawn and no line will be produced, but the path will still move
    /// an adequate amount to draw the image. (Consistent with the behaviour of LineTo)
    Image(Image),
}

#[derive(Debug)]
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
