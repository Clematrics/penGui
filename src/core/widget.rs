// It can be built into an interface and when built, returns an event from the previous frame, or nothing
// Since a widget holds subwidgets, it must allow to iter through them, or find one with some id
// It must give layout informations (constraints and liberty) to construct the global layout
// It must be able to receive and store local transformations
// A widget must be able to give interaction surfaces and associated functions to react to events (doing nothing eventually)
// It must give visual informations through the form of draw commands after applying the global transformation to its local one

use std::any::Any; // Implement Any for type coercion
use nalgebra::Point3;

use super::{CodeLocation, ComponentId, DrawList, NodeMetadata, NodeReference, WidgetQueryResult};

pub trait WidgetBuilder {
    type AchievedType: Widget + 'static;
    type BuildFeedback;

    fn update(self, metadata: &NodeMetadata, old: &mut Self::AchievedType) -> ();
    fn create(self) -> Self::AchievedType;

    fn build(self, loc: CodeLocation, node: NodeReference) -> Self::BuildFeedback;
}

pub trait WidgetBase {
    fn query(&mut self, _id: ComponentId) -> WidgetQueryResult {
        panic!("Trying to query a widget from another one which does not contains one (or has not implemented the 'query' function")
    }
    fn draw(&self, _position: Point3<f32>, _size: (f32, f32)) -> DrawList {
        DrawList::new()
    }
}

pub trait Widget: WidgetBase {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T> Widget for T
where
    T: WidgetBase + Any,
{
    fn as_any(&self) -> &dyn Any
    where
        Self: Sized,
    {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any
    where
        Self: Sized,
    {
        self
    }
}

pub struct DummyWidget;

impl WidgetBase for DummyWidget {
    fn query(&mut self, _id: ComponentId) -> WidgetQueryResult {
        panic!("Attempting to query a child of a DummyWidget")
    }
}
