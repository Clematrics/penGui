use crate::core;
use crate::core::{DrawCommand, Vertex};

use glium::Surface;

static VERTEX_SHADER_SRC: &'static str = r#"
#version 330

in vec3 position;
in vec4 color;

out vec4 pipe_color;

uniform mat4 perspective;
uniform mat4 view;
uniform mat4 model;

void main() {
	gl_Position = perspective * view * model * vec4(position, 1.0);
	pipe_color = color;
}
"#;

static FRAGMENT_SHADER_SRC: &'static str = r#"
#version 330

in vec4 pipe_color;

out vec4 out_color;

void main() {
	out_color = vec4(pipe_color.xyz, 1.0);
}
"#;

pub struct GliumBackend {
    display: glium::Display,
    draw_parameters: glium::DrawParameters<'static>,
    program: glium::Program,
}

glium::implement_vertex!(Vertex, position, color);

impl GliumBackend {
    pub fn new(facade: glium::Display) -> Self {
        let program =
            glium::Program::from_source(&facade, &VERTEX_SHADER_SRC, &FRAGMENT_SHADER_SRC, None)
                .unwrap();

        Self {
            display: facade,
            draw_parameters: glium::DrawParameters {
				backface_culling: glium::BackfaceCullingMode::CullingDisabled,
				depth: glium::Depth {
					test: glium::draw_parameters::DepthTest::IfLess,
					write: true,
					.. Default::default()
				},
				.. Default::default()
			},
            program: program,
        }
    }
}

impl core::Backend for GliumBackend {
    type DrawResult = Result<(), glium::DrawError>;
    type Frame = glium::Frame;

    fn draw_command(&self, frame: &mut Self::Frame, command: &DrawCommand) -> Self::DrawResult {
        let vertex_buffer =
            glium::VertexBuffer::immutable(&self.display, &command.vertex_buffer.as_slice())
                .unwrap();
        let index_buffer = glium::IndexBuffer::immutable(
            &self.display,
            glium::index::PrimitiveType::TrianglesList,
            &command.index_buffer.as_slice(),
        )
        .unwrap();

        let core::Uniform::Mat4(perspective) = command.uniforms[0];
        let core::Uniform::Mat4(view) = command.uniforms[1];
        let core::Uniform::Mat4(model) = command.uniforms[2];
        let uniforms = glium::uniform! { perspective: perspective, view: view, model: model };

        frame.draw(
            &vertex_buffer,
            &index_buffer,
            &self.program,
            &uniforms,
            &self.draw_parameters,
        )
    }

    fn new_frame(&self) -> Self::Frame {
        self.display.draw()
    }
}
