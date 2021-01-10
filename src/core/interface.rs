use std::rc::Weak;

use crate::core::*;
use crate::widget::WindowHandler;

/// Global properties of an interface
pub struct GlobalProperties {
    // no events, but input state, stats, ...
    global_transformation: Mat4x4,
    input_state: InputState,
    _focus: NodeWeakReference,
}

/// A structure holding an interface during its buildind process
pub struct Interface {
    properties: GlobalProperties,
    pub root: NodeReference,
}

// pub struct LockedInterface {
//     properties: GlobalProperties,
//     root: NodeReference,
// }

// impl Interface {
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
//     pub fn new_frame(self) -> Interface {
//         // Invalidate all windows
//         self.root.borrow_mut().metadata.invalid = true;
//         Interface {
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

impl Interface {
    /// Creates a new interface
    /// The root of the widget tree is a `WindowHandler`, so only windows can be built.
    /// TODO: ensure that only windows can indeed be built below the root
    pub fn new() -> Interface {
        Interface {
            properties: GlobalProperties {
                global_transformation: Mat4x4::identity(),
                input_state: Default::default(),
                _focus: Weak::new(),
            },
            root: Node::new_reference_from(
                ComponentId::new_custom::<WindowHandler>(0),
                Box::new(WindowHandler::new()),
            ),
        }
    }

    /// Sets the global transformation applied on all vertices of this interface.
    /// The transformation is the last one applied. It is especially useful to set
    /// the projection and the view matrix
    pub fn global_transformation(&mut self, transform: Mat4x4) {
        self.properties.global_transformation = transform;
    }

    /// Ends the frame. After this, no changes to the interface can be applied
    /// TODO: ensure this by using the type system and returning a locked interface
    pub fn end_frame(&self) {}

    /// Starts a new frame. After this point, the interface can be reconstructed in
    /// an *immediate* style: recalling the function to build the same interface will update it,
    /// by creating new widgets if they were not there previously, and deleting ones that are not reconstructed.
    /// Widgets reconstructed are updated properly.
    pub fn new_frame(&mut self) {
        // Invalidate all windows
        self.root.borrow_mut().metadata.invalid = true;
    }

    /// Computes the layout, trying to satisfy all constraints provided by each widget.
    pub fn generate_layout(&self) -> LayoutResponse {
        self.root.borrow_mut().layout(&LayoutQuery {
            available_space: (Some(0.), Some(0.)),
            objectives: (Objective::None, Objective::None),
        })
    }

    /// Returns a `DrawList`, a tree structure with `DrawCommand`s on its node, holding all the information
    /// necessary to draw the interface. This does not draw anything on the screen: the result of this function
    /// has to be passed to a backend, in charge of drawing.
    /// TODO: change the name?
    pub fn draw(&self /*, position: Point3<f32>, size: (f32, f32)*/) -> DrawList {
        self.root.borrow_mut().draw(/*position, size*/)
    }

    /// Registers an event in the interface, propagating it to the right widget
    pub fn register_event(&self, event: Event, ray: Option<&Ray>) -> EventResponse {
        // TODO: change the InputState of the interface accordingly
        if let Some(ray) = ray {
            let mut distances = self
                .root
                .borrow()
                .interaction_distance(ray, self.root.clone());
            distances.sort_unstable_by(|(d1, _), (d2, _)| d1.partial_cmp(d2).unwrap());

            let mut passively_registered = false;
            for (_distance, widget) in &distances {
                let response = widget.borrow_mut().send_event(&event);
                match response {
                    EventResponse::Registered => return EventResponse::Registered,
                    EventResponse::PassivelyRegistered => passively_registered = true,
                    _ => (),
                }
            }
            if passively_registered {
                return EventResponse::PassivelyRegistered;
            }
        }
        EventResponse::Pass
        // TODO: if no ray, send to focused widget
        // self.root.borrow_mut().send_event(&event, ray, origin);
    }
}

impl Default for Interface {
    fn default() -> Self {
        Self::new()
    }
}
