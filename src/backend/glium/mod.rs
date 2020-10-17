use crate::core;
use crate::core{DrawCommand};

let 'static vertex_shader_src = r#"
#version 330

in vec3 position;

void main() {
	gl_Position = vec4(position, 1.0);
}
"#;

let 'static fragment_shader_src = r#"
#version 330

out vec4 color;

void main() {
	color = vec4(1.0, 1.0, 1.0, 1.0);
}
"#;

struct GliumBackend {
	draw_parameters: glium::DrawParameters,
	uniforms: uniform! {},
	program: glium::Program
}

impl Backend<Glium::Frame> for GliumBackend<> {
	fn new() {
		GliumBackend {
			draw_parameters = Default::default(),
			uniforms = uniform! {},
			program: glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();
		}
	}

	fn draw_command(&self, &mut frame: glium::Frame, &command: DrawCommand) {
		target
            .draw(
                &command.vertex_buffer,
				&command.index_buffer,
                &self.program,
                &self.uniforms,
                &self.draw_parameters,
            )
            .unwrap();
	}
}


