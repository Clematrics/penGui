use std::cell::RefCell;
use std::rc::Weak;

use crate::core::*;

/// A basic widget that can display a text given a font
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

impl WidgetLogic for Text {
    fn layout(&mut self, query: &LayoutQuery) -> LayoutResponse {
        let font = self
            .font
            .upgrade()
            .expect("A font is not owned anymore by the backend");

        let (width, _height) = font.borrow().size_of(self.text.as_str());

        if let Some(available_height) = query.available_space.1 {
            if available_height <= 1. {
                return LayoutResponse {
                    size: (0., 0.),
                    status: (LayoutStatus::Ok, LayoutStatus::WontDisplay),
                };
            }
        }

        // let font_height = {
        //     let metrics = font.borrow().get_vertical_metrics();
        //     metrics.ascent + metrics.descent
        // };

        // let (height, factor) = match query.available_space.1 {
        //     None => (font_height, 1.),
        //     Some(height) => (height, height / font_height),
        // };

        // let mut width = 0.0;
        // let mut last_char = None;
        // self.text.chars().for_each(|c| {
        //     let mut font = font.borrow_mut();

        //     let CharacterInfo { advance_width, .. } = font.char_info(c, last_char);

        //     width += advance_width * factor;
        //     last_char = Some(c);
        // });

        LayoutResponse {
            size: (width, 1.),
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
            self.font
                .upgrade()
                .expect("A font is not owned anymore by the backend"),
            color,
            nalgebra::Translation3::from(nalgebra::Vector3::new(x, y, z)).to_homogeneous(),
        );

        let mut list = DrawList::new();
        list.commands.push(text_command);
        list
    }
}
