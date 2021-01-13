//! The `glium`-based backend module.
//!
//! Check the `glium-experimental` example to see how the backend is used.

use std::cell::RefCell;
use std::rc::Rc;

use crate::core::{DrawCommand, DrawList, DrawMode, Mat4x4, TextureId, Vertex};

use glium::Surface;

use super::rusttype_glium::FontWrapper;

/// Conversion from a `nalgebra` matrix to a four by four float array.
/// The resulting matrix is transposed so it can be imported directly
/// in openGL.
fn raw_matrix(mat: &Mat4x4) -> [[f32; 4]; 4] {
    [
        [mat[(0, 0)], mat[(1, 0)], mat[(2, 0)], mat[(3, 0)]],
        [mat[(0, 1)], mat[(1, 1)], mat[(2, 1)], mat[(3, 1)]],
        [mat[(0, 2)], mat[(1, 2)], mat[(2, 2)], mat[(3, 2)]],
        [mat[(0, 3)], mat[(1, 3)], mat[(2, 3)], mat[(3, 3)]],
    ]
}

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
    debug_poly_mode: glium::draw_parameters::PolygonMode,
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
            debug_poly_mode: glium::draw_parameters::PolygonMode::Fill,
            debug_rendering: false,
            program,
            blank_texture,
            textures: Vec::new(),
            fonts: vec![Rc::new(RefCell::new(default_font))],
        }
    }

    fn draw_parameters<'a>() -> glium::DrawParameters<'a> {
        glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            blend: glium::Blend::alpha_blending(),
            line_width: Some(1.0),
            point_size: Some(1.0),
            ..Default::default()
        }
    }

    pub fn switch_debug_rendering(&mut self) {
        match self.debug_poly_mode {
            glium::draw_parameters::PolygonMode::Fill => {
                self.debug_poly_mode = glium::draw_parameters::PolygonMode::Line;
                self.debug_rendering = true;
            }
            glium::draw_parameters::PolygonMode::Line => {
                self.debug_poly_mode = glium::draw_parameters::PolygonMode::Point;
                self.debug_rendering = true;
            }
            glium::draw_parameters::PolygonMode::Point => {
                self.debug_poly_mode = glium::draw_parameters::PolygonMode::Fill;
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
        local_transform: Mat4x4,
        command: &DrawCommand,
    ) -> DrawResult {
        let vertex_buffer =
            glium::VertexBuffer::immutable(&self.display, &command.vertex_buffer.as_slice())
                .unwrap();
        let primitve_type = match command.draw_mode {
            DrawMode::Triangles => glium::index::PrimitiveType::TrianglesList,
            DrawMode::Lines => glium::index::PrimitiveType::LinesList,
            DrawMode::Points => glium::index::PrimitiveType::Points,
        };
        let index_buffer = glium::IndexBuffer::immutable(
            &self.display,
            primitve_type,
            &command.index_buffer.as_slice(),
        )
        .unwrap();

        let mut draw_parameters = Self::draw_parameters();

        draw_parameters.polygon_mode = if !self.debug_rendering {
            match command.draw_mode {
                DrawMode::Triangles => glium::draw_parameters::PolygonMode::Fill,
                DrawMode::Lines => glium::draw_parameters::PolygonMode::Line,
                DrawMode::Points => glium::draw_parameters::PolygonMode::Point,
            }
        } else {
            self.debug_poly_mode
        };

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
                perspective_view: raw_matrix(&global_transform),
                model: raw_matrix(&(local_transform * command.uniforms.model_matrix)),
                t: if self.debug_rendering { &self.blank_texture } else { texture },
            };

            frame.draw(
                &vertex_buffer,
                &index_buffer,
                &self.program,
                &uniforms,
                &draw_parameters,
            )
        } else {
            let uniforms = glium::uniform! {
                perspective_view: raw_matrix(&global_transform),
                model: raw_matrix(&(local_transform * command.uniforms.model_matrix)),
                t: &self.blank_texture,
            };

            frame.draw(
                &vertex_buffer,
                &index_buffer,
                &self.program,
                &uniforms,
                &draw_parameters,
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
        local_transform: Mat4x4,
        list: &DrawList,
    ) -> DrawResult {
        list.commands.iter().try_for_each(|command| {
            self.draw_command(frame, global_transform, local_transform, command)
        })?;
        let transform = local_transform * list.list_transform;
        list.list
            .iter()
            .try_for_each(|list| self.draw_list(frame, global_transform, transform, list))
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
    pub fn get_font(&mut self, id: usize) -> Rc<RefCell<FontWrapper>> {
        self.fonts[id].clone()
    }
}

// Useful type abbreviations
type DrawResult = Result<(), glium::DrawError>;
type Frame = glium::Frame;
type RawTexture<'a> = glium::texture::RawImage2d<'a, u8>;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_array() {
        let matrix = nalgebra::Matrix4::new(
            1., 0., 0., 10., 0., 1., 0., 20., 0., 0., 1., 30., 0., 0., 0., 1.,
        );
        let array = [
            [1., 0., 0., 0.],
            [0., 1., 0., 0.],
            [0., 0., 1., 0.],
            [10., 20., 30., 1.],
        ];
        assert_eq!(raw_matrix(&matrix), array);
    }
}
