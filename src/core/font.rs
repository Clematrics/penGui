use crate::core::TextureId;

pub struct CharacterInfo {
    pub texture_uv: (f32, f32),
    pub size: (f32, f32),
}

pub trait FontAtlas {
    fn get_texture(&self) -> TextureId;
    fn char_texture(&mut self, character: char) -> CharacterInfo;
}
