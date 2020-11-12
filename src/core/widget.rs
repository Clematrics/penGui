// It can be built into an interface and when built, returns an event from the previous frame, or nothing
// Since a widget holds subwidgets, it must allow to iter through them, or find one with some id
// It must give layout informations (constraints and liberty) to construct the global layout
// It must be able to receive and store local transformations
// A widget must be able to give interaction surfaces and associated functions to react to events (doing nothing eventually)
// It must give visual informations through the form of draw commands after applying the global transformation to its local one

use crate::core::{DrawCommand, InterfaceNode, NullDrawCommand};
use std::any::Any;

pub trait Widget {

}
