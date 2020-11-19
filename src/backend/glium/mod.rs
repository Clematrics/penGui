use crate::core::{DrawCommand, Mat4x4, Vertex};

use glium::Surface;

static VERTEX_SHADER_SRC: &'static str = r#"
#version 330

in vec3 position;
in vec4 color;
in vec2 tex_uv;

out vec4 pipe_color;
out vec2 pipe_tex_uv;

uniform mat4 perspective_view;
uniform mat4 model;

void main() {
	gl_Position = perspective_view * model * vec4(position, 1.0);
	pipe_color = color;
	pipe_tex_uv = tex_uv;
}
"#;

static FRAGMENT_SHADER_SRC: &'static str = r#"
#version 330

in vec4 pipe_color;
in vec2 pipe_tex_uv;

out vec4 out_color;

uniform sampler2D t;

void main() {
	out_color = vec4(pipe_color.xyz, 1.0) * texture(t, pipe_tex_uv);
}
"#;

pub struct GliumBackend {
    display: glium::Display,
    draw_parameters: glium::DrawParameters<'static>,
    program: glium::Program,
    blank_texture: glium::Texture2d,
    textures: Vec<glium::Texture2d>,
}

type DrawResult = Result<(), glium::DrawError>;
type Frame = glium::Frame;

glium::implement_vertex!(Vertex, position, color, tex_uv);

impl GliumBackend {
    pub fn new(facade: glium::Display) -> Self {
        let program =
            glium::Program::from_source(&facade, &VERTEX_SHADER_SRC, &FRAGMENT_SHADER_SRC, None)
                .unwrap();

        let img: Vec<u8> = vec![255, 255, 255, 255];
        let blank_texture = glium::texture::RawImage2d::from_raw_rgba(img, (1, 1));
        let blank_texture = glium::Texture2d::new(&facade, blank_texture).unwrap();

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
            program: program,
            blank_texture,
            textures: vec![],
        }
    }

    pub fn draw_command(
        &self,
        frame: &mut Frame,
        global_transform: Mat4x4,
        command: &DrawCommand,
    ) -> DrawResult {
        let vertex_buffer =
            glium::VertexBuffer::immutable(&self.display, &command.vertex_buffer.as_slice())
                .unwrap();
        let index_buffer = glium::IndexBuffer::immutable(
            &self.display,
            glium::index::PrimitiveType::TrianglesList,
            &command.index_buffer.as_slice(),
        )
        .unwrap();

        let uniforms = glium::uniform! {
            perspective_view: global_transform,
            model: command.uniforms.model_matrix,
            t: if let Some(id) = command.uniforms.texture { &self.textures[id as usize] } else { &self.blank_texture },
        };

        frame.draw(
            &vertex_buffer,
            &index_buffer,
            &self.program,
            &uniforms,
            &self.draw_parameters,
        )
    }

    pub fn new_frame(&self) -> Frame {
        self.display.draw()
    }
}
