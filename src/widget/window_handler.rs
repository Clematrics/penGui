use std::rc::Rc;

use crate::core::{
    CodeLocation, ComponentId, DrawList, Node, NodeMetadata, NodeReference, WidgetBase,
    WidgetBuilder, WidgetQueryResult,
};

pub struct WindowHandler {
    windows: Vec<NodeReference>,
}

impl WindowHandler {
    pub fn new() -> Self {
        WindowHandler {
            windows: Vec::new(),
        }
    }
}

impl WidgetBuilder for WindowHandler {
    type AchievedType = WindowHandler;
    type BuildFeedback = ();

    fn update(self, _metadata: &NodeMetadata, _old: &mut Self::AchievedType) {}

    fn create(self) -> Self::AchievedType {
        Self::AchievedType {
            windows: self.windows,
        }
    }

    fn build(self, _loc: CodeLocation, _parent: NodeReference) -> Self::BuildFeedback {}
}

impl WidgetBase for WindowHandler {
    fn query(&mut self, id: ComponentId) -> WidgetQueryResult {
        let child = self
            .windows
            .iter()
            .find(|&other| (*other).borrow().metadata.id == id)
            .map(Rc::clone);
        match child {
            Some(node_ref) => WidgetQueryResult::Initialized(node_ref),
            None => {
                let node_ref = Node::new_reference(id);
                self.windows.push(node_ref.clone());
                WidgetQueryResult::Uninitialized(node_ref)
            }
        }
    }

    fn draw(&self) -> DrawList {
        let mut list = DrawList::new();
        self.windows
            .iter()
            .for_each(|node| list.list.push(node.borrow_mut().draw()));
        list
    }
}
