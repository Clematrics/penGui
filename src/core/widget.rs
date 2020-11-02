// It can be built into an interface and when built, returns an event from the previous frame, or nothing
// Since a widget holds subwidgets, it must allow to iter through them, or find one with some id
// It must give layout informations (constraints and liberty) to construct the global layout
// It must be able to receive and store local transformations
// A widget must be able to give interaction surfaces and associated functions to react to events (doing nothing eventually)
// It must give visual informations through the form of draw commands after applying the global transformation to its local one

use super::{DrawCommand, UserInterface};

pub trait WidgetDraft {
	type BuildFeedback;

	// Should consume or not ? If not, a widget must be copyable, but could cause problems with pointers
	// fn build(self, ui: &UserInterface) -> Self::BuildFeedback;

	fn feedback(&self) -> Self::BuildFeedback;
}


pub trait Widget {
	fn mark_valid(&self) -> ();
	fn mark_invalid(&self) -> ();
	fn is_valid(&self) -> bool;

	// Impl iter trait
	// Retrieve subwidget by id

	// Layout & transformations

	// Interaction
	// Retrieve event from the past

	fn draw_commands(&self) -> DrawCommand;
}

pub trait ContainerDraft {
	type BuildFeedback;

	// Should consume or not ? If not, a widget must be copyable, but could cause problems with pointers
	// fn build(self, ui: &UserInterface) -> Self::BuildFeedback;

	fn feedback(&self) -> Self::BuildFeedback;
}

pub trait Container : Widget {
	fn validate_content(&mut self) -> ();

	fn invalidate_content(&self) -> ();

	fn add(&self, widget: &dyn Widget) -> ();
}
