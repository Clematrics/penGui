use std::collections::HashSet;
use crate::core::{Backend, Mat4x4, UNIT_TRANSFORM, Widget, WidgetAny};

pub type ComponentId = u128;

/// Encapsulate all metadata about the contained widget
/// For instance, whether it is valid or not in the current frame,
/// the local styling options, layout solutions, event capture ...
pub struct InterfaceNode {
	pub invalid: bool,
	pub id: ComponentId,

	pub widget: Box<dyn WidgetAny>
}

impl InterfaceNode {
	pub fn new(id: ComponentId, widget: Box<dyn WidgetAny>) -> Self {
		InterfaceNode {
			invalid: false,
			id,
			widget
		}
	}

	pub fn update_widget<T>(&self, id: ComponentId, widget: T)
	where
		T: Widget<Item = InterfaceNode> + 'static
	{
		let new_widget = Box::new(widget);
		let old_widget = self.widget.find_map(|sub_w| {
			if sub_w.id == id {
				sub_w.widget.as_mut_any().downcast_mut::<T>()
			}
			else {
				None
			}
		});
		match old_widget {
			Some(old_widget) => {
				old_widget.update_from(new_widget)
			}
			None => {
				self.widget.add(InterfaceNode::new(id, Box::new(widget)));
			}
		}
	}
}

pub struct GlobalProperties<T, U> {
	// no events, but input state, stats, ...
	backend: Box<dyn Backend<DrawResult=T, Frame=U>>,
	global_transformation: Mat4x4,
}

/// Default user interface
pub struct UserInterface<T, U> {
	properties: GlobalProperties<T, U>,
	windows: HashSet<InterfaceNode>
}

pub struct LockedInterface<T, U> {
	properties: GlobalProperties<T, U>,
	windows: HashSet<InterfaceNode>
}

impl<T, U> UserInterface<T, U> {
	pub fn new(backend: Box<dyn Backend<DrawResult=T, Frame=U>>) -> LockedInterface<T, U> {
		LockedInterface {
			properties: GlobalProperties {
				backend,
				global_transformation: UNIT_TRANSFORM,
			},
			windows: HashSet::new()
		}
	}

	pub fn global_transformation(&self, transform: Mat4x4) {
		self.properties.global_transformation = transform;
	}

	pub fn end_frame(self) -> LockedInterface<T, U> {
		LockedInterface {
			properties: self.properties,
			windows: self.windows
		}
	}
}

impl<T, U> LockedInterface<T, U> {
	pub fn new_frame(self) -> UserInterface<T, U> {
		// Invalidate all windows
		for ui in self.windows {
			ui.invalid = true;
		}

		UserInterface {
			properties: self.properties,
			windows: self.windows,
		}
	}

	pub fn generate_layout(&self) {
		// TODO: implement
	}

	pub fn draw(&self) {
		// TODO: implement
	}

	pub fn register_event(&self) {
		// TODO: implement
	}
}
