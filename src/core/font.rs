use crate::core::TextureId;

/// Holds all information necessary to draw a character
/// or to understand its shape.
pub struct CharacterInfo {
    pub texture_uv: (f32, f32),
    pub texture_size: (f32, f32),
    pub top_left: (f32, f32),
    pub bottom_right: (f32, f32),
    pub advance_width: f32,
    pub kerning: f32,
}

/// The vertical metrics of a font.
///
/// `ascent` is the maximum height a character can take
/// from the baseline (positive number in general)
///
/// `descent` is the maximum depth a character can have
/// starting at the baseline (typically negative)
///
/// `line_gap` is the space between two lines
pub struct VerticalMetrics {
    pub ascent: f32,
    pub descent: f32,
    pub line_gap: f32,
}

/// A trait describing an atlas of characters tied to a font.
pub trait FontAtlas {
    /// Returns the vertical metrics of the font behind this atlas
    fn get_vertical_metrics(&self) -> VerticalMetrics;

    /// Returns the texture ID having all necessary characters
    fn get_texture(&self) -> TextureId;

    /// Returns the character information.
    ///
    /// The previous character can be given to get more accurate information,
    /// especially for the kerning between the two characters
    fn char_info(&mut self, character: char, previous_char: Option<char>) -> CharacterInfo;

    /// Special function which returns the width and height a single-line
    /// string would take if rendered with this atlas.
    fn size_of(&self, string: &str) -> (f32, f32);
}
