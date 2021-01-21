use std::cell::RefCell;
use std::rc::Rc;

use nalgebra::{Point3, Translation3, Vector2, Vector3};

use crate::core::*;

/// A basic checkbox widget, with a label
pub struct CheckBox {
    label: String,
    background_color: (f32, f32, f32, f32),
    checked_color: (f32, f32, f32, f32),
    unchecked_color: (f32, f32, f32, f32),
    font: Rc<RefCell<dyn FontAtlas>>,
    checked: bool,
    texture: Option<TextureId>,
}

impl CheckBox {
    pub fn new(label: String, font: &Rc<RefCell<dyn FontAtlas>>) -> Self {
        Self {
            label,
            background_color: (0., 0.4, 1., 1.),
            checked_color: (0., 1., 0.4, 1.),
            unchecked_color: (0.4, 0., 1., 1.),
            font: font.clone(),
            checked: false,
            texture: None,
        }
    }

    pub fn background_color(self, background_color: (f32, f32, f32, f32)) -> Self {
        Self {
            background_color,
            ..self
        }
    }

    pub fn checked_color(self, checked_color: (f32, f32, f32, f32)) -> Self {
        Self {
            checked_color,
            ..self
        }
    }

    pub fn unchecked_color(self, unchecked_color: (f32, f32, f32, f32)) -> Self {
        Self {
            unchecked_color,
            ..self
        }
    }

    pub fn texture(self, texture_id: TextureId) -> Self {
        Self {
            texture: Some(texture_id),
            ..self
        }
    }
}

impl WidgetBuilder for CheckBox {
    type AchievedType = CheckBox;
    type UpdateFeedback = bool;
    type BuildFeedback = bool;

    fn update(
        self,
        _metadata: &NodeMetadata,
        widget: &mut Self::AchievedType,
    ) -> Self::UpdateFeedback {
        widget.label = self.label;
        widget.background_color = self.background_color;
        widget.checked_color = self.checked_color;
        widget.unchecked_color = self.unchecked_color;
        widget.checked
    }

    fn create(self) -> Self::AchievedType {
        self
    }

    fn build(self, loc: CodeLocation, parent: &NodeReference) -> Self::BuildFeedback {
        let id = ComponentId::new::<Self::AchievedType>(loc);
        let (_, feedback) = parent.query::<Self::AchievedType>(id).update(self);
        feedback
    }
}

const PADDING: f32 = 0.2;

impl WidgetLogic for CheckBox {
    fn layout(&mut self, query: &LayoutQuery) -> LayoutResponse {
        let (label_width, label_height) = self.font.borrow().size_of(self.label.as_str(), 1.);

        let box_size = label_height;
        let mut width = box_size + PADDING + label_width + 2. * PADDING;
        let mut height = label_height + 2. * PADDING;

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
        let background_color = self.background_color;

        let color = {
            if self.checked {
                self.checked_color
            } else {
                self.unchecked_color
            }
        };
        let size = metadata.size;

        let box_size = size.1 - 2. * PADDING;

        let background_command = {
            let mut uniforms = Uniforms::new();
            uniforms.model_matrix = metadata.transform.to_homogeneous();
            uniforms.texture = self.texture;

            DrawCommand {
                vertex_buffer: vec![
                    Vertex {
                        position: Vector3::new(0., 0., 0.),
                        color: background_color,
                        tex_uv: Vector2::new(0., 0.),
                    },
                    Vertex {
                        position: Vector3::new(size.0, 0., 0.),
                        color: background_color,
                        tex_uv: Vector2::new(1., 0.),
                    },
                    Vertex {
                        position: Vector3::new(0., size.1, 0.),
                        color: background_color,
                        tex_uv: Vector2::new(0., 1.),
                    },
                    Vertex {
                        position: Vector3::new(size.0, size.1, 0.),
                        color: background_color,
                        tex_uv: Vector2::new(1., 1.),
                    },
                    Vertex {
                        position: Vector3::new(PADDING, PADDING, 0.001),
                        color,
                        tex_uv: Vector2::new(0., 0.),
                    },
                    Vertex {
                        position: Vector3::new(PADDING + box_size, PADDING, 0.001),
                        color,
                        tex_uv: Vector2::new(1., 0.),
                    },
                    Vertex {
                        position: Vector3::new(PADDING, PADDING + box_size, 0.001),
                        color,
                        tex_uv: Vector2::new(0., 1.),
                    },
                    Vertex {
                        position: Vector3::new(PADDING + box_size, PADDING + box_size, 0.001),
                        color,
                        tex_uv: Vector2::new(1., 1.),
                    },
                ],
                index_buffer: vec![0, 1, 2, 1, 2, 3, 4, 5, 6, 5, 6, 7],
                draw_mode: DrawMode::Triangles,
                uniforms,
            }
        };

        let text_command = crate::core::draw_text(
            self.label.as_str(),
            &self.font,
            1.,
            color,
            (metadata.transform * Translation3::new(box_size + 2. * PADDING, PADDING, 0.001))
                .to_homogeneous(),
        );

        let mut list = DrawList::new();
        list.commands.push(background_command);
        list.commands.push(text_command);
        list
    }

    fn interaction_distance(
        &self,
        metadata: &NodeMetadata,
        ray: &Ray,
        self_node: NodeReference,
    ) -> Vec<(f32, NodeReference)> {
        let transformation = metadata.transform.inverse();
        let new_ray = Ray::new(ray.direction(), transformation * ray.origin());
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
        .map(|triangle| intersection(&new_ray, triangle))
        .filter_map(|opt| opt)
        .min_by(|d1, d2| d1.partial_cmp(d2).unwrap())
        .map(|d| vec![(d, self_node)])
        .unwrap_or_default()
    }

    fn send_event(&mut self, _metadata: &mut NodeMetadata, event: &Event) -> EventResponse {
        match event {
            Event::MouseButtonPressed(MouseButton::Left) => {
                self.checked = !self.checked;
                EventResponse::Registered
            }
            _ => EventResponse::Pass,
        }
    }
}
