use std::cell::RefCell;
use std::rc::Weak;

use nalgebra::Point3;

use crate::core::{
    CharacterInfo, CodeLocation, ComponentId, DrawCommand, DrawList, DrawMode, FontAtlas,
    NodeMetadata, NodeReference, Uniforms, Vertex, WidgetBuilder, WidgetLogic,
};

pub struct Text {
    text: String,
    font: Weak<RefCell<dyn FontAtlas>>,
    color: (f32, f32, f32, f32),
}

impl Text {
    pub fn new(text: String, font: Weak<RefCell<dyn FontAtlas>>) -> Self {
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

        let count = self.text.chars().count();
        let mut vertex_buffer = Vec::with_capacity(count * 4);
        let mut index_buffer = Vec::<u32>::with_capacity(count * 6);
        let mut cursor = 0.;
        let mut last_char = None;
        self.text.chars().enumerate().for_each(|(i, c)| {
            let mut font = font.borrow_mut();

            let base = 4 * i;

            let CharacterInfo {
                texture_uv: (u, v),
                texture_size: (w, h),
                advance_width,
                top_left: (tx, ty),
                bottom_right: (bx, by),
                kerning,
            } = font.char_info(c, last_char);

            let ax = cursor + tx + kerning;
            let ay = ty;
            let bx = cursor + bx + kerning;
            let by = by;

            vertex_buffer.push(Vertex {
                position: [ax, ay, 0.],
                color,
                tex_uv: [u, v],
            });
            vertex_buffer.push(Vertex {
                position: [bx, ay, 0.],
                color,
                tex_uv: [u + w, v],
            });
            vertex_buffer.push(Vertex {
                position: [ax, by, 0.],
                color,
                tex_uv: [u, v + h],
            });
            vertex_buffer.push(Vertex {
                position: [bx, by, 0.],
                color,
                tex_uv: [u + w, v + h],
            });

            index_buffer.push(base as u32);
            index_buffer.push((base + 1) as u32);
            index_buffer.push((base + 2) as u32);
            index_buffer.push((base + 1) as u32);
            index_buffer.push((base + 2) as u32);
            index_buffer.push((base + 3) as u32);

            cursor += advance_width;
            last_char = Some(c);
        });

        let command = DrawCommand {
            vertex_buffer,
            index_buffer,
            draw_mode: DrawMode::Triangles,
            uniforms,
        };

        let mut list = DrawList::new();
        list.commands.push(command);
        list
    }
}
