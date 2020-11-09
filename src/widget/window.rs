use crate::core::{
	ComponentId,
	DrawCommand,
	NullDrawCommand,
	DummyNode,
	InterfaceNode,
	IntoIterator,
	OneIterator,
	WidgetDraft,
	WidgetBase,
	Widget,
};

pub struct WindowDraft {
	id: ComponentId,
	title: &'static str
}

impl WidgetDraft for WindowDraft {
	type BuildFeedback = ();
	type AchievedType = Window;

	fn build(self, ui: &InterfaceNode) -> Self::BuildFeedback {
		ui.update_widget::<Self::AchievedType>(self.id, Window {
			title: self.title,
			content: DummyNode::new(),
		});
	}
}

pub struct Window {
	title: &'static str,
	content: InterfaceNode
}

impl IntoIterator for Window {
	fn into_iter(self) -> Box<dyn Iterator<Item=&'static InterfaceNode>> {
		Box::new(
			OneIterator::new(&self.content)
		)
	}
}

impl WidgetBase for Window {
	fn update_from(&mut self, other: Box<dyn Widget>) {
		self.content.widget = other;
	}

	fn add(&self, node: InterfaceNode) -> () {}

	// fn generate_surfaces(&self);

	fn draw_commands(&self) -> DrawCommand {
		NullDrawCommand
	}
}