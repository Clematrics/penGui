struct Vertex {
	position : [f32; 3],
	color : [f32; 4]
}

struct Uniform {} // TODO : adapt definition to real uniforms

enum DrawMode {
	TriangleFan,
	LineFan,
	// ... TODO : to complete
}

struct DrawCommand<Texture> {
	vertex_buffer : Vec<Vertex>,
	index_buffer  : Vec<u32>, // Wrapper
	draw_mode     : DrawMode, //
	clipping      : [[f32;2];2],
	uniforms      : Vec<Uniform> // Option
	texture       : Texture
}

trait Backend<T> {
	fn draw_command(&self, T, &DrawCommand);
}