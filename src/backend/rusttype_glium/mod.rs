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
        let texture = glium::texture::Texture2d::with_mipmaps(
            display,
            raw,
            glium::texture::MipmapsOption::NoMipmap,
        )
        .unwrap();
        Self {
            cache: CacheBuilder::default().dimensions(size, size).build(),
            font,
            scale: Scale::uniform(default_scale),
            texture,
            texture_id: None,
        }
    }

    pub fn set_id(&mut self, texture_id: TextureId) {
        self.texture_id = Some(texture_id);
    }
}

impl FontAtlas for FontWrapper {
    fn get_vertical_metrics(&self) -> VerticalMetrics {
        let VMetrics {
            ascent,
            descent,
            line_gap,
        } = self.font.v_metrics(self.scale);
        VerticalMetrics {
            ascent: ascent / self.scale.x,
            descent: descent / self.scale.x,
            line_gap: line_gap / self.scale.x,
        }
    }

    fn get_texture(&self) -> TextureId {
        match self.texture_id {
            Some(id) => id,
            None => panic!("No id was attributed to this FontWrapper"),
        }
    }

    fn char_info(&mut self, character: char, previous_char: Option<char>) -> CharacterInfo {
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

        let kerning = previous_char
            .map(|c| {
                let previous_glyph = self
                    .font
                    .glyph(c)
                    .scaled(self.scale)
                    .positioned(Point { x: 0.0, y: 0.0 });
                self.font
                    .pair_kerning(self.scale, previous_glyph.id(), glyph.id())
                    / self.scale.x
            })
            .unwrap_or(0.);
        let advance_width = glyph.unpositioned().h_metrics().advance_width / self.scale.x;

        let (uv_coords, rect) = self
            .cache
            .rect_for(0, &glyph)
            .ok()
            .flatten()
            .unwrap_or_default();

        let top_left = (
            rect.min.x as f32 / self.scale.x,
            (-rect.min.y) as f32 / self.scale.y,
            // rusttype uses screen coordinates, where y increases when going downward
            // thus we flip the y axis
        );
        let bottom_right = (
            rect.max.x as f32 / self.scale.x,
            (-rect.max.y) as f32 / self.scale.y,
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

    fn size_of(&self, string: &str) -> (f32, f32) {
        let mut previous_glyph: Option<rusttype::PositionedGlyph> = None;
        let mut width = 0.;

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
                        / self.scale.x
                })
                .unwrap_or(0.);
            let advance_width = glyph.unpositioned().h_metrics().advance_width / self.scale.x;
            width += advance_width + kerning;

            previous_glyph = Some(glyph);
        }

        let vmetrics = self.font.v_metrics(self.scale);
        (width, vmetrics.ascent - vmetrics.descent)
    }
}
