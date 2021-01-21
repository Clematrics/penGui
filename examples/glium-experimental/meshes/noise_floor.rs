use crate::mesh::Mesh;

use pengui::core::{DrawCommand, DrawList, DrawMode, Mat4x4, Uniforms, Vertex};

use nalgebra::{Vector2, Vector3};

pub struct NoiseFloor {
    pub radius: f32,
    draw_list: DrawList,
}

impl Default for NoiseFloor {
    fn default() -> Self {
        Self::new()
    }
}

impl NoiseFloor {
    pub fn new() -> Self {
        Self {
            radius: 5.0,
            draw_list: Self::new_draw_list(5.),
        }
    }

    pub fn change_radius(&mut self, radius: f32) {
        self.radius = radius;
        self.draw_list = Self::new_draw_list(radius);
    }

    fn new_draw_list(radius: f32) -> DrawList {
        let mut cube_vertices: Vec<Vertex> = Vec::new();
        let mut cube_indices: Vec<u32> = Vec::new();

        const A: isize = -10;
        const B: isize = 10;
        const S: u32 = (B - A + 1) as u32;

        for i in A..=B {
            for j in A..=B {
                let dist = f32::sqrt((i * i) as f32 + (j * j) as f32);
                let dist = f32::max(3. - f32::abs(dist - radius), 0.);
                cube_vertices.push(Vertex {
                    position: Vector3::new(i as f32, -10. + dist, j as f32),
                    color: (0.1 - dist / 10., 1. - dist / 3., 0.3 - dist / 4., 1.),
                    tex_uv: Vector2::zeros(),
                });
            }
        }

        for i in 0..S - 1 {
            for j in 0..S - 1 {
                let i_j_ = i + j * S;
                let i1j_ = i_j_ + 1;
                let i_j1 = i_j_ + S;
                let i1j1 = i_j1 + 1;
                cube_indices.extend_from_slice(&[i_j_, i_j1, i1j_, i_j1, i1j1, i1j_]);
            }
        }

        DrawList {
            list: vec![],
            list_transform: Mat4x4::identity(),
            commands: vec![DrawCommand {
                vertex_buffer: cube_vertices,
                index_buffer: cube_indices,
                draw_mode: DrawMode::Triangles,
                uniforms: Uniforms::new(),
            }],
        }
    }
}

impl Mesh for NoiseFloor {
    fn draw_list(&self) -> &'_ DrawList {
        &self.draw_list
    }
}
