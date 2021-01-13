use crate::mesh::Mesh;

use pengui::core::{DrawCommand, DrawList, DrawMode, Mat4x4, Uniforms, Vertex};

pub struct TestCube {
    draw_list: DrawList,
}

impl Default for TestCube {
    fn default() -> Self {
        Self::new()
    }
}

impl TestCube {
    pub fn new() -> Self {
        let cube_vertices: Vec<Vertex> = vec![
            // Front face
            Vertex {
                position: [-0.5, 0.5, -0.5],
                color: [1., 0., 0., 0.],
                tex_uv: [0., 0.],
            },
            Vertex {
                position: [0.5, 0.5, -0.5],
                color: [0., 0., 1., 0.],
                tex_uv: [0., 0.],
            },
            Vertex {
                position: [-0.5, -0.5, -0.5],
                color: [0., 1., 0., 0.],
                tex_uv: [0., 0.],
            },
            Vertex {
                position: [0.5, -0.5, -0.5],
                color: [0., 0., 0., 0.],
                tex_uv: [0., 0.],
            },
            // Right face
            Vertex {
                position: [0.5, -0.5, 0.5],
                color: [1., 1., 0., 0.],
                tex_uv: [0., 0.],
            },
            Vertex {
                position: [0.5, 0.5, 0.5],
                color: [1., 0., 1., 0.],
                tex_uv: [0., 0.],
            },
            // Left face
            Vertex {
                position: [-0.5, -0.5, 0.5],
                color: [0., 1., 1., 0.],
                tex_uv: [0., 0.],
            },
            Vertex {
                position: [-0.5, 0.5, 0.5],
                color: [1., 1., 1., 0.],
                tex_uv: [0., 0.],
            },
        ];

        let cube_indices: Vec<u32> = vec![
            0, 1, 2, 1, 3, 2, 1, 5, 3, 5, 4, 3, 6, 7, 0, 0, 2, 6, 2, 3, 6, 3, 4, 6,
        ];

        Self {
            draw_list: DrawList {
                list: vec![],
                list_transform: Mat4x4::identity(),
                commands: vec![DrawCommand {
                    vertex_buffer: cube_vertices,
                    index_buffer: cube_indices,
                    draw_mode: DrawMode::Triangles,
                    uniforms: Uniforms::new(),
                }],
            },
        }
    }
}

impl Mesh for TestCube {
    fn draw_list(&self) -> &'_ DrawList {
        &self.draw_list
    }
}
