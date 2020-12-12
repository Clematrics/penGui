use nalgebra::Point3;

use crate::core::{
    CodeLocation, ComponentId, DrawCommand, DrawList, DrawMode, NodeMetadata, NodeReference,
    TextureId, Uniforms, Vertex, WidgetBuilder, WidgetLogic,
};

pub struct Button {
    label: String,
    /*size: (f32, f32),*/
    color: (f32, f32, f32, f32),
    /*position: (f32, f32, f32),*/
    texture: Option<TextureId>,
}

impl Button {
    pub fn new(label: String) -> Self {
        Self {
            label,
            /*size: (0.75, 0.25),*/
            color: (0., 0.4, 1., 1.),
            /*position: (0., 0., 0.),*/
            texture: None,
        }
    }

    pub fn color(mut self, color: (f32, f32, f32, f32)) -> Self {
        self.color = color;
        self
    }

    /*pub fn position(mut self, pos: (f32, f32, f32)) -> Self {
        self.position = pos;
        self
    }*/

    pub fn texture(mut self, texture_id: TextureId) -> Self {
        self.texture = Some(texture_id);
        self
    }
}

impl WidgetBuilder for Button {
    type AchievedType = Button;
    type BuildFeedback = bool;

    fn update(self, _metadata: &NodeMetadata, old: &mut Self::AchievedType) {
        old.label = self.label;
        old.color = self.color;
    }

    fn create(self) -> Self::AchievedType {
        self
    }

    fn build(self, loc: CodeLocation, parent: NodeReference) -> Self::BuildFeedback {
        let id = ComponentId::new::<Self::AchievedType>(loc);

        parent
            .borrow_mut()
            .query::<Self::AchievedType>(id)
            .update(self);

        true
    }
}

fn to_array(mat: &nalgebra::Matrix4<f32>) -> [[f32; 4]; 4] {
    [
        [mat[(0, 0)], mat[(1, 0)], mat[(2, 0)], mat[(3, 0)]],
        [mat[(0, 1)], mat[(1, 1)], mat[(2, 1)], mat[(3, 1)]],
        [mat[(0, 2)], mat[(1, 2)], mat[(2, 2)], mat[(3, 2)]],
        [mat[(0, 3)], mat[(1, 3)], mat[(2, 3)], mat[(3, 3)]],
    ]
}

impl WidgetLogic for Button {
    fn draw(&self, _metadata: &NodeMetadata, position: Point3<f32>, size: (f32, f32)) -> DrawList {
        #![allow(clippy::many_single_char_names)]
        let (r, g, b, a) = self.color;
        let color = [r, g, b, a];

        let mut uniforms = Uniforms::new();
        let (x, y, z) = (position.x, position.y, position.z);
        uniforms.model_matrix = to_array(
            &nalgebra::Translation3::from(nalgebra::Vector3::new(x, y, z)).to_homogeneous(),
        );
        uniforms.texture = self.texture;

        let command = DrawCommand {
            vertex_buffer: vec![
                Vertex {
                    position: [-size.0 / 2., -size.1 / 2., 0.],
                    color,
                    tex_uv: [0., 0.],
                },
                Vertex {
                    position: [size.0 / 2., -size.1 / 2., 0.],
                    color,
                    tex_uv: [1., 0.],
                },
                Vertex {
                    position: [-size.0 / 2., size.1 / 2., 0.],
                    color,
                    tex_uv: [0., 1.],
                },
                Vertex {
                    position: [size.0 / 2., size.1 / 2., 0.],
                    color,
                    tex_uv: [1., 1.],
                },
            ],
            index_buffer: vec![0, 1, 2, 1, 2, 3],
            draw_mode: DrawMode::Triangles,
            uniforms,
        };

        let mut list = DrawList::new();
        list.commands.push(command);
        list
    }
}
