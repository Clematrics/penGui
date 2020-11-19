#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 4],
    pub tex_uv: [f32; 2],
}

pub type Mat4x4 = [[f32; 4]; 4];
pub type TextureId = u32;

pub const UNIT_TRANSFORM: Mat4x4 = [
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0, 0.0],
    [0.0, 0.0, 0.0, 1.0],
];

pub struct Uniforms {
    pub model_matrix: Mat4x4,
    pub texture: Option<TextureId>,
}

impl Uniforms {
    pub const fn new() -> Self {
        Self {
            model_matrix: UNIT_TRANSFORM,
            texture: None,
        }
    }
}

pub enum DrawMode {
    TriangleFan,
    LineFan,
    // ... TODO : to complete
}

pub struct DrawCommand {
    pub vertex_buffer: Vec<Vertex>,
    pub index_buffer: Vec<u32>, // Wrapper
    pub draw_mode: DrawMode,    //
    pub uniforms: Uniforms,
}

pub static NULL_DRAW_COMMAND: DrawCommand = DrawCommand {
    vertex_buffer: vec![],
    index_buffer: vec![],
    draw_mode: DrawMode::TriangleFan,
    uniforms: Uniforms::new(),
};
