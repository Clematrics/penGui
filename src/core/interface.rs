use std::rc::Weak;

use super::node::{Node, NodeWeakReference};
use crate::core::{Backend, Mat4x4, UNIT_TRANSFORM};

pub struct GlobalProperties<T, U> {
    // no events, but input state, stats, ...
    backend: Box<dyn Backend<DrawResult = T, Frame = U>>,
    global_transformation: Mat4x4,
    focus: NodeWeakReference,
}

/// Default user interface
pub struct UserInterface<T, U> {
    properties: GlobalProperties<T, U>,
    windows: Vec<Node>,
}

pub struct LockedInterface<T, U> {
    properties: GlobalProperties<T, U>,
    windows: Vec<Node>,
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
