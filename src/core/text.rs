use std::cell::RefCell;
use std::rc::Rc;

use nalgebra::{Vector2, Vector3};

use crate::core::*;

/// A helper function returning
/// the `DrawCommand` drawing the given text with the given font,
/// color and transformation
pub fn draw_text(
    text: &str,
    font: &Rc<RefCell<dyn FontAtlas>>,
    font_size: f32,
    color: (f32, f32, f32, f32),
    transformation: Mat4x4,
) -> DrawCommand {
    let mut font = font.borrow_mut();
    let mut uniforms = Uniforms::new();
    uniforms.model_matrix = transformation;

    uniforms.texture = Some(font.get_texture());

    let count = text.chars().count();
    let mut vertex_buffer = Vec::with_capacity(count * 4);
    let mut index_buffer = Vec::<u32>::with_capacity(count * 6);
    let mut cursor = 0.;
    let mut last_char = None;
    text.chars().enumerate().for_each(|(i, c)| {
        let base = 4 * i;

        let CharacterInfo {
            texture_uv: (u, v),
            texture_size: (w, h),
            advance_width,
            top_left: (tx, ty),
            bottom_right: (bx, by),
            kerning,
        } = font.char_info(c, last_char, font_size);

        let ax = cursor + tx + kerning;
        let ay = ty;
        let bx = cursor + bx + kerning;
        let by = by;

        vertex_buffer.push(Vertex {
            position: Vector3::new(ax, ay, 0.),
            color,
            tex_uv: Vector2::new(u, v),
        });
        vertex_buffer.push(Vertex {
            position: Vector3::new(bx, ay, 0.),
            color,
            tex_uv: Vector2::new(u + w, v),
        });
        vertex_buffer.push(Vertex {
            position: Vector3::new(ax, by, 0.),
            color,
            tex_uv: Vector2::new(u, v + h),
        });
        vertex_buffer.push(Vertex {
            position: Vector3::new(bx, by, 0.),
            color,
            tex_uv: Vector2::new(u + w, v + h),
        });

        index_buffer.push(base as u32);
        index_buffer.push((base + 1) as u32);
        index_buffer.push((base + 2) as u32);
        index_buffer.push((base + 1) as u32);
        index_buffer.push((base + 2) as u32);
        index_buffer.push((base + 3) as u32);

        cursor += advance_width + kerning;
        last_char = Some(c);
    });

    DrawCommand {
        vertex_buffer,
        index_buffer,
        draw_mode: DrawMode::Triangles,
        uniforms,
    }
}
