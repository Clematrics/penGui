use std::cell::RefCell;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::rc::{Rc, Weak};

use crate::core::*;

use nalgebra::Similarity3;

/// Type describing a shared, mutable reference to a `Node`
#[derive(Clone, PartialEq, Eq)]
pub struct NodeReference {
    inner: Rc<RefCell<Node>>,
}

/// Type describing a weak, mutable reference to a `Node`
#[derive(Clone)]
pub struct NodeWeakReference {
    inner: Weak<RefCell<Node>>,
}

impl NodeReference {
    pub fn new(inner: Rc<RefCell<Node>>) -> Self {
        Self { inner }
    }

    pub fn has_id(&self, id: ComponentId) -> bool {
        self.inner.borrow().metadata.id == id
    }

    pub fn is_valid(&self) -> bool {
        !self.inner.borrow().metadata.invalid
    }

    pub fn validate(&self) {
        self.inner.borrow_mut().metadata.invalid = false;
    }

    pub fn invalidate(&self) {
        self.inner.borrow_mut().metadata.invalid = true;
    }

    pub fn set_size(&self, size: (f32, f32)) {
        self.inner.borrow_mut().metadata.size = size;
    }

    pub fn set_transform(&self, transform: Similarity3<f32>) {
        self.inner.borrow_mut().metadata.transform = transform;
    }

    pub fn get_transform(&self) -> Similarity3<f32> {
        self.inner.borrow().metadata.transform.clone()
    }

    /// Wraps the `query` function of the contained node.
    pub fn query<T: Widget>(&self, id: ComponentId) -> NodeQueryResult<T> {
        self.inner.borrow_mut().query(id)
    }

    /// Wraps the `layout` function of the contained node.
    pub fn layout(&self, query: &LayoutQuery) -> LayoutResponse {
        self.inner.borrow_mut().layout(query)
    }

    /// Wraps the `draw` function of the contained node.
    pub fn draw(&self) -> DrawList {
        self.inner.borrow().draw()
    }

    /// Wraps the `interaction_distance` function of the contained node.
    pub fn interaction_distance(
        &self,
        ray: &Ray,
        self_node: NodeReference,
    ) -> Vec<(f32, NodeReference)> {
        self.inner.borrow().interaction_distance(ray, self_node)
    }

    /// Wraps the `send_event` function of the contained node.
    pub fn send_event(&self, event: &Event) -> EventResponse {
        self.inner.borrow_mut().send_event(event)
    }

    pub fn set_content(&self, content: Box<dyn Widget + 'static>) {
        self.inner.borrow_mut().content = content;
    }

    pub fn update<
        T: Widget + 'static,
        U,
        V: WidgetBuilder<AchievedType = T, UpdateFeedback = U>,
    >(
        &self,
        builder: V,
    ) -> U {
        self.validate();
        let mut node = self.inner.borrow_mut();
        let (metadata, content) = node.borrow_parts();
        let downcast_res = content.as_any_mut().downcast_mut::<T>();

        match downcast_res {
            Some(content_ref) => builder.update(&metadata, content_ref),
            None => panic!("Could not downcast content to concrete type requested"),
        }
    }

    pub fn apply_to_widget<T: Widget + 'static, F>(&self, f: F)
    where
        F: FnOnce(&NodeMetadata, &mut T),
    {
        let mut node = self.inner.borrow_mut();
        let (metadata, content) = node.borrow_parts();
        let downcast_res = content.as_any_mut().downcast_mut::<T>();

        match downcast_res {
            Some(content_ref) => (f)(metadata, content_ref),
            None => panic!("Could not downcast content to concrete type requested"),
        }
    }
}

impl NodeWeakReference {
    pub fn new() -> Self {
        Self { inner: Weak::new() }
    }

    pub fn from(other: &Rc<RefCell<Node>>) -> Self {
        Self {
            inner: Rc::downgrade(other),
        }
    }

    /// Wraps the `send_event` function of the contained node.
    pub fn send_event(&self, event: &Event) -> Option<EventResponse> {
        self.inner
            .upgrade()
            .map(|node| node.borrow_mut().send_event(event))
    }
}

impl Default for NodeWeakReference {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for NodeWeakReference {
    fn eq(&self, other: &Self) -> bool {
        self.inner.upgrade() == other.inner.upgrade()
    }
}

impl Eq for NodeWeakReference {}

/// A structure holding contextual information about a widget:
/// - the `ComponentId` with respect to the parent
/// - the validity of the widget
/// - events intercepted last frame (TODO: unimplemented)
/// - layout constraints and its solution (NOTE: implementation in progress)
/// - local style (TODO: unimplemented)
pub struct NodeMetadata {
    pub id: ComponentId,
    pub invalid: bool,
    pub size: (f32, f32),
    pub transform: Similarity3<f32>,
    pub ui_properties: Weak<RefCell<GlobalProperties>>,
    pub myself: NodeWeakReference,
}

impl NodeMetadata {
    pub fn new(
        id: ComponentId,
        myself: &NodeWeakReference,
        properties: &Weak<RefCell<GlobalProperties>>,
    ) -> Self {
        NodeMetadata {
            id,
            invalid: false,
            size: (0., 0.),
            transform: Similarity3::identity(),
            ui_properties: properties.clone(),
            myself: myself.clone(),
        }
    }

    pub fn is_focused(&self) -> bool {
        self.ui_properties
            .upgrade()
            .map(|prop| prop.borrow_mut().is_focused(&self.myself))
            .unwrap_or(false)
    }

    pub fn request_focus(&self) {
        self.ui_properties
            .upgrade()
            .map(|prop| prop.borrow_mut().request_focus(&self.myself));
    }
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
    pub fn new_reference(
        id: ComponentId,
        properties: &Weak<RefCell<GlobalProperties>>,
    ) -> NodeReference {
        // Note: should use new_cyclic as soon as it is stabilized. See #75861
        let cell = RefCell::new(Node {
            metadata: NodeMetadata::new(id, &Default::default(), properties),
            content: Box::new(DummyWidget),
        });
        let rc = Rc::new(cell);
        rc.borrow_mut().metadata.myself = NodeWeakReference::from(&rc);
        NodeReference::new(rc)
    }

    /// Creates a new valid `NodeReference` from the given `ComponentId`
    /// and the provided boxed widget
    pub fn new_reference_from(
        id: ComponentId,
        properties: &Weak<RefCell<GlobalProperties>>,
        widget: Box<dyn Widget>,
    ) -> NodeReference {
        // Note: should use new_cyclic as soon as it is stabilized. See #75861
        let cell = RefCell::new(Node {
            metadata: NodeMetadata::new(id, &Default::default(), properties),
            content: widget,
        });
        let rc = Rc::new(cell);
        rc.borrow_mut().metadata.myself = NodeWeakReference::from(&rc);
        NodeReference::new(rc)
    }

    /// Forwards the query to the contained widget and
    /// annotates the result with the type requested
    pub fn query<T: Widget>(&mut self, id: ComponentId) -> NodeQueryResult<T> {
        match self.content.query(&self.metadata, id) {
            WidgetQueryResult::Uninitialized(node) => {
                NodeQueryResult::<T>::UninitializedNode(node, PhantomData)
            }
            WidgetQueryResult::Initialized(node) => {
                NodeQueryResult::<T>::InitializedNode(node, PhantomData)
            }
        }
    }

    /// Wraps the `layout` function of the contained widget.
    pub fn layout(&mut self, query: &LayoutQuery) -> LayoutResponse {
        self.content.layout(query)
    }

    /// Wraps the `draw` function of the contained widget.
    ///
    /// NOTE: Takes a position and a size to place the widget in the local space.
    /// This is temporary, and will change when the layout system comes in place
    pub fn draw(&self) -> DrawList {
        self.content.draw(&self.metadata)
    }

    pub fn interaction_distance(
        &self,
        ray: &Ray,
        self_node: NodeReference,
    ) -> Vec<(f32, NodeReference)> {
        self.content
            .interaction_distance(&self.metadata, ray, self_node)
    }

    pub fn send_event(&mut self, event: &Event) -> EventResponse {
        self.content.send_event(&mut self.metadata, event)
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
    pub fn update<U: Default, V: WidgetBuilder<AchievedType = T, UpdateFeedback = U>>(
        self,
        builder: V,
    ) -> (NodeReference, U) {
        match self {
            Self::UninitializedNode(node_ref, _) => {
                node_ref.set_content(Box::new(builder.create()));
                (node_ref, Default::default())
            }
            Self::InitializedNode(node_ref, _) => {
                let feedback = node_ref.update(builder);
                (node_ref, feedback)
            }
        }
    }
}
