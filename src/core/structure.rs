use std::cell::{RefCell, RefMut};
use std::rc::Rc;

use super::{Container, GlobalContainer};

pub struct Structure {
	tree: Rc<RefCell<dyn Container>>,
	cursor: Rc<RefCell<dyn Container>>
}

impl Structure {
	pub fn new() -> Self {
		let global_container = GlobalContainer::new();
		let tree = Rc::new(RefCell::new(global_container));
		let cursor = Rc::clone(&tree);
		Structure {
			tree,
			cursor
		}
	}

	pub fn cursor(&self) -> RefMut<dyn Container> {
		self.cursor.borrow_mut()
	}
}
