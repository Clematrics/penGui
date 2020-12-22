use crate::core::TextureId;

pub struct CharacterInfo {
    pub texture_uv: (f32, f32),
    pub size: (f32, f32),
    pub advance_width: f32,
}

pub struct VerticalMetrics {
    pub ascent: f32,
    pub descent: f32,
    pub line_gap: f32,
}

pub trait FontAtlas {
    fn get_vertical_metrics(&self) -> VerticalMetrics;
    fn get_texture(&self) -> TextureId;
    fn char_info(&mut self, character: char) -> CharacterInfo;
}
