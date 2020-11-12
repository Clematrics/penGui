use std::any::TypeId;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::rc::{Rc, Weak};

use crate::core::{Backend, Mat4x4, Widget, UNIT_TRANSFORM};
use crate::core::UniqueId;

struct InterfaceNodeInner;

type InterfaceNodeOuter = Rc<RefCell<InterfaceNodeInner>>;

trait CustomTrait {}

impl CustomTrait for InterfaceNodeOuter {

}

// pub type ComponentId = u128;

pub struct NodeHolder {
	pub id: ComponentId,
	type_id: TypeId,
	order: usize,
	node: Rc<RefCell<InterfaceNode>>,
}

impl Hash for NodeHolder {
	fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.type_id.hash(state);
    }
}

impl PartialEq for NodeHolder {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id && self.type_id == other.type_id
	}
}

impl Eq for NodeHolder {
}

/// Encapsulate all metadata about the contained widget
/// For instance, whether it is valid or not in the current frame,
/// the local styling options, layout solutions, event capture ...
pub struct InterfaceNode {
    pub invalid: bool,
    pub content: Box<dyn Widget>,

	pub subnodes: HashSet<NodeHolder>,
	// TODO: received events
}

impl InterfaceNode {
    pub fn new<T: 'static + Widget>(id: ComponentId, widget: T) -> NodeHolder {
		NodeHolder {
			id,
			type_id: TypeId::of::<T>(),
			node: Rc::new(RefCell::new(InterfaceNode {
				invalid: false,
				content: Box::new(widget),
				subnodes: HashSet::new(),
			}))
		}
    }
}

pub struct GlobalProperties<T, U> {
    // no events, but input state, stats, ...
    backend: Box<dyn Backend<DrawResult = T, Frame = U>>,
    global_transformation: Mat4x4,
    focus: Weak<RefCell<InterfaceNode>>,
}

/// Default user interface
pub struct UserInterface<T, U> {
    properties: GlobalProperties<T, U>,
    windows: Vec<InterfaceNode>,
}

pub struct LockedInterface<T, U> {
    properties: GlobalProperties<T, U>,
    windows: Vec<InterfaceNode>,
}

impl<T, U> UserInterface<T, U> {
    pub fn new(backend: Box<dyn Backend<DrawResult = T, Frame = U>>) -> LockedInterface<T, U> {
        LockedInterface {
            properties: GlobalProperties {
                backend,
                global_transformation: UNIT_TRANSFORM,
                focus: Weak::new(),
            },
            windows: Vec::new(),
        }
    }

    pub fn global_transformation(&mut self, transform: Mat4x4) {
        self.properties.global_transformation = transform;
    }

    pub fn end_frame(self) -> LockedInterface<T, U> {
        LockedInterface {
            properties: self.properties,
            windows: self.windows,
        }
    }
}

impl<T, U> LockedInterface<T, U> {
    pub fn new_frame(self) -> UserInterface<T, U> {
        // Invalidate all windows
        UserInterface {
            properties: self.properties,
            windows: self
                .windows
                .into_iter()
                .map(|mut ui| {
                    ui.invalid = true;
                    ui
                })
                .collect(),
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
