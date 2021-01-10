use crate::core::*;

pub struct FrameCounter {
    count: u32,
    count_next_frame: bool,
}
impl FrameCounter {
    pub fn new() -> Self {
        Self {
            count: 0,
            count_next_frame: true,
        }
    }
    pub fn count_next(mut self, cond: bool) -> Self {
        self.count_next_frame = cond;
        self
    }
}
impl Default for FrameCounter {
    fn default() -> Self {
        Self::new()
    }
}
impl WidgetBuilder for FrameCounter {
    type AchievedType = FrameCounter;
    type BuildFeedback = u32;
    fn update(self, _metadata: &NodeMetadata, widget: &mut Self::AchievedType) {
        if self.count_next_frame {
            widget.count += 1;
        }
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
            let node_bis = node_ref;
            let mut node = node_bis.borrow_mut();
            let (_, content) = node.borrow_parts();
            let window = content
                .as_any_mut()
                .downcast_mut::<Self::AchievedType>()
                .unwrap();
            window.count
        }
    }
}
impl WidgetLogic for FrameCounter {}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{loc, widget};
    use widget::*;
    #[test]
    fn frame_counter_1() {
        let mut ui = Interface::new();
        for i in 0..12 {
            ui.new_frame();
            WindowBuilder::new(move |ui| {
                PaddingBuilder::new((0.2, 0.2), FrameCounter::new()).build(loc!(), ui.clone());
                FrameCounter::new().build(loc!(), ui.clone());
                assert_eq!(i, FrameCounter::new().build(loc!(), ui.clone()));
                FrameCounter::new().build(loc!(), ui.clone());
            })
            .build(loc!(), ui.root.clone());
            ui.end_frame();
            ui.generate_layout();
        }
    }
    #[test]
    fn frame_counter_2() {
        let mut ui = Interface::new();
        for i in 0..12 {
            ui.new_frame();
            WindowBuilder::new(move |ui| {
                PaddingBuilder::new((0.2, 0.2), FrameCounter::new()).build(loc!(), ui.clone());
                FrameCounter::new();
                let mut fake_loc = loc!();
                fake_loc = CodeLocation(fake_loc.0, fake_loc.1 * i, fake_loc.2 + i);
                assert_eq!(0, FrameCounter::new().build(fake_loc, ui.clone()));
                FrameCounter::new().build(loc!(), ui.clone());
            })
            .build(loc!(), ui.root.clone());
            ui.end_frame();
            ui.generate_layout();
        }
    }

    #[test]
    fn frame_counter_with_padding_1() {
        let mut ui = Interface::new();
        for i in 0..12 {
            ui.new_frame();
            WindowBuilder::new(move |ui| {
                PaddingBuilder::new((0.2, 0.2), FrameCounter::new()).build(loc!(), ui.clone());
                FrameCounter::new().build(loc!(), ui.clone());
                assert_eq!(
                    i,
                    PaddingBuilder::new((0.2, 0.2), FrameCounter::new()).build(loc!(), ui.clone())
                );
                FrameCounter::new().build(loc!(), ui.clone());
            })
            .build(loc!(), ui.root.clone());
            ui.end_frame();
            ui.generate_layout();
        }
    }
    #[test]
    fn frame_counter_with_padding_2() {
        let mut ui = Interface::new();
        for i in 0..12 {
            ui.new_frame();
            WindowBuilder::new(move |ui| {
                PaddingBuilder::new((0.2, 0.2), FrameCounter::new()).build(loc!(), ui.clone());
                FrameCounter::new();
                let mut fake_loc = loc!();
                fake_loc = CodeLocation(fake_loc.0, fake_loc.1 * i, fake_loc.2 + i);
                assert_eq!(
                    0,
                    PaddingBuilder::new((0.2, 0.2), FrameCounter::new())
                        .build(fake_loc, ui.clone())
                );
                FrameCounter::new().build(loc!(), ui.clone());
            })
            .build(loc!(), ui.root.clone());
            ui.end_frame();
            ui.generate_layout();
        }
    }
}
