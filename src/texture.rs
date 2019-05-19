use crate::canvas::Canvas;

pub struct Texture {
}

impl Texture {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
        }
    }

    pub fn canvas(&mut self) -> Canvas {
        Canvas::new(self)
    }
}
