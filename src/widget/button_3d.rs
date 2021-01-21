use std::cell::RefCell;
use std::rc::Rc;

use nalgebra::{Point3, Translation3, Vector2, Vector3};

use crate::core::*;

/// A basic, clickable, button, which can be customised with a label, a color and a texture
/// at the same time
pub struct Button3D {
    label: String,
    extrude: f32,
    color: (f32, f32, f32, f32),
    font: Rc<RefCell<dyn FontAtlas>>,
    pressed: bool,
    texture: Option<TextureId>,
}

impl Button3D {
    pub fn new(label: String, font: &Rc<RefCell<dyn FontAtlas>>) -> Self {
        Self {
            label,
            extrude: 0.,
            color: (0., 0.4, 1., 1.),
            font: font.clone(),
            pressed: false,
            texture: None,
        }
    }

    pub fn extrude(self, extrude: f32) -> Self {
        Self { extrude, ..self }
    }

    pub fn color(self, color: (f32, f32, f32, f32)) -> Self {
        Self { color, ..self }
    }

    pub fn texture(self, texture_id: TextureId) -> Self {
        Self {
            texture: Some(texture_id),
            ..self
        }
    }
}

impl WidgetBuilder for Button3D {
    type AchievedType = Button3D;
    type UpdateFeedback = bool;
    type BuildFeedback = bool;

    fn update(
        self,
        _metadata: &NodeMetadata,
        widget: &mut Self::AchievedType,
    ) -> Self::UpdateFeedback {
        let pressed = widget.pressed;

        widget.pressed = false;
        widget.label = self.label;
        widget.extrude = self.extrude;
        widget.color = self.color;

        pressed
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

impl WidgetLogic for Button3D {
    fn layout(&mut self, query: &LayoutQuery) -> LayoutResponse {
        let (label_width, label_height) = self.font.borrow().size_of(self.label.as_str(), 1.);

        let mut width = label_width + 2. * PADDING;
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
        let text_color = (
            self.color.0 / 1.5,
            self.color.1 / 1.5,
            self.color.2 / 1.5,
            self.color.3,
        );
        let size = metadata.size;

        let mesh_command = {
            let top_points = [
                Vector3::new(0., 0., self.extrude),
                Vector3::new(size.0, 0., self.extrude),
                Vector3::new(0., size.1, self.extrude),
                Vector3::new(size.0, size.1, self.extrude),
            ];
            let bottom_points = [
                Vector3::new(0., 0., 0.),
                Vector3::new(size.0, 0., 0.),
                Vector3::new(0., size.1, 0.),
                Vector3::new(size.0, size.1, 0.),
            ];
            let uv = [
                Vector2::new(0., 0.),
                Vector2::new(1., 0.),
                Vector2::new(0., 1.),
                Vector2::new(1., 1.),
            ];
            let vertex_buffer: Vec<Vertex> = top_points
                .iter()
                .zip(uv.iter())
                .map(|(&position, &tex_uv)| Vertex {
                    position,
                    color: self.color,
                    tex_uv,
                })
                .chain(bottom_points.iter().zip(uv.iter().skip(1).cycle()).map(
                    |(&position, &tex_uv)| Vertex {
                        position,
                        color: self.color,
                        tex_uv,
                    },
                ))
                .collect();
            let index_buffer = vec![
                // Front face
                0, 1, 2, 1, 2, 3, // Top face
                0, 1, 4, 1, 4, 5, // Bottom face
                2, 3, 6, 3, 6, 7, // Left face
                0, 2, 4, 2, 4, 6, // Right face
                1, 3, 5, 3, 5, 7,
            ];
            let mut uniforms = Uniforms::new();
            uniforms.texture = self.texture;
            uniforms.model_matrix = metadata.transform.to_homogeneous();

            DrawCommand {
                vertex_buffer,
                index_buffer,
                draw_mode: DrawMode::Triangles,
                uniforms,
            }
        };

        let text_command = draw_text(
            self.label.as_str(),
            &self.font,
            1.,
            text_color,
            (metadata.transform * Translation3::new(PADDING, PADDING, self.extrude + 0.001))
                .to_homogeneous(),
        );

        let mut list = DrawList::new();
        list.commands.push(mesh_command);
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
            // Top points
            Point3::new(0., 0., self.extrude),
            Point3::new(size.0, 0., self.extrude),
            Point3::new(0., size.1, self.extrude),
            Point3::new(size.0, size.1, self.extrude),
            // Bottom points
            Point3::new(0., 0., 0.),
            Point3::new(size.0, 0., 0.),
            Point3::new(0., size.1, 0.),
            Point3::new(size.0, size.1, 0.),
        ];
        [
            // Front face
            [points[0], points[1], points[2]],
            [points[1], points[2], points[3]],
            // Top face
            [points[0], points[1], points[4]],
            [points[1], points[4], points[5]],
            // Bottom face
            [points[2], points[3], points[6]],
            [points[3], points[6], points[7]],
            // Left face
            [points[0], points[2], points[4]],
            [points[2], points[4], points[6]],
            // Right face
            [points[1], points[3], points[5]],
            [points[3], points[5], points[7]],
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
            Event::MouseButtonPressed(MouseButton::Left)
            | Event::MouseButtonPressed(MouseButton::Touch) => {
                self.pressed = true;
                EventResponse::Registered
            }
            _ => EventResponse::Pass,
        }
    }
}
