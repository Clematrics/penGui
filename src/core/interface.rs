// Structure for old layout and current layout
// Current event and input states

// An interface can:
// - end the drawing state and compute the layout (from UserInterface to LockedInterface)
//   - compute a suitable layout
//   - apply the layout
// - export the 3D data and draw the frame (only on LockedInterface)
// - receive an event and store it (only on LockedInterface)
// - start a new frame (from LockedInterface to UserInterface)

use super::{ContainerDraft, Structure, WidgetDraft};

pub struct UserInterface {
	structure: Structure,
	// no events, but input state, stats, ...
}

pub struct LockedInterface {
	structure: Structure,

	// events, input state, stats, ...
}

impl UserInterface {
	pub fn new() -> LockedInterface {
		LockedInterface {
			structure: Structure::new()
		}
	}

	pub fn build_widget<W: WidgetDraft>(&self, widget: &W) -> W::BuildFeedback {
		widget.feedback()
	}

	pub fn build_container<C: ContainerDraft>(&self, container: &C) -> C::BuildFeedback {
		// cursor down
		let feedback = container.feedback();
		// cursor up
		feedback
	}

	pub fn end_frame(self) -> LockedInterface {
		// The cursor of the structure is supposed to be on the root
		self.structure.cursor().validate_content();

		// TODO: compute layout etc

		LockedInterface {
			structure: self.structure
		}
	}
}

impl LockedInterface {
	pub fn new_frame(self) -> UserInterface {
		self.structure.cursor().invalidate_content();
		UserInterface {
			structure: self.structure
		}
	}

	pub fn draw(&self) {
		// TODO: implement
	}

	pub fn register_event(&self) {
		// TODO: implement
	}
}
