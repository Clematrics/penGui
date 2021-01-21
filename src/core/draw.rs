use crate::core::*;
use nalgebra::{Similarity3, Vector2, Vector3};

pub fn quad(
    width: f32,
    height: f32,
    texture: Option<TextureId>,
    color: (f32, f32, f32, f32),
    transform: Similarity3<f32>,
) -> DrawCommand {
    let mut uniforms = Uniforms::new();
    uniforms.model_matrix = transform.to_homogeneous();
    uniforms.texture = texture;

    DrawCommand {
        vertex_buffer: vec![
            Vertex {
                position: Vector3::new(0., 0., 0.),
                color,
                tex_uv: Vector2::new(0., 0.),
            },
            Vertex {
                position: Vector3::new(width, 0., 0.),
                color,
                tex_uv: Vector2::new(1., 0.),
            },
            Vertex {
                position: Vector3::new(0., height, 0.),
                color,
                tex_uv: Vector2::new(0., 1.),
            },
            Vertex {
                position: Vector3::new(width, height, 0.),
                color,
                tex_uv: Vector2::new(1., 1.),
            },
        ],
        index_buffer: vec![0, 1, 2, 1, 2, 3],
        draw_mode: DrawMode::Triangles,
        uniforms,
    }
}

pub fn debug_quad(
    width: f32,
    height: f32,
    color: (f32, f32, f32, f32),
    transform: Similarity3<f32>,
) -> DrawCommand {
    let tex_uv = Vector2::new(0., 0.);

    let mut uniforms = Uniforms::new();
    uniforms.model_matrix = transform.to_homogeneous();

    DrawCommand {
        vertex_buffer: vec![
            Vertex {
                position: Vector3::new(0., 0., 0.),
                color,
                tex_uv,
            },
            Vertex {
                position: Vector3::new(width, 0., 0.),
                color,
                tex_uv,
            },
            Vertex {
                position: Vector3::new(0., height, 0.),
                color,
                tex_uv,
            },
            Vertex {
                position: Vector3::new(width, height, 0.),
                color,
                tex_uv,
            },
        ],
        index_buffer: vec![0, 1, 0, 2, 1, 3, 2, 3],
        draw_mode: DrawMode::Lines,
        uniforms,
    }
}
