use nalgebra::Point3;

use crate::core::{
    CodeLocation, ComponentId, DrawCommand, DrawList, DrawMode, NodeMetadata, NodeReference,
    TextureId, Uniforms, Vertex, WidgetBuilder, WidgetLogic,
};

pub struct FrameCounter {
    count: u32,
}

impl FrameCounter {
    pub fn new() -> Self {
        Self { count: 0 }
    }

    /*pub fn position(mut self, pos: (f32, f32, f32)) -> Self {
        self.position = pos;
        self
    }*/
}

impl WidgetBuilder for FrameCounter {
    type AchievedType = FrameCounter;
    type BuildFeedback = u32;

    fn update(self, _metadata: &NodeMetadata, widget: &mut Self::AchievedType) {
        widget.count = widget.count + 1
    }

    fn create(self) -> Self::AchievedType {
        self
    }

    fn build(self, loc: CodeLocation, parent: NodeReference) -> Self::BuildFeedback {
        let id = ComponentId::new::<Self::AchievedType>(loc);
        let node_ref = parent
            .borrow_mut()
            .query::<Self::AchievedType>(id)
            .update(self);

        {
            let node_bis = node_ref.clone();
            let mut node = node_bis.borrow_mut();
            let (_, content) = node.borrow_parts();
            let window = content
                .as_any_mut()
                .downcast_mut::<Self::AchievedType>()
                .unwrap();
            window
                .count
        }
    }
}

impl WidgetLogic for FrameCounter {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::*;
    use crate::*;
    use widget::*;

    #[test]
    fn test_frame_counter_1() {
        let mut ui = Interface::new();
        for i in 0..12 {
            ui.new_frame();
            WindowBuilder::new(move |ui| {
                PaddingBuilder::new((0.2, 0.2), Button::new("label not displayed".to_string()))
                    .build(loc!(), ui.clone());
                Button::new("label not displayed".to_string()).build(loc!(), ui.clone());

                assert_eq!(i, FrameCounter::new().build(loc!(), ui.clone()));
                Button::new("label not displayed".to_string())
                    .color((1., 0., 0., 0.5))
                    .color((1., 1., 1., 1.))
                    .build(loc!(), ui.clone());
            })
            .build(loc!(), ui.root.clone());

            ui.end_frame();
            ui.generate_layout();
        }
    }
    #[test]
    fn test_frame_counter_2() {
        let mut ui = Interface::new();
        for i in 0..12 {
            ui.new_frame();
            WindowBuilder::new(move |ui| {
                PaddingBuilder::new((0.2, 0.2), Button::new("label not displayed".to_string()))
                    .build(loc!(), ui.clone());
                Button::new("label not displayed".to_string()).build(loc!(), ui.clone());
                let mut fake_loc = loc!();
                fake_loc = CodeLocation(fake_loc.0, fake_loc.1 * i, fake_loc.2 + i);
                assert_eq!(0, FrameCounter::new().build(fake_loc, ui.clone()));
                Button::new("label not displayed".to_string())
                    .color((1., 0., 0., 0.5))
                    .color((1., 1., 1., 1.))
                    .build(loc!(), ui.clone());
            })
            .build(loc!(), ui.root.clone());

            ui.end_frame();
            ui.generate_layout();
        }
    }
}
