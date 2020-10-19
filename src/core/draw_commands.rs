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

pub struct DrawCommand {
	pub vertex_buffer: Vec<Vertex>,
	pub index_buffer: Vec<u32>, // Wrapper
	pub draw_mode: DrawMode,    //
	pub clipping: [[f32; 2]; 2],
	pub uniforms: Vec<Uniform>, // Option
	pub texture: Option<u32>,
}

pub trait Backend {
	fn draw_command(&self, t: Option<u32>, d: &DrawCommand);
}
