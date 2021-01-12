//! A wrapper around `rusttype`'s fonts that implements the `FontAtlas` trait.
//! Can only be used with a `glium` backend.

extern crate rusttype;

use std::borrow::Cow;

use crate::core::{CharacterInfo, FontAtlas, TextureId, VerticalMetrics};

use rusttype::gpu_cache::{Cache, CacheBuilder};
use rusttype::{Font, Point, Scale, VMetrics};

pub struct FontWrapper {
    cache: Cache<'static>,
    font: Font<'static>,
    scale: Scale,
    pub texture: glium::Texture2d,
    texture_id: Option<TextureId>,
    ascent: f32,
    descent: f32,
    line_gap: f32,
}

impl FontWrapper {
    pub fn new(display: &glium::Display) -> Self {
        let default_scale = 128.0;
        let default_count = 32.0;
        let size = (default_count * default_scale) as u32;
        let raw = glium::texture::RawImage2d {
            data: Cow::Owned(vec![255u8; size as usize * size as usize * 4]),
            width: size,
            height: size,
            format: glium::texture::ClientFormat::U8U8U8U8,
        };
        let font_data = include_bytes!("../../../resources/wqy-microhei/wqy-microhei.ttc");
        let font = Font::try_from_bytes(font_data).unwrap();
        let scale = Scale::uniform(default_scale);
        let VMetrics {
            ascent,
            descent,
            line_gap,
        } = font.v_metrics(scale);
        let f = ascent - descent;
        let ascent = ascent / f;
        let descent = descent / f;
        let line_gap = if line_gap == 0. { 0.2 } else { line_gap / f };
        let texture = glium::texture::Texture2d::with_mipmaps(
            display,
            raw,
            glium::texture::MipmapsOption::NoMipmap,
        )
        .unwrap();
        Self {
            cache: CacheBuilder::default().dimensions(size, size).build(),
            font,
            scale,
            texture,
            texture_id: None,
            ascent,
            descent,
            line_gap,
        }
    }

    pub fn set_id(&mut self, texture_id: TextureId) {
        self.texture_id = Some(texture_id);
    }
}

impl FontAtlas for FontWrapper {
    fn get_vertical_metrics(&self) -> VerticalMetrics {
        VerticalMetrics {
            ascent: self.ascent,
            descent: self.descent,
            line_gap: self.line_gap,
        }
    }

    fn get_texture(&self) -> TextureId {
        match self.texture_id {
            Some(id) => id,
            None => panic!("No id was attributed to this FontWrapper"),
        }
    }

    fn char_info(
        &mut self,
        character: char,
        previous_char: Option<char>,
        size: f32,
    ) -> CharacterInfo {
        let glyph = self
            .font
            .glyph(character)
            .scaled(self.scale)
            .positioned(Point { x: 0.0, y: 0.0 });
        self.cache.queue_glyph(0, glyph.clone());
        let texture = &&self.texture;
        self.cache
            .cache_queued(|rect, data| {
                let vec: Vec<u8> = data.iter().flat_map(|u| vec![255, 255, 255, *u]).collect();
                texture.main_level().write(
                    glium::Rect {
                        left: rect.min.x,
                        bottom: rect.min.y,
                        width: rect.width(),
                        height: rect.height(),
                    },
                    glium::texture::RawImage2d {
                        data: Cow::Borrowed(vec.as_slice()),
                        width: rect.width(),
                        height: rect.height(),
                        format: glium::texture::ClientFormat::U8U8U8U8,
                    },
                );
            })
            .unwrap();

        let factor = size / self.scale.x;
        let kerning = previous_char
            .map(|c| {
                let previous_glyph = self
                    .font
                    .glyph(c)
                    .scaled(self.scale)
                    .positioned(Point { x: 0.0, y: 0.0 });
                factor
                    * self
                        .font
                        .pair_kerning(self.scale, previous_glyph.id(), glyph.id())
            })
            .unwrap_or(0.);
        let advance_width = factor * glyph.unpositioned().h_metrics().advance_width;

        let (uv_coords, rect) = self
            .cache
            .rect_for(0, &glyph)
            .ok()
            .flatten()
            .unwrap_or_default();

        // println!("character is {} with rect {:?}", character, rect);
        // let l = std::io::stdin().read_line(&mut String::new());

        let top_left = (
            rect.min.x as f32 * factor,
            (-rect.min.y as f32) * factor - self.descent * size,
            // rusttype uses screen coordinates, where y increases when going downward
            // thus we flip the y axis
        );
        let bottom_right = (
            rect.max.x as f32 * factor,
            (-rect.max.y as f32) * factor - self.descent * size,
            // rusttype uses screen coordinates, where y increases when going downward
            // thus we flip the y axis
        );

        CharacterInfo {
            texture_size: (
                uv_coords.max.x - uv_coords.min.x,
                uv_coords.max.y - uv_coords.min.y,
            ),
            texture_uv: (uv_coords.min.x, uv_coords.min.y),
            top_left,
            bottom_right,
            advance_width,
            kerning,
        }
    }

    fn size_of(&self, string: &str, size: f32) -> (f32, f32) {
        let mut previous_glyph: Option<rusttype::PositionedGlyph> = None;
        let mut width = 0.;
        let factor = size / self.scale.x;

        for c in string.chars() {
            let glyph = self
                .font
                .glyph(c)
                .scaled(self.scale)
                .positioned(Point { x: 0.0, y: 0.0 });
            let kerning = previous_glyph
                .map(|previous_glyph| {
                    self.font
                        .pair_kerning(self.scale, previous_glyph.id(), glyph.id())
                })
                .unwrap_or(0.);
            let advance_width = glyph.unpositioned().h_metrics().advance_width;
            width += (advance_width + kerning) * factor;

            previous_glyph = Some(glyph);
        }

        (width, size)
    }
}
