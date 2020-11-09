use crate::core::{
    DrawCommand, DrawMode, InterfaceNode, IntoIterator, NullDrawCommand, NullIterator, Uniforms,
    Widget, WidgetBase, WidgetDraft,
};

struct Button;

impl WidgetDraft for Button {
    type BuildFeedback = (); // TODO: implement click response
    type AchievedType = Button;

    fn build(self, ui: &InterfaceNode) {
        // TODO: change default id
        ui.update_widget::<Self::AchievedType>(0, self);
    }
}

impl IntoIterator for Button {
    fn into_iter(self) -> Box<dyn Iterator<Item = &'static InterfaceNode>> {
        Box::new(NullIterator)
    }
}

impl WidgetBase for Button {
    fn update_from(&mut self, widget: Box<dyn Widget>) {
        let widget = widget.as_any().downcast_ref::<Self>().unwrap();
        // Update information
    }

    fn draw_commands(&self) -> DrawCommand {
        NullDrawCommand
    }
}
