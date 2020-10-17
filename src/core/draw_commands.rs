pub struct Vertex {
	position: [f32; 3],
	color: [f32; 4],
}

pub struct Uniform {} // TODO : adapt definition to real uniforms

pub enum DrawMode {
	TriangleFan,
	LineFan,
	// ... TODO : to complete
}

pub struct DrawCommand<Texture> {
	pub vertex_buffer: Vec<Vertex>,
	pub index_buffer: Vec<u32>, // Wrapper
	pub draw_mode: DrawMode,    //
	pub clipping: [[f32; 2]; 2],
	pub uniforms: Vec<Uniform>, // Option
	pub texture: Option<Texture>,
}

pub trait Backend<T> {
	fn draw_command(&self, t: T, d: &DrawCommand<T>);
}
