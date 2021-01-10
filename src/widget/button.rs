use std::cell::RefCell;
use std::rc::Weak;

use nalgebra::{Point3, Translation3, Vector3};

use crate::core::*;

pub struct Button {
    label: String,
    color: (f32, f32, f32, f32),
    font: Weak<RefCell<dyn FontAtlas>>,
    pressed: bool,
    texture: Option<TextureId>,
}

impl Button {
    pub fn new(label: String, font: Weak<RefCell<dyn FontAtlas>>) -> Self {
        Self {
            label,
            color: (0., 0.4, 1., 1.),
            font,
            pressed: false,
            texture: None,
        }
    }

    pub fn color(mut self, color: (f32, f32, f32, f32)) -> Self {
        self.color = color;
        self
    }

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

        let node = parent
            .borrow_mut()
            .query::<Self::AchievedType>(id)
            .update(self);

        {
            let mut node = node.borrow_mut();
            let (_, button) = node.borrow_parts();
            let button = button
                .as_any_mut()
                .downcast_mut::<Self::AchievedType>()
                .unwrap();
            let pressed = button.pressed;
            button.pressed = false;
            pressed
        }
    }
}

const PADDING: f32 = 0.2;

impl WidgetLogic for Button {
    fn layout(&mut self, query: &LayoutQuery) -> LayoutResponse {
        let (label_width, _label_height) = self
            .font
            .upgrade()
            .expect("A font is not owned anymore by the backend")
            .borrow()
            .size_of(self.label.as_str());

        let mut width = label_width + 2. * PADDING;
        let mut height = 1. + 2. * PADDING;

        let (available_width, available_height) = (
            query.available_space.0.unwrap_or(width),
            query.available_space.1.unwrap_or(height),
        );

        let x_status = if width <= available_width {
            LayoutStatus::Ok
        } else {
            LayoutStatus::Inconsistencies
        };
        let y_status = if height <= available_height {
            LayoutStatus::Ok
        } else {
            LayoutStatus::Inconsistencies
        };

        match query.objectives.0 {
            Objective::Maximize => {
                width = available_width;
            }
            Objective::Minimize | Objective::None => {
                if width > available_width {
                    width = available_width;
                }
            }
        }
        match query.objectives.1 {
            Objective::Maximize => {
                height = available_height;
            }
            Objective::Minimize | Objective::None => {
                if height > available_height {
                    height = available_height;
                }
            }
        }

        LayoutResponse {
            size: (width, height),
            status: (x_status, y_status),
        }
    }

    fn draw(&self, metadata: &NodeMetadata) -> DrawList {
        #![allow(clippy::many_single_char_names)]
        let (r, g, b, a) = self.color;
        let color = [r, g, b, a];
        let size = metadata.size;
        let (x, y, z) = metadata.position;

        let background_command = {
            let mut uniforms = Uniforms::new();
            uniforms.model_matrix =
                nalgebra::Translation3::from(nalgebra::Vector3::new(x, y, z)).to_homogeneous();
            uniforms.texture = self.texture;

            DrawCommand {
                vertex_buffer: vec![
                    Vertex {
                        position: [0., 0., 0.],
                        color,
                        tex_uv: [0., 0.],
                    },
                    Vertex {
                        position: [size.0, 0., 0.],
                        color,
                        tex_uv: [1., 0.],
                    },
                    Vertex {
                        position: [0., size.1, 0.],
                        color,
                        tex_uv: [0., 1.],
                    },
                    Vertex {
                        position: [size.0, size.1, 0.],
                        color,
                        tex_uv: [1., 1.],
                    },
                ],
                index_buffer: vec![0, 1, 2, 1, 2, 3],
                draw_mode: DrawMode::Triangles,
                uniforms,
            }
        };

        let text_command = crate::core::draw_text(
            self.label.as_str(),
            self.font
                .upgrade()
                .expect("A font is not owned anymore by the backend"),
            color,
            nalgebra::Translation3::from(nalgebra::Vector3::new(x, y, z + 0.001)).to_homogeneous(),
        );

        let mut list = DrawList::new();
        list.commands.push(background_command);
        list.commands.push(text_command);
        list
    }

    fn interaction_distance(
        &self,
        metadata: &NodeMetadata,
        ray: &Vector3<f32>,
        origin: &Point3<f32>,
        self_node: NodeReference,
    ) -> Vec<(f32, NodeReference)> {
        let (x, y, z) = metadata.position;
        let transformation = Translation3::new(x, y, z);
        let new_origin = transformation * origin;
        let size = metadata.size;
        let points = [
            Point3::new(0., 0., 0.),
            Point3::new(size.0, 0., 0.),
            Point3::new(0., size.1, 0.),
            Point3::new(size.0, size.1, 0.),
        ];
        [
            [points[0], points[1], points[2]],
            [points[1], points[2], points[3]],
        ]
        .iter()
        .map(|triangle| intersection(ray, &new_origin, triangle))
        .min_by(|d1, d2| d1.partial_cmp(d2).unwrap())
        .flatten()
        .map(|d| vec![(d, self_node)])
        .unwrap_or(vec![])
    }

    fn send_event(&mut self, _metadata: &mut NodeMetadata, event: &Event) -> EventResponse {
        match event {
            Event::MouseButtonPressed(MouseButton::Left) => {
                self.pressed = true;
                EventResponse::Registered
            }
            _ => return EventResponse::Pass,
        }
    }
}
