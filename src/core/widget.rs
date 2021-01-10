// NOTE:
// It can be built into an interface and when built, returns an event from the previous frame, or nothing
// Since a widget holds subwidgets, it must allow to iter through them, or find one with some id
// It must give layout informations (constraints and liberty) to construct the global layout
// It must be able to receive and store local transformations
// A widget must be able to give interaction surfaces and associated functions to react to events (doing nothing eventually)
// It must give visual informations through the form of draw commands after applying the global transformation to its local one

use std::any::Any; // Implement Any for type coercion

use nalgebra::{Point3, Vector3};

use super::{
    CodeLocation, ComponentId, DrawList, Event, EventResponse, LayoutQuery, LayoutResponse,
    LayoutStatus, NodeMetadata, NodeReference, WidgetQueryResult,
};

/// Trait for the builder of a `Widget`.
///
/// Only the `build` function is relevant
///
/// # Examples
///
/// This can be used in the following way:
///
/// ```compile_fail
/// WidgetBuilder::new(/* */)
///     .function(/* */)
///     .other_function(/* */)
///     /// ...
///     .build(loc!(), parent_node);
/// ```
///
/// Using a chain of functions, you can gather all parameters needed to build
/// a widget, and then, as a last step, build the widget and attach it to a parent.
///
/// `build` can return a type specific to each widget builder, so a button for instance,
/// could return whether it was clicked or not. It could also returns nothing.
pub trait WidgetBuilder {
    /// The type of the widget built and added to the interface
    type AchievedType: Widget + 'static;
    /// The type returned from the `build` function
    type BuildFeedback;

    /// This function should update the widget already in the interface with the new information
    /// provided during this frame.
    ///
    /// This function should not be called outside of the `build` function
    fn update(self, metadata: &NodeMetadata, old: &mut Self::AchievedType);

    /// This function should create a new instance of a widget, based on the information the
    /// builder has. This will be used to create a new node if the widget was inexistant during
    /// the previous frame.
    ///
    /// This function should not be called outside of the `build` function
    fn create(self) -> Self::AchievedType;

    /// This function is the final step when building a widget. It should create the `ComponentId`
    /// of the widget from the `CodeLocation` provided and an optional custom id.
    /// Then it should query this id to the given parent to check if a previous widget with the same id
    /// existed, and update it accordingly. If it did not existed, one will be created automatically.
    ///
    /// If a widget contains other widgets that can be deleted and added in a custom way between frames,
    /// it should invalidate them before updating them, and then eliminate
    /// all invalid widgets remaining (since updating a widget will mark it as valid for this frame)
    ///
    /// Since the update consumes the builder, you cannot extract information by reference from the builder
    /// to use it afterwards. It is thus recommended to use an `Option<Type>` and use the `take` function to
    /// take ownership of the contained value, so it can be used later.
    ///
    /// # Examples
    ///
    /// A typical implementation for a classic widget could look like this:
    ///
    /// ```compile_fail
    /// let id = ComponentId::new::<Self::AchievedType>(loc);
    ///
    /// let needed_afterward = self.needed_afterward.take().unwrap();
    ///
    /// parent
    ///     .borrow_mut()
    ///     .query::<Self::AchievedType>(id)
    ///     .update(self);
    ///
    /// using(needed_afterward);
    /// return feedback;
    /// ```
    ///
    /// The implementation for a container (here of only one widget) could look like this:
    ///
    /// ```compile_fail
    /// let id = ComponentId::new::<Self::AchievedType>(loc);
    ///
    /// // widget_inside is the builder for a widget that should be contained in the current one
    /// let widget_inside = self.widget_inside.take().unwrap();
    ///
    /// // node is the node of the current widget (re)built
    /// let node = parent
    ///     .borrow_mut()
    ///     .query::<Self::AchievedType>(id)
    ///     .update(self);
    ///
    /// // The code location provided here does not matter since there is only one widget stored
    /// widget_inside.build(loc!(), node);
    /// return feedback;
    /// ```
    ///
    /// It is here unnecessary to invalidate the contained widget since there should always be one slot
    /// which is never deleted.
    ///
    /// The implementation for a container with multiple widgets could look like this
    /// (all widegts are placed in a Vec<NodeReference> field called content):
    ///
    /// ```compile_fail
    /// let id = ComponentId::new::<Self::AchievedType>(loc);
    ///
    /// // generator is a lambda that will add mutliple widgets to this container
    /// // simply using .build(...)
    /// let generator = self.generator.take().unwrap();
    ///
    /// // node is the node of the current widget (re)built
    /// let node = parent
    ///     .borrow_mut()
    ///     .query::<Self::AchievedType>(id)
    ///     .update(self);
    ///
    /// self.content.iter().for_each(|&node_ref| node_ref.borrow_mut().metadata.invalid = true);
    /// (generator)(node_ref);
    /// self.content.iter().filter(|&node_ref| node_ref.borrow_mut().metadata.invalid = false);
    ///
    /// return feedback;
    /// ```
    ///
    /// TODO: implement macros to help this kind of implementation
    fn build(self, loc: CodeLocation, parent: NodeReference) -> Self::BuildFeedback;
}

/// Trait that defines the core logic of a widget in the interface
pub trait WidgetLogic {
    /// A widget can receive a query of a contained widget, and should return the corresponding
    /// NodeReference (or a new one if it did not exist) in a WidgetQueryResult.
    ///
    /// Panics by default since there is no contained widget yet.
    fn query(&mut self, _id: ComponentId) -> WidgetQueryResult {
        panic!("Trying to query a widget from another one which does not contains one (or has not implemented the 'query' function")
    }

    /// A widget can receive a layout query from its parent, with the available space and the constraints.
    /// The widget must respond to this query.
    fn layout(&mut self, _query: &LayoutQuery) -> LayoutResponse {
        LayoutResponse {
            size: (0., 0.),
            status: (LayoutStatus::Ok, LayoutStatus::Ok),
        }
    }

    /// A widget must be able to be drawn on screen. It should then returns the information
    /// indicating how to draw it. The `DrawList` returned should be in a way that represents
    /// adequately the contained widgets, if there are some.
    fn draw(&self, _metadata: &NodeMetadata) -> DrawList {
        DrawList::new()
    }

    fn interaction_distance(
        &self,
        _metadata: &NodeMetadata,
        _ray: &Vector3<f32>,
        _origin: &Point3<f32>,
        _self_node: NodeReference,
    ) -> Vec<(f32, NodeReference)> {
        vec![]
    }

    fn send_event(&mut self, _metadata: &mut NodeMetadata, _event: &Event) -> EventResponse {
        EventResponse::Pass
    }
}

/// Trait that gives dynamic typing capabilities to objects implementing
/// `WidgetLogic`, `Any` and implicitly `Sized`. It is automatically implemented
/// for all such objects. This completes the interface needed for all widgets.
pub trait Widget: WidgetLogic + Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

// Implementing `Widget` for all types
impl<T> Widget for T
where
    T: WidgetLogic + Any,
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

/// Dummy widget used as the default widget when a new node is created.
/// This is normally automatically replaced during the building process,
/// if `WidgetBuilder::create` is correctly implemented (that is, it does not
/// return a `DummyWidget`)
pub struct DummyWidget;

impl WidgetLogic for DummyWidget {
    /// Should never be called
    fn query(&mut self, _id: ComponentId) -> WidgetQueryResult {
        panic!("Attempting to query a child of a DummyWidget")
    }
}
