//! A simplified version of the turtle crate designed to test out the renderer in order to make
//! sure that it meets the needs of the turtle crate.
//!
//! The renderer is designed to only re-render as much as it needs to. This is in contrast to the
//! original naive renderer which always re-rendered all of the drawing commands every frame.

use std::thread;
use std::time::Duration;

use minifb::{Window, WindowOptions, Key, KeyRepeat};
use vek::{Vec2, Rgba};

mod radians {
    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
    pub struct Radians(f64);

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
    Push {id: ID, command: DrawCommand},
    /// Replace the last draw command pushed onto the display list with this ID.
    ///
    /// Clears the redo stack for this ID.
    Replace {id: ID, command: DrawCommand},
    /// Designates the current point in the display list as the beginning of a shape that will be
    /// filled with the given color. If either Undo/Redo are used after this command, it will clear
    /// this designation without applying the fill in any way. Use EndFill to avoid this.
    ///
    /// Subsequent BeginFill commands do not do anything. Only the first will be applied.
    BeginFill {id: ID, color: Color},
    /// Completes the filled shape started by BeginFill.
    ///
    /// If no shape was being filled, this does nothing.
    EndFill {id: ID},
    /// Remove the last draw command pushed onto the display list with this ID.
    ///
    /// Places the removed draw command onto the redo stack for this ID.
    /// Does nothing if there is no path command to remove.
    Undo {id: ID},
    /// Removes the last path command placed on the redo stack for this ID and adds it to the
    /// display list. The command remains associated with this ID.
    Redo {id: ID},
    /// Clears the entire display list for this ID
    ///
    /// Note that the very last SetPen command (if any) will be retained.
    Clear {id: ID},
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
    ///
    /// Note that since this is a draw command, undo/redo will apply to this.
    SetPen(Pen),
    /// Draw a line to the given point.
    ///
    /// Only moves the path without drawing anything if the current pen is not enabled.
    LineTo(Point),
    /// Draw a circular arc with the given parameters.
    ///
    /// Only moves the path without drawing anything if the current pen is not enabled.
    Arc {
        /// The heading to use as reference when determining the meaning of various directions like
        /// "forward", "left", "right", "backwards". (see other fields)
        heading: Radians,
        /// The radius of the circular arc to the "left" of the current position based on the
        /// heading.
        ///
        /// The arc will be drawn counterclockwise if this is position and clockwise otherwise.
        radius: f64,
        /// The angle of the circular arc to draw. Can be an entire circle (or more or less).
        ///
        /// If this is negative, the arc will be drawn "backwards" from the current position based
        /// on the heading.
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
            color: Rgba::black(),
            stroke_width: 1.0,
        }
    }
}

struct Renderer {
    width: usize,
    height: usize,
    commands: Vec<DrawCommand>,
}

impl Renderer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            commands: Vec::new(),
        }
    }

    pub fn apply(&mut self, command: RendererCommand) {
    }

    pub fn render(&self, frame_buffer: &mut [u32]) {

    }
}

fn main() {
    let id = 1;
    let pen = Pen {
        color: RED,
        stroke_width: 30.0,
        ..Pen::default()
    };
    let pen2 = Pen {
        color: BLUE,
        ..pen
    };
    let fill_color = CYAN;

    use RendererCommand::*;
    use DrawCommand::*;
    let commands = vec![
        // This line will be immediately replaced because we set the pen
        // The Replace command isn't actually meant to be used like this (it's for animations), but
        // this is a good test to make sure it works more generally
        Push {id, command: LineTo(Point {x: 0.0, y: 100.0})},
        Replace {id, command: SetPen(pen)},

        // Should still keep the pen no matter how many times we clear
        Clear {id},
        Clear {id},
        Clear {id},

        BeginFill {id, color: fill_color},

        Push {id, command: LineTo(Point {x: 0.0, y: 100.0})},
        Push {id, command: LineTo(Point {x: 100.0, y: 100.0})},
        // Change the pen halfway through, still filling
        Push {id, command: SetPen(pen2)},
        Undo {id},
        Redo {id},
        Push {id, command: LineTo(Point {x: 100.0, y: 0.0})},
        Push {id, command: Arc {
            heading: Radians::from_degrees(-90.0),
            radius: -40.0,
            extent: Radians::from_degrees(180.0),
        }},

        Undo {id},
        Undo {id},
        Redo {id},
        Redo {id},

        EndFill {id},

        Undo {id},
        Redo {id},
    ];

    const WIDTH: usize = 640;
    const HEIGHT: usize = 480;

    let mut frame_buffer = vec![0; WIDTH * HEIGHT];

    let mut renderer = Renderer::new(WIDTH, HEIGHT);

    let mut win = Window::new("Kale", WIDTH, HEIGHT,
        WindowOptions {scale: minifb::Scale::X2, ..WindowOptions::default()}).unwrap();
    for command in commands {
        if !(win.is_open() && !win.is_key_pressed(Key::Escape, KeyRepeat::No)) {
            return;
        }

        renderer.apply(command);
        renderer.render(&mut frame_buffer);
        win.update_with_buffer(&frame_buffer).unwrap();

        thread::sleep(Duration::from_millis(500));
    }

    while win.is_open() && !win.is_key_pressed(Key::Escape, KeyRepeat::No) {
        thread::sleep(Duration::from_millis(1000 / 60));
    }
}
