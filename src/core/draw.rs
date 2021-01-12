use crate::core::*;

pub fn quad(
    width: f32,
    height: f32,
    texture: Option<TextureId>,
    color: (f32, f32, f32, f32),
    position: (f32, f32, f32),
) -> DrawCommand {
    let (r, g, b, a) = color;
    let color = [r, g, b, a];

    let (x, y, z) = position;
    let mut uniforms = Uniforms::new();
    uniforms.model_matrix = nalgebra::Translation3::new(x, y, z).to_homogeneous();
    uniforms.texture = texture;

    DrawCommand {
        vertex_buffer: vec![
            Vertex {
                position: [0., 0., 0.],
                color,
                tex_uv: [0., 0.],
            },
            Vertex {
                position: [width, 0., 0.],
                color,
                tex_uv: [1., 0.],
            },
            Vertex {
                position: [0., height, 0.],
                color,
                tex_uv: [0., 1.],
            },
            Vertex {
                position: [width, height, 0.],
                color,
                tex_uv: [1., 1.],
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
    position: (f32, f32, f32),
) -> DrawCommand {
    let (r, g, b, a) = color;
    let color = [r, g, b, a];

    let tex_uv = [0., 0.];

    let (x, y, z) = position;
    let mut uniforms = Uniforms::new();
    uniforms.model_matrix = nalgebra::Translation3::new(x, y, z).to_homogeneous();

    DrawCommand {
        vertex_buffer: vec![
            Vertex {
                position: [0., 0., 0.],
                color,
                tex_uv,
            },
            Vertex {
                position: [width, 0., 0.],
                color,
                tex_uv,
            },
            Vertex {
                position: [0., height, 0.],
                color,
                tex_uv,
            },
            Vertex {
                position: [width, height, 0.],
                color,
                tex_uv,
            },
        ],
        index_buffer: vec![0, 1, 0, 2, 1, 3, 2, 3],
        draw_mode: DrawMode::Lines,
        uniforms,
    }
}
