//! The `glium`-based backend module.
//!
//! Check the `glium-experimental` example to see how the backend is used.

use std::cell::RefCell;
use std::rc::*;

use crate::core::{DrawCommand, DrawList, Mat4x4, TextureId, Vertex};

use glium::Surface;

use super::rusttype_glium::FontWrapper;

/// `glium`-based backend
///
/// A structure holding all the information needed to
/// display 3D content in a window using the [`glium`](https://crates.io/crates/glium) crate.
///
/// This backend is also a texture manager.
///
/// Has a few functions to draw penGui interface given the interface's draw list.
pub struct GliumBackend {
    display: glium::Display,
    draw_parameters: glium::DrawParameters<'static>,
    debug_rendering: bool,
    program: glium::Program,
    blank_texture: glium::Texture2d,
    textures: Vec<glium::Texture2d>,
    fonts: Vec<Rc<RefCell<FontWrapper>>>,
}

impl GliumBackend {
    /// Creates a new glium backend from a drawing surface.
    /// The texture manager is loaded and a default blank texture is created.
    pub fn new(facade: glium::Display) -> Self {
        let program =
            glium::Program::from_source(&facade, &VERTEX_SHADER_SRC, &FRAGMENT_SHADER_SRC, None)
                .unwrap();

        let img: Vec<u8> = vec![255, 255, 255, 255];
        let blank_texture = glium::texture::RawImage2d::from_raw_rgba(img, (1, 1));
        let blank_texture = glium::Texture2d::new(&facade, blank_texture).unwrap();

        let mut default_font = FontWrapper::new(&facade);
        default_font.set_id(TextureId::Font(0));

        Self {
            display: facade,
            draw_parameters: glium::DrawParameters {
                depth: glium::Depth {
                    test: glium::draw_parameters::DepthTest::IfLess,
                    write: true,
                    ..Default::default()
                },
                blend: glium::Blend::alpha_blending(),
                line_width: Some(1.0),
                point_size: Some(1.0),
                ..Default::default()
            },
            debug_rendering: false,
            program,
            blank_texture,
            textures: Vec::new(),
            fonts: vec![Rc::new(RefCell::new(default_font))],
        }
    }

    pub fn switch_debug_rendering(&mut self) {
        match self.draw_parameters.polygon_mode {
            glium::draw_parameters::PolygonMode::Fill => {
                self.draw_parameters.polygon_mode = glium::draw_parameters::PolygonMode::Line;
                self.debug_rendering = true;
            }
            glium::draw_parameters::PolygonMode::Line => {
                self.draw_parameters.polygon_mode = glium::draw_parameters::PolygonMode::Point;
                self.debug_rendering = true;
            }
            glium::draw_parameters::PolygonMode::Point => {
                self.draw_parameters.polygon_mode = glium::draw_parameters::PolygonMode::Fill;
                self.debug_rendering = false;
            }
        }
    }

    /// Draws a single draw command from penGui on a frame.
    /// A transformation is applied globally to every vertex after all others.
    /// This transformation is especially useful to specify the perspective & view matrix.
    ///
    /// # Errors
    ///
    /// Passes any `DrawError` `glium` returns.
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

        if let Some(id) = &command.uniforms.texture {
            let texture_id = match id {
                TextureId::Texture(id) => id,
                TextureId::Font(id) => id,
            };
            let font = self.fonts[*texture_id].borrow();
            let texture: &Texture = match id {
                TextureId::Texture(id) => &self.textures[*id],
                TextureId::Font(_) => &font.texture,
            };
            let uniforms = glium::uniform! {
                perspective_view: global_transform,
                model: command.uniforms.model_matrix,
                t: if self.debug_rendering { &self.blank_texture } else { texture },
            };

            frame.draw(
                &vertex_buffer,
                &index_buffer,
                &self.program,
                &uniforms,
                &self.draw_parameters,
            )
        } else {
            let uniforms = glium::uniform! {
                perspective_view: global_transform,
                model: command.uniforms.model_matrix,
                t: &self.blank_texture,
            };

            frame.draw(
                &vertex_buffer,
                &index_buffer,
                &self.program,
                &uniforms,
                &self.draw_parameters,
            )
        }

        // let texture: &Texture = if let Some(id) = &command.uniforms.texture {
        //     match id {
        //         TextureId::Texture(id) => &self.textures[*id],
        //         TextureId::Font(id) => &font.texture,
        //     }
        // } else {
        //     &self.blank_texture
        // };

        // let uniforms = glium::uniform! {
        //     perspective_view: global_transform,
        //     model: command.uniforms.model_matrix,
        //     t: texture,
        // };

        // frame.draw(
        //     &vertex_buffer,
        //     &index_buffer,
        //     &self.program,
        //     &uniforms,
        //     &self.draw_parameters,
        // )
    }

    /// Creates a new frame to draw on
    pub fn new_frame(&self) -> Frame {
        self.display.draw()
    }

    /// Draws recursively a list of commands from penGui on a frame.
    /// A transformation is applied globally to every vertex after all others.
    /// This transformation is especially useful to specify the perspective & view matrix.
    ///
    /// # Errors
    ///
    /// Passes any `DrawError` `glium` returns.
    pub fn draw_list(
        &self,
        frame: &mut Frame,
        global_transform: Mat4x4,
        list: &DrawList,
    ) -> DrawResult {
        list.commands
            .iter()
            .try_for_each(|command| self.draw_command(frame, global_transform, command))?;
        list.list
            .iter()
            .try_for_each(|list| self.draw_list(frame, global_transform, list))
    }

    /// Registers a new texture and returns the unique ID associated with it.
    pub fn register_texture(&mut self, image: RawTexture) -> TextureId {
        let texture = glium::texture::Texture2d::with_mipmaps(
            &self.display,
            image,
            glium::texture::MipmapsOption::NoMipmap,
        )
        .unwrap();
        let id = self.textures.len();
        self.textures.push(texture);
        TextureId::Texture(id)
    }

    /// Registers a new font and returns the unique ID associated with it.
    pub fn register_font(&mut self, font: FontWrapper) -> TextureId {
        let id = self.fonts.len();
        self.fonts.push(Rc::new(RefCell::new(font)));
        TextureId::Font(id)
    }

    /// Get a font from its id
    pub fn get_font(&mut self, id: usize) -> Weak<RefCell<FontWrapper>> {
        Rc::downgrade(&self.fonts[id])
    }
}

// Useful type abbreviations
type DrawResult = Result<(), glium::DrawError>;
type Frame = glium::Frame;
type RawTexture = glium::texture::RawImage2d<'static, u8>;
type Texture = glium::Texture2d;

// creation of the vertex structure for `glium` from the penGui one
glium::implement_vertex!(Vertex, position, color, tex_uv);

/// GLSL vertex shader source
static VERTEX_SHADER_SRC: &str = r#"
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

/// GLSL fragment shader source
static FRAGMENT_SHADER_SRC: &str = r#"
#version 330

in vec4 pipe_color;
in vec2 pipe_tex_uv;

out vec4 out_color;

uniform sampler2D t;

void main() {
	out_color = vec4(pipe_color.xyz, 1.0) * texture(t, pipe_tex_uv);
}
"#;
