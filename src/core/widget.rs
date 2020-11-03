// It can be built into an interface and when built, returns an event from the previous frame, or nothing
// Since a widget holds subwidgets, it must allow to iter through them, or find one with some id
// It must give layout informations (constraints and liberty) to construct the global layout
// It must be able to receive and store local transformations
// A widget must be able to give interaction surfaces and associated functions to react to events (doing nothing eventually)
// It must give visual informations through the form of draw commands after applying the global transformation to its local one

use std::any::Any;
use crate::core::{DrawCommand, NullDrawCommand, NullIterator, InterfaceNode};

pub trait WidgetDraft {
	type BuildFeedback;
	type AchievedType: Widget;

	fn build(self, ui: &InterfaceNode) -> Self::BuildFeedback;
}

pub trait IntoIterator {
	fn into_iter(self) -> Box<dyn Iterator<Item = &'static InterfaceNode>>;
}

pub trait WidgetBase : IntoIterator {
	fn update_from(&mut self, other: Box<dyn Widget>);

	fn add(&self, node: InterfaceNode) -> () {}

	// fn generate_surfaces(&self);

	fn draw_commands(&self) -> DrawCommand;
}

pub trait Widget: Any + WidgetBase {
	fn as_any(&self) -> &dyn Any;
	fn as_mut_any(&mut self) -> &mut dyn Any;
}

impl<T> Widget for T
where
	T: WidgetBase + Any
{
		fn as_any(&self) -> &dyn Any { self }
		fn as_mut_any(&mut self) -> &mut dyn Any { self }
}

pub struct DummyWidget;

impl IntoIterator for DummyWidget {
	fn into_iter(self) -> Box<dyn Iterator<Item = &'static InterfaceNode>> {
		Box::new(NullIterator)
	}
}

impl WidgetBase for DummyWidget {
	fn update_from(&mut self, other: Box<dyn Widget>) {}

	fn add(&self, node: InterfaceNode) -> () {}

	// fn generate_surfaces(&self);

	fn draw_commands(&self) -> DrawCommand {
		NullDrawCommand
	}
}