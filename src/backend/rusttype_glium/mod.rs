extern crate rusttype;

use std::borrow::Cow;

use crate::core::{CharacterInfo, FontAtlas, TextureId};

use rusttype::gpu_cache::{Cache, CacheBuilder};
use rusttype::{Font, Point, Scale};

pub struct FontWrapper {
    cache: Cache<'static>,
    font: Font<'static>,
    scale: Scale,
    pub texture: glium::Texture2d,
    texture_id: Option<TextureId>,
}

impl FontWrapper {
    pub fn new(display: &glium::Display) -> Self {
        let default_scale = 12.0;
        let default_size = 512.0;
        let size = (default_size * default_scale) as u32;
        let raw = glium::texture::RawImage2d {
            data: Cow::Owned(vec![128u8; size as usize * size as usize]),
            width: size,
            height: size,
            format: glium::texture::ClientFormat::U8,
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
            scale: Scale::uniform(12.0),
            texture,
            texture_id: None,
        }
    }

    pub fn set_id(&mut self, texture_id: TextureId) {
        self.texture_id = Some(texture_id);
    }
}

impl FontAtlas for FontWrapper {
    fn get_texture(&self) -> TextureId {
        match self.texture_id {
            Some(id) => id,
            None => panic!("No id was attributed to this FontWrapper"),
        }
    }

    fn char_info(&mut self, character: char) -> CharacterInfo {
        let glyph = self
            .font
            .glyph(character)
            .scaled(self.scale)
            .positioned(Point { x: 0.0, y: 0.0 });
        self.cache.queue_glyph(0, glyph.clone());
        let texture = &&self.texture;
        self.cache
            .cache_queued(|rect, data| {
                texture.main_level().write(
                    glium::Rect {
                        left: rect.min.x,
                        bottom: rect.min.y,
                        width: rect.width(),
                        height: rect.height(),
                    },
                    glium::texture::RawImage2d {
                        data: Cow::Borrowed(data),
                        width: rect.width(),
                        height: rect.height(),
                        format: glium::texture::ClientFormat::U8,
                    },
                );
            })
            .unwrap();

        let (uv_coords, _) = self.cache.rect_for(0, &glyph).ok().flatten().unwrap();

        CharacterInfo {
            size: (
                uv_coords.max.x - uv_coords.min.x,
                uv_coords.max.y - uv_coords.min.y,
            ),
            texture_uv: (uv_coords.min.x, uv_coords.min.y),
        }
    }
}
