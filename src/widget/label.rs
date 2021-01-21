use std::cell::RefCell;
use std::rc::Rc;

use crate::core::*;

/// An editable text
pub struct LabelBuilder<'a> {
    text: &'a str,
    font: Rc<RefCell<dyn FontAtlas>>,
    size: f32,
    text_color: (f32, f32, f32, f32),
}

const TEXT: (f32, f32, f32, f32) = (1., 1., 1., 1.);

impl<'a> LabelBuilder<'a> {
    pub fn new(text: &'a str, font: &Rc<RefCell<dyn FontAtlas>>) -> Self {
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

impl<'a> WidgetBuilder for LabelBuilder<'a> {
    type AchievedType = Label;
    type UpdateFeedback = ();
    type BuildFeedback = ();

    fn update(
        self,
        _metadata: &NodeMetadata,
        widget: &mut Self::AchievedType,
    ) -> Self::UpdateFeedback {
        widget.text = self.text.to_string();
        widget.size = self.size;
        widget.text_color = self.text_color;
    }

    fn create(self) -> Self::AchievedType {
        Label {
            text: self.text.to_string(),
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
pub struct Label {
    text: String,
    font: Rc<RefCell<dyn FontAtlas>>,
    size: f32,
    text_color: (f32, f32, f32, f32),
}

impl WidgetLogic for Label {
    fn layout(&mut self, query: &LayoutQuery) -> LayoutResponse {
        let (width, height) = if let Some(max_width) = query.available_space.0 {
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
        let text_command = draw_multiline_text(
            self.text.as_str(),
            &self.font,
            self.size,
            metadata.size.0,
            metadata.size.1,
            self.text_color,
            metadata.transform.to_homogeneous(),
        );

        let mut list = DrawList::new();
        list.commands.push(text_command);
        list
    }
}
