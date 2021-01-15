use std::cell::RefCell;
use std::rc::Rc;

use crate::core::*;

use nalgebra::{Point3, Translation3};

/// An editable text
pub struct LabelBuilder<'a> {
    text: &'a str,
    font: Rc<RefCell<dyn FontAtlas>>,
    size: f32,
    color: (f32, f32, f32, f32),
}

impl<'a> LabelBuilder<'a> {
    pub fn new(text: &'a str, font: &Rc<RefCell<dyn FontAtlas>>) -> Self {
        Self {
            text,
            font: font.clone(),
            size: 1.0,
            color: (1.0, 1.0, 1.0, 1.0),
        }
    }

    pub fn color(self, color: (f32, f32, f32, f32)) -> Self {
        Self { color, ..self }
    }

    pub fn size(self, size: f32) -> Self {
        Self { size, ..self }
    }
}

impl<'a> WidgetBuilder for LabelBuilder<'a> {
    type AchievedType = Label;
    type BuildFeedback = ();

    fn update(self, _metadata: &NodeMetadata, widget: &mut Self::AchievedType) {
        widget.text = self.text.to_string();
        widget.size = self.size;
        widget.color = self.color;
    }

    fn create(self) -> Self::AchievedType {
        Label {
            text: self.text.to_string(),
            font: self.font,
            size: self.size,
            color: self.color,
        }
    }

    fn build(self, loc: CodeLocation, parent: &NodeReference) -> Self::BuildFeedback {
        let id = ComponentId::new::<Self::AchievedType>(loc);

        parent
            .borrow_mut()
            .query::<Self::AchievedType>(id)
            .update(self);
    }
}

/// A basic widget that can display a text given a font
pub struct Label {
    text: String,
    font: Rc<RefCell<dyn FontAtlas>>,
    size: f32,
    color: (f32, f32, f32, f32),
}

impl WidgetLogic for Label {
    fn layout(&mut self, query: &LayoutQuery) -> LayoutResponse {
        let (width, height) = self.font.borrow().size_of(self.text.as_str(), self.size);

        if let Some(available_height) = query.available_space.1 {
            if available_height <= height {
                return LayoutResponse {
                    size: (0., 0.),
                    status: (LayoutStatus::Ok, LayoutStatus::WontDisplay),
                };
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
        #![allow(clippy::many_single_char_names)]
        let (r, g, b, a) = self.color;
        let color = [r, g, b, a];
        let (x, y, z) = metadata.position;

        let text_command = draw_text(
            self.text.as_str(),
            &self.font,
            self.size,
            color,
            nalgebra::Translation3::from(nalgebra::Vector3::new(x, y, z)).to_homogeneous(),
        );

        let mut list = DrawList::new();
        list.commands.push(text_command);
        list
    }
}
