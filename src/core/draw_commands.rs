#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 4],
}

pub struct Uniform {} // TODO : adapt definition to real uniforms

pub enum DrawMode {
    TriangleFan,
    LineFan,
    // ... TODO : to complete
}

pub type TextureId = u32;

pub struct DrawCommand {
    pub vertex_buffer: Vec<Vertex>,
    pub index_buffer: Vec<u32>, // Wrapper
    pub draw_mode: DrawMode,    //
    pub clipping: [[f32; 2]; 2],
    pub uniforms: Vec<Uniform>, // Option
    pub texture: Option<TextureId>,
}

pub trait Backend {
    type DrawResult;
    type Frame;

    fn draw_command(
        &self,
        target: &mut Self::Frame,
        draw_command: &DrawCommand,
    ) -> Self::DrawResult;
    fn new_frame(&self) -> Self::Frame;
}
