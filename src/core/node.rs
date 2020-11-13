use std::any::Any;
use std::cell::RefCell;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::rc::{Rc, Weak};

use crate::core::ComponentId;
use crate::core::{DummyWidget, Widget, WidgetBuilder};

pub type NodeReference = Rc<RefCell<Node>>;
pub type NodeWeakReference = Weak<RefCell<Node>>;

pub struct NodeMetadata {
    pub id: ComponentId,
    pub invalid: bool,
}

/// An node is a wrapper around a widget that also holds
/// contextual information about the widget:
/// - the id with respect to the parent
/// - the validity of the widget
/// - events intercepted last frame
/// - layout constraints and its solution
/// - local style
pub struct Node {
    pub metadata: NodeMetadata,
    content: Box<dyn (Widget)>,
}

impl Node {
    pub fn new_reference(id: ComponentId) -> NodeReference {
        Rc::new(RefCell::new(Node {
            metadata: NodeMetadata { id, invalid: false },
            content: Box::new(DummyWidget),
        }))
    }

    pub fn query<T: Widget>(&mut self, id: ComponentId) -> NodeQueryResult<T> {
        match self.content.query(id) {
            WidgetQueryResult::Uninitialized(node) => {
                NodeQueryResult::<T>::VacantNode(node, PhantomData)
            }
            WidgetQueryResult::Initialized(node) => {
                NodeQueryResult::<T>::OccupiedNode(node, PhantomData)
            }
        }
    }

    pub fn borrow_parts(&mut self) -> (&NodeMetadata, &mut Box<dyn Widget>) {
        (&self.metadata, &mut self.content)
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

// enum NodeStatus {
// 	Uninitialized,
// 	Initialized
// }

pub enum WidgetQueryResult {
    Uninitialized(NodeReference),
    Initialized(NodeReference), // Uninitialized(&'a mut Node),
                                // Initialized(&'a mut Node)
}

pub enum NodeQueryResult<T: Widget> {
    VacantNode(NodeReference, PhantomData<T>),
    OccupiedNode(NodeReference, PhantomData<T>),
    // VacantNode(&'a mut Node, PhantomData<T>),
    // OccupiedNode(&'a mut Node, PhantomData<T>),
}

impl<T: Widget + 'static> NodeQueryResult<T> {
    pub fn update<U: WidgetBuilder<AchievedType = T>>(self, builder: U) -> NodeReference {
        match self {
            Self::VacantNode(node_ref, _) => {
                node_ref.borrow_mut().content = Box::new(builder.create());
                node_ref
            }
            Self::OccupiedNode(node_ref, _) => {
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

    // pub fn update(self, f: &dyn FnOnce(&NodeMetadata, &mut T) -> ()) -> Self {
    //     match self {
    //         Self::VacantNode(_, _) => self,
    //         Self::OccupiedNode(node_ref, _) => {
    //             {
    //                 let mut node = node_ref.borrow_mut();
    //                 let (metadata, content) = node.borrow_parts();
    //                 let downcast_res = content.as_any_mut().downcast_mut::<T>();
    //                 match downcast_res {
    //                     Some(content_ref) => f(metadata, content_ref),
    //                     None => panic!("Could not downcast content to concrete type requested"),
    //                 };
    //             }
    //             Self::OccupiedNode(node_ref, PhantomData)
    //         }
    //     }
    // }

    // pub fn or_create(self, content: T) -> NodeReference {
    //     match self {
    //         // Self::VacantNode(&mut node, _) => {
    //         Self::VacantNode(node_ref, _) => {
    //             node_ref.borrow_mut().content = Box::new(content);
    //             node_ref
    //         }
    //         Self::OccupiedNode(node_ref, _) => node_ref,
    //     }
    // }
}
