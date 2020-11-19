use std::rc::Weak;

use super::node::{Node, NodeWeakReference};
use crate::core::{Mat4x4, UNIT_TRANSFORM};

pub struct GlobalProperties {
    // no events, but input state, stats, ...
    global_transformation: Mat4x4,
    _focus: NodeWeakReference,
}

/// Default user interface
pub struct UserInterface {
    properties: GlobalProperties,
    windows: Vec<Node>,
}

pub struct LockedInterface {
    properties: GlobalProperties,
    windows: Vec<Node>,
}

impl UserInterface {
    pub fn new() -> LockedInterface {
        LockedInterface {
            properties: GlobalProperties {
                global_transformation: UNIT_TRANSFORM,
                _focus: Weak::new(),
            },
            windows: Vec::new(),
        }
    }

    pub fn global_transformation(&mut self, transform: Mat4x4) {
        self.properties.global_transformation = transform;
    }

    pub fn end_frame(self) -> LockedInterface {
        LockedInterface {
            properties: self.properties,
            windows: self.windows,
        }
    }
}

impl LockedInterface {
    pub fn new_frame(self) -> UserInterface {
        // Invalidate all windows
        UserInterface {
            properties: self.properties,
            windows: self
                .windows
                .into_iter()
                .map(|mut ui| {
                    ui.metadata.invalid = true;
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
