use std::cell::RefCell;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::rc::{Rc, Weak};

use nalgebra::*;

use crate::core::ComponentId;
use crate::core::{DrawList, DummyWidget, Widget, WidgetBuilder};

/// Type describing a shared, mutable reference to a `Node`
pub type NodeReference = Rc<RefCell<Node>>;
/// Type describing a weak, mutable reference to a `Node`
pub type NodeWeakReference = Weak<RefCell<Node>>;

/// A structure holding contextual information about a widget:
/// - the `ComponentId` with respect to the parent
/// - the validity of the widget
/// - events intercepted last frame (TODO: unimplemented)
/// - layout constraints and its solution (TODO: unimplemented)
/// - local style (TODO: unimplemented)
pub struct NodeMetadata {
    pub id: ComponentId,
    pub invalid: bool,
}

/// An node is a wrapper around an object with a `Widget` trait,
/// that also holds its metadata
pub struct Node {
    pub metadata: NodeMetadata,
    content: Box<dyn Widget>,
}

impl Node {
    /// Creates a new valid `NodeReference` from the given `ComponentId`,
    /// with a `DummyWidget` inside
    pub fn new_reference(id: ComponentId) -> NodeReference {
        Rc::new(RefCell::new(Node {
            metadata: NodeMetadata { id, invalid: false },
            content: Box::new(DummyWidget),
        }))
    }

    /// Creates a new valid `NodeReference` from the given `ComponentId`
    /// and the provided boxed widget
    pub fn new_reference_from(id: ComponentId, widget: Box<dyn Widget>) -> NodeReference {
        Rc::new(RefCell::new(Node {
            metadata: NodeMetadata { id, invalid: false },
            content: widget,
        }))
    }

    /// Forwards the query to the contained widget and
    /// annotates the result with the type requested
    pub fn query<T: Widget>(&mut self, id: ComponentId) -> NodeQueryResult<T> {
        match self.content.query(id) {
            WidgetQueryResult::Uninitialized(node) => {
                NodeQueryResult::<T>::UninitializedNode(node, PhantomData)
            }
            WidgetQueryResult::Initialized(node) => {
                NodeQueryResult::<T>::InitializedNode(node, PhantomData)
            }
        }
    }

    /// Wraps the `draw` function of the contained widget.
    ///
    /// NOTE: Takes a position and a size to place the widget in the local space.
    /// This is temporary, and will change when the layout system comes in place
    pub fn draw(&self, position: Point3<f32>, size: (f32, f32)) -> DrawList {
        self.content.draw(position, size)
    }

    /// Helper function to mutably borrow the contained widget and its metadata independently
    pub fn borrow_parts(&mut self) -> (&mut NodeMetadata, &mut Box<dyn Widget>) {
        (&mut self.metadata, &mut self.content)
    }
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.metadata.id.hash(state);
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.metadata.id == other.metadata.id
    }
}

impl Eq for Node {}

/// The result of a query on a widget, holding a reference to the node holding it.
///
/// If the widget was inexistant during the previous frame, the result is `Uninitialized`,
/// and the node references a `DummyWidget`, waiting to be changed.
///
/// Otherwise, it is `Initialized`, and the node referenced is the one with the widget queried.
pub enum WidgetQueryResult {
    Uninitialized(NodeReference),
    Initialized(NodeReference),
}

/// The result of a query on a node (implicitly on the content of the widget contained), holding a reference to
/// to the child node. This structure is annotated during the compilation by the type requested.
///
/// If the widget was inexistant during the previous frame, the result is `UninitializedNode`,
/// and the node references a `DummyWidget`, waiting to be changed.
///
/// Otherwise, it is `InitializedNode`, and the node referenced is the one with the widget queried.
pub enum NodeQueryResult<T: Widget> {
    UninitializedNode(NodeReference, PhantomData<T>),
    InitializedNode(NodeReference, PhantomData<T>),
}

impl<T: Widget + 'static> NodeQueryResult<T> {
    /// From the result of a query, this function either:
    /// - builds a new widget from its builder type because the widget was inexistant during last frame
    /// - extracts the widget with the concrete type requested (this should not fail since the request
    /// matches on the `ComponentId` of the nodes, which are equal only if the TypeId are equal), and then
    /// updates the widget with the builder
    ///
    /// # Panics
    ///
    /// Panics if the type requested is different from the concrete type of the widget in the result.
    /// This should not append however. If it does, then either there is a bug in the library, or there is a
    /// custom widget builder which is not implemented correctly.
    ///
    /// TODO: test this properly. Is it possible to test this on all types that implement the `Widget` trait?
    pub fn update<U: WidgetBuilder<AchievedType = T>>(self, builder: U) -> NodeReference {
        match self {
            Self::UninitializedNode(node_ref, _) => {
                node_ref.borrow_mut().content = Box::new(builder.create());
                node_ref
            }
            Self::InitializedNode(node_ref, _) => {
                {
                    let mut node = node_ref.borrow_mut();
                    let (metadata, content) = node.borrow_parts();
                    let downcast_res = content.as_any_mut().downcast_mut::<T>();
                    match downcast_res {
                        Some(content_ref) => builder.update(metadata, content_ref),
                        None => panic!("Could not downcast content to concrete type requested"),
                    };
                }
                node_ref
            }
        }
    }
}
