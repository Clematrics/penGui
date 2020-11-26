use std::rc::Weak;

use super::node::{Node, NodeReference, NodeWeakReference};
use crate::core::{ComponentId, DrawList, Mat4x4, UNIT_TRANSFORM};
use crate::widget::WindowHandler;

pub struct GlobalProperties {
    // no events, but input state, stats, ...
    global_transformation: Mat4x4,
    _focus: NodeWeakReference,
}

/// Default user interface
pub struct UserInterface {
    properties: GlobalProperties,
    pub root: NodeReference,
}

// pub struct LockedInterface {
//     properties: GlobalProperties,
//     root: NodeReference,
// }

// impl UserInterface {
//     pub fn new() -> LockedInterface {
//         LockedInterface {
//             properties: GlobalProperties {
//                 global_transformation: UNIT_TRANSFORM,
//                 _focus: Weak::new(),
//             },
//             root: Node::new_reference_from(
//                 ComponentId::new_custom::<WindowHandler>(0),
//                 Box::new(WindowHandler::new()),
//             ),
//         }
//     }

//     pub fn global_transformation(&mut self, transform: Mat4x4) {
//         self.properties.global_transformation = transform;
//     }

//     pub fn end_frame(self) -> LockedInterface {
//         LockedInterface {
//             properties: self.properties,
//             root: self.root,
//         }
//     }
// }

// impl LockedInterface {
//     pub fn new_frame(self) -> UserInterface {
//         // Invalidate all windows
//         self.root.borrow_mut().metadata.invalid = true;
//         UserInterface {
//             properties: self.properties,
//             root: self.root,
//         }
//     }

//     pub fn generate_layout(&self) {
//         // TODO: implement
//     }

//     pub fn draw(&self) {
//         // TODO: implement
//     }

//     pub fn register_event(&self) {
//         // TODO: implement
//     }
// }
impl UserInterface {
    pub fn new() -> UserInterface {
        UserInterface {
            properties: GlobalProperties {
                global_transformation: UNIT_TRANSFORM,
                _focus: Weak::new(),
            },
            root: Node::new_reference_from(
                ComponentId::new_custom::<WindowHandler>(0),
                Box::new(WindowHandler::new()),
            ),
        }
    }

    pub fn global_transformation(&mut self, transform: Mat4x4) {
        self.properties.global_transformation = transform;
    }

    pub fn end_frame(&self) {}

    pub fn new_frame(&mut self) {
        // Invalidate all windows
        self.root.borrow_mut().metadata.invalid = true;
    }

    pub fn generate_layout(&self) {
        // TODO: implement
    }

    pub fn draw(&self) -> DrawList {
        self.root.borrow_mut().draw()
    }

    pub fn register_event(&self) {
        // TODO: implement
    }
}
