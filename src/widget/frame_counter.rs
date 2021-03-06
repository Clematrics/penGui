use crate::core::*;

/// A widget used mainly to test the correctness of the library's algorithms
/// or to serve as a placeholder. It does not draw anything nor react to any event.
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
    pub fn count_next(self, cond: bool) -> Self {
        Self {
            count_next_frame: cond,
            ..self
        }
    }
}
impl Default for FrameCounter {
    fn default() -> Self {
        Self::new()
    }
}
impl WidgetBuilder for FrameCounter {
    type AchievedType = FrameCounter;
    type UpdateFeedback = u32;
    type BuildFeedback = u32;

    fn update(
        self,
        _metadata: &NodeMetadata,
        widget: &mut Self::AchievedType,
    ) -> Self::UpdateFeedback {
        if self.count_next_frame {
            widget.count += 1;
        }
        widget.count
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
                PaddingBuilder::new((0.2, 0.2), FrameCounter::new()).build(loc!(), &ui);
                FrameCounter::new().build(loc!(), &ui);
                assert_eq!(i, FrameCounter::new().build(loc!(), &ui));
                FrameCounter::new().build(loc!(), &ui);
            })
            .build(loc!(), &ui.root);
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
                PaddingBuilder::new((0.2, 0.2), FrameCounter::new()).build(loc!(), &ui);
                FrameCounter::new();
                let mut fake_loc = loc!();
                fake_loc = CodeLocation(fake_loc.0, fake_loc.1 * i, fake_loc.2 + i);
                assert_eq!(0, FrameCounter::new().build(fake_loc, &ui));
                FrameCounter::new().build(loc!(), &ui);
            })
            .build(loc!(), &ui.root);
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
                PaddingBuilder::new((0.2, 0.2), FrameCounter::new()).build(loc!(), &ui);
                FrameCounter::new().build(loc!(), &ui);
                assert_eq!(
                    i,
                    PaddingBuilder::new((0.2, 0.2), FrameCounter::new()).build(loc!(), &ui)
                );
                FrameCounter::new().build(loc!(), &ui);
            })
            .build(loc!(), &ui.root);
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
                PaddingBuilder::new((0.2, 0.2), FrameCounter::new()).build(loc!(), &ui);
                FrameCounter::new();
                let mut fake_loc = loc!();
                fake_loc = CodeLocation(fake_loc.0, fake_loc.1 * i, fake_loc.2 + i);
                assert_eq!(
                    0,
                    PaddingBuilder::new((0.2, 0.2), FrameCounter::new()).build(fake_loc, &ui)
                );
                FrameCounter::new().build(loc!(), &ui);
            })
            .build(loc!(), &ui.root);
            ui.end_frame();
            ui.generate_layout();
        }
    }
}
