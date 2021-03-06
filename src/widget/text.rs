use std::cell::RefCell;
use std::rc::Rc;

use crate::core::*;

use nalgebra::{Point3, Translation3};

/// An editable text
pub struct TextBuilder<'a> {
    text: &'a mut String,
    font: Rc<RefCell<dyn FontAtlas>>,
    size: f32,
    text_color: (f32, f32, f32, f32),
}

const BACKGROUND_FOCUSED: (f32, f32, f32, f32) = (0.231, 0.294, 0.451, 1.);
const BACKGROUND: (f32, f32, f32, f32) = (0.161, 0.176, 0.216, 1.);
const TEXT: (f32, f32, f32, f32) = (1., 1., 1., 1.);

impl<'a> TextBuilder<'a> {
    pub fn new(text: &'a mut String, font: &Rc<RefCell<dyn FontAtlas>>) -> Self {
        Self {
            text,
            font: font.clone(),
            size: 1.0,
            text_color: TEXT,
        }
    }

    pub fn text_color(self, text_color: (f32, f32, f32, f32)) -> Self {
        Self { text_color, ..self }
    }

    pub fn size(self, size: f32) -> Self {
        Self { size, ..self }
    }
}

impl<'a> WidgetBuilder for TextBuilder<'a> {
    type AchievedType = Text;
    type UpdateFeedback = ();
    type BuildFeedback = ();

    fn update(
        self,
        _metadata: &NodeMetadata,
        widget: &mut Self::AchievedType,
    ) -> Self::UpdateFeedback {
        self.text.clone_from(&widget.text);
        widget.size = self.size;
        widget.text_color = self.text_color;
    }

    fn create(self) -> Self::AchievedType {
        Text {
            text: self.text.clone(),
            font: self.font,
            size: self.size,
            text_color: self.text_color,
        }
    }

    fn build(self, loc: CodeLocation, parent: &NodeReference) -> Self::BuildFeedback {
        let id = ComponentId::new::<Self::AchievedType>(loc);

        parent.query::<Self::AchievedType>(id).update(self);
    }
}

/// A basic widget that can display a text given a font
pub struct Text {
    text: String,
    font: Rc<RefCell<dyn FontAtlas>>,
    size: f32,
    text_color: (f32, f32, f32, f32),
}

impl WidgetLogic for Text {
    fn layout(&mut self, query: &LayoutQuery) -> LayoutResponse {
        let (mut width, height) = if let Some(max_width) = query.available_space.0 {
            self.font
                .borrow()
                .multiline_size_of(self.text.as_str(), self.size, max_width)
        } else {
            self.font.borrow().size_of(self.text.as_str(), self.size)
        };

        if let Some(available_height) = query.available_space.1 {
            if available_height <= height {
                return LayoutResponse {
                    size: (0., 0.),
                    status: (LayoutStatus::Ok, LayoutStatus::WontDisplay),
                };
            }
        }

        if let Objective::Maximize = query.objectives.0 {
            if let Some(max_width) = query.available_space.0 {
                width = width.max(max_width);
            }
        }

        LayoutResponse {
            size: (width, height),
            status: (
                if width > query.available_space.0.unwrap_or(std::f32::INFINITY) {
                    LayoutStatus::Inconsistencies
                } else {
                    LayoutStatus::Ok
                },
                LayoutStatus::Ok,
            ),
        }
    }

    fn draw(&self, metadata: &NodeMetadata) -> DrawList {
        let background_command = quad(
            metadata.size.0,
            metadata.size.1,
            None,
            if metadata.is_focused() {
                BACKGROUND_FOCUSED
            } else {
                BACKGROUND
            },
            metadata.transform,
        );

        let text_command = draw_multiline_text(
            self.text.as_str(),
            &self.font,
            self.size,
            metadata.size.0,
            metadata.size.1,
            self.text_color,
            (metadata.transform * Translation3::new(0., 0., 0.01)).to_homogeneous(),
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

    fn send_event(&mut self, metadata: &mut NodeMetadata, event: &Event) -> EventResponse {
        match event {
            Event::MouseButtonPressed(MouseButton::Left)
            | Event::MouseButtonPressed(MouseButton::Touch) => {
                metadata.request_focus();
                EventResponse::Registered
            }
            Event::Character(c) => {
                match c {
                    '\u{8}' => {
                        self.text.pop();
                    }
                    _ if *c != '\u{7f}' => self.text.push(*c),
                    _ => {}
                }
                EventResponse::Registered
            }
            _ => EventResponse::Pass,
        }
    }
}
