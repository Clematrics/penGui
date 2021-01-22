use std::cell::RefCell;
use std::rc::Rc;

use nalgebra::{Point3, Translation3};

use crate::core::*;

/// A basic, clickable, button, which can be customised with a label, a color and a texture
/// at the same time
pub struct Button {
    label: String,
    color: (f32, f32, f32, f32),
    text_color: (f32, f32, f32, f32),
    font: Rc<RefCell<dyn FontAtlas>>,
    font_size: f32,
    pressed: bool,
    texture: Option<TextureId>,
}

const BACKGROUND: (f32, f32, f32, f32) = (0.231, 0.294, 0.451, 1.);
const TEXT: (f32, f32, f32, f32) = (1., 1., 1., 1.);

impl Button {
    pub fn new(label: String, font: &Rc<RefCell<dyn FontAtlas>>) -> Self {
        Self {
            label,
            color: BACKGROUND,
            text_color: TEXT,
            font: font.clone(),
            font_size: 1.,
            pressed: false,
            texture: None,
        }
    }

    pub fn font_size(self, font_size: f32) -> Self {
        Self { font_size, ..self }
    }

    pub fn color(self, color: (f32, f32, f32, f32)) -> Self {
        Self { color, ..self }
    }

    pub fn text_color(self, text_color: (f32, f32, f32, f32)) -> Self {
        Self { text_color, ..self }
    }

    pub fn texture(self, texture_id: TextureId) -> Self {
        Self {
            texture: Some(texture_id),
            ..self
        }
    }
}

impl WidgetBuilder for Button {
    type AchievedType = Button;
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

impl WidgetLogic for Button {
    fn layout(&mut self, query: &LayoutQuery) -> LayoutResponse {
        let (label_width, label_height) = self
            .font
            .borrow()
            .size_of(self.label.as_str(), self.font_size);

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
        let size = metadata.size;

        let background_command = quad(size.0, size.1, self.texture, self.color, metadata.transform);

        let text_command = draw_text(
            self.label.as_str(),
            &self.font,
            self.font_size,
            self.text_color,
            (metadata.transform * Translation3::new(PADDING, PADDING, 0.01)).to_homogeneous(),
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
            Event::MouseButtonPressed(MouseButton::Left)
            | Event::MouseButtonPressed(MouseButton::Touch) => {
                self.pressed = true;
                EventResponse::Registered
            }
            _ => EventResponse::Pass,
        }
    }
}
