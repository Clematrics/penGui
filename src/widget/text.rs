use std::cell::RefCell;
use std::rc::Weak;

use nalgebra::Point3;

use crate::core::{
    CharacterInfo, CodeLocation, ComponentId, DrawCommand, DrawList, DrawMode, FontAtlas,
    NodeMetadata, NodeReference, Uniforms, Vertex, WidgetBuilder, WidgetLogic,
};

pub struct Text {
    text: &'static str,
    font: Weak<RefCell<dyn FontAtlas>>,
    color: (f32, f32, f32, f32),
}

impl Text {
    pub fn new(text: &'static str, font: Weak<RefCell<dyn FontAtlas>>) -> Self {
        Self {
            text,
            font,
            color: (1.0, 1.0, 1.0, 1.0),
        }
    }

    pub fn color(mut self, color: (f32, f32, f32, f32)) -> Self {
        self.color = color;
        self
    }
}

impl WidgetBuilder for Text {
    type AchievedType = Text;
    type BuildFeedback = ();

    fn update(self, _metadata: &NodeMetadata, widget: &mut Self::AchievedType) {
        widget.text = self.text;
        widget.color = self.color;
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

impl WidgetLogic for Text {
    fn draw(&self, _metadata: &NodeMetadata, position: Point3<f32>, size: (f32, f32)) -> DrawList {
        #![allow(clippy::many_single_char_names)]
        let (r, g, b, a) = self.color;
        let color = [r, g, b, a];

        let mut uniforms = Uniforms::new();
        let (x, y, z) = (position.x, position.y, position.z);
        uniforms.model_matrix = to_array(
            &nalgebra::Translation3::from(nalgebra::Vector3::new(x, y, z)).to_homogeneous(),
        );

        let font = self
            .font
            .upgrade()
            .expect("A font is not owned anymore by the backend");
        uniforms.texture = Some(font.borrow().get_texture());

        let CharacterInfo {
            texture_uv: (u, v),
            size: (w, h),
        } = font.borrow_mut().char_info('c');

        let command = DrawCommand {
            vertex_buffer: vec![
                Vertex {
                    position: [-size.0 / 2., -size.1 / 2., 0.],
                    color,
                    tex_uv: [u, v],
                },
                Vertex {
                    position: [size.0 / 2., -size.1 / 2., 0.],
                    color,
                    tex_uv: [u + w, v],
                },
                Vertex {
                    position: [-size.0 / 2., size.1 / 2., 0.],
                    color,
                    tex_uv: [u, v + h],
                },
                Vertex {
                    position: [size.0 / 2., size.1 / 2., 0.],
                    color,
                    tex_uv: [u + w, v + h],
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
