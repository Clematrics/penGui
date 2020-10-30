#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 4],
    pub tex_uv: [f32; 2],
}

pub type Mat4 = [[f32; 4]; 4];

#[derive(Copy, Clone)]
pub struct Uniforms {
    pub perspective: Mat4,
    pub view: Mat4,
    pub model: Mat4,
    pub texture_0: Option<TextureId>,
}

// #[derive(Copy, Clone)]
// pub enum Uniform {
// 	Mat4([[f32; 4]; 4]),
// 	Texture2D(TextureId)
// }

pub enum DrawMode {
    TriangleFan,
    LineFan,
    // ... TODO : to complete
}

pub type TextureId = usize;

pub struct DrawCommand {
    pub vertex_buffer: Vec<Vertex>,
    pub index_buffer: Vec<u32>, // Wrapper
    pub draw_mode: DrawMode,    //
    pub clipping: [[f32; 2]; 2],
    pub uniforms: Uniforms, // Option
}

pub trait Backend {
    type DrawResult;
    type Frame;
    type Texture;

    fn draw_command(
        &self,
        target: &mut Self::Frame,
        draw_command: &DrawCommand,
    ) -> Self::DrawResult;
    fn new_frame(&self) -> Self::Frame;

    fn register_texture(&mut self, image: Self::Texture) -> TextureId;
}
