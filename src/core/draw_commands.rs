/// Structure used to store all the information about vertices.
/// It holds:
/// - a position in the 3D space
/// - a color
/// - UV coordinates in case there is a texture
#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 4],
    pub tex_uv: [f32; 2],
}

/// Internal type for a 4x4 matrix
// pub type Mat4x4 = [[f32; 4]; 4];
// pub use nalgebra::Matrix4 as Mat4x4;
pub type Mat4x4 = nalgebra::Matrix4<f32>;
/// Type to hold texture identifiers
#[derive(Copy, Clone)]
pub enum TextureId {
    Texture(usize),
    Font(usize),
}

/// A structure to store uniforms needed for each draw command
/// It holds:
/// - a mandatory transformation matrix, used as the model matrix
/// (first transformation applied on the vertices)
/// - an optional texture to be used on the object of the draw command
pub struct Uniforms {
    pub model_matrix: Mat4x4,
    pub texture: Option<TextureId>,
}

impl Uniforms {
    /// Creates new uniforms, with a unit model matrix, and no texture
    pub fn new() -> Self {
        Self {
            model_matrix: Mat4x4::identity(),
            texture: None,
        }
    }
}

impl Default for Uniforms {
    fn default() -> Self {
        Self::new()
    }
}

/// The drawing mode of a `DrawCommand`
/// - `Triangles` allows to build filled triangles with disjoint groups of three vertices
/// - `Lines` (not implemented yet) allows to build lines with disjoint pair of two vertices
/// - `Points` (not implemented yet) allows to draw points, one for each vertex
pub enum DrawMode {
    Triangles,
    Lines,
    Points,
    // ... TODO: to complete with TriangleFan, TriangleStrip, LineStrip, LineLoop
}

/// Type describing how to draw
/// It holds:
/// - a list of vertices in the 3D space
/// - a list of indices to describe how to join points if needed
/// - a draw mode to describe how is organized the list of indices
/// - uniforms to have more options during the transformation and rendering of the object
pub struct DrawCommand {
    pub vertex_buffer: Vec<Vertex>,
    pub index_buffer: Vec<u32>,
    pub draw_mode: DrawMode,
    pub uniforms: Uniforms,
}

/// A structure
pub struct DrawList {
    pub commands: Vec<DrawCommand>,
    pub list: Vec<DrawList>,
    pub list_transform: Mat4x4,
}

impl DrawList {
    /// Creates a new draw list with no sub-`DrawList` and no `DrawCommand`
    pub fn new() -> DrawList {
        DrawList {
            commands: Vec::new(),
            list: Vec::new(),
            list_transform: Mat4x4::identity(),
        }
    }
}

impl Default for DrawList {
    fn default() -> Self {
        Self::new()
    }
}
