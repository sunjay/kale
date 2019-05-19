use crate::texture::Texture;

pub struct Canvas<'a> {
    texture: &'a mut Texture,
}

impl<'a> Canvas<'a> {
    pub(crate) fn new(texture: &'a mut Texture) -> Self {
        Self {texture}
    }
}
