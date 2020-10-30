use crate::core;
use crate::core::{DrawCommand, TextureId, Vertex};

use glium::Surface;

static VERTEX_SHADER_NO_TEXTURE_SRC: &'static str = r#"
#version 330

in vec3 position;
in vec4 color;
in vec2 tex_uv;

out vec4 pipe_color;

uniform mat4 perspective;
uniform mat4 view;
uniform mat4 model;

void main() {
	gl_Position = perspective * view * model * vec4(position, 1.0);
	pipe_color = color;
}
"#;

static FRAGMENT_SHADER_NO_TEXTURE_SRC: &'static str = r#"
#version 330

in vec4 pipe_color;

out vec4 out_color;

void main() {
	out_color = vec4(pipe_color.xyz, 1.0);
}
"#;

static VERTEX_SHADER_SRC: &'static str = r#"
#version 330

in vec3 position;
in vec4 color;
in vec2 tex_uv;

out vec4 pipe_color;
out vec2 pipe_tex_uv;

uniform mat4 perspective;
uniform mat4 view;
uniform mat4 model;

void main() {
	gl_Position = perspective * view * model * vec4(position, 1.0);
	pipe_color = color;
	pipe_tex_uv = tex_uv;
}
"#;

static FRAGMENT_SHADER_SRC: &'static str = r#"
#version 330

in vec4 pipe_color;
in vec2 pipe_tex_uv;

uniform sampler2D texture_0;

out vec4 out_color;

void main() {
	out_color = texture(texture_0, pipe_tex_uv) * vec4(pipe_color.xyz, 1.0);
}
"#;

pub struct GliumBackend {
    display: glium::Display,
    draw_parameters: glium::DrawParameters<'static>,
    program_wo_texture: glium::Program,
    program_w_texture: glium::Program,
    textures: Vec<glium::Texture2d>,
}

glium::implement_vertex!(Vertex, position, color, tex_uv);

impl GliumBackend {
    pub fn new(facade: glium::Display) -> Self {
        let program_wo_texture = glium::Program::from_source(
            &facade,
            &VERTEX_SHADER_NO_TEXTURE_SRC,
            &FRAGMENT_SHADER_NO_TEXTURE_SRC,
            None,
        )
        .unwrap();

        let program_w_texture =
            glium::Program::from_source(&facade, &VERTEX_SHADER_SRC, &FRAGMENT_SHADER_SRC, None)
                .unwrap();

        Self {
            display: facade,
            draw_parameters: glium::DrawParameters {
                depth: glium::Depth {
                    test: glium::draw_parameters::DepthTest::IfLess,
                    write: true,
                    ..Default::default()
                },
                blend: glium::Blend::alpha_blending(),
                ..Default::default()
            },
            program_wo_texture: program_wo_texture,
            program_w_texture: program_w_texture,
            textures: vec![],
        }
    }
}

impl core::Backend for GliumBackend {
    type DrawResult = Result<(), glium::DrawError>;
    type Frame = glium::Frame;
    type Texture = glium::texture::RawImage2d<'static, u8>;

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

        match command.uniforms.texture_0 {
            Some(texture_id) => {
                let uniforms = glium::uniform! { perspective: command.uniforms.perspective, view: command.uniforms.view, model: command.uniforms.model, texture_0: &self.textures[texture_id] };

                frame.draw(
                    &vertex_buffer,
                    &index_buffer,
                    &self.program_w_texture,
                    &uniforms,
                    &self.draw_parameters,
                )
            }
            None => {
                let uniforms = glium::uniform! { perspective: command.uniforms.perspective, view: command.uniforms.view, model: command.uniforms.model };

                frame.draw(
                    &vertex_buffer,
                    &index_buffer,
                    &self.program_wo_texture,
                    &uniforms,
                    &self.draw_parameters,
                )
            }
        }
    }

    fn new_frame(&self) -> Self::Frame {
        self.display.draw()
    }

    fn register_texture(&mut self, image: Self::Texture) -> TextureId {
        let texture = glium::texture::Texture2d::new(&self.display, image).unwrap();
        let id = self.textures.len();
        self.textures.push(texture);
        id
    }
}
