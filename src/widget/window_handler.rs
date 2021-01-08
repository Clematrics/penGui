use std::rc::Rc;

use nalgebra::*;

use crate::core::{
    CodeLocation, ComponentId, DrawList, LayoutQuery, LayoutResponse, LayoutStatus, Node,
    NodeMetadata, NodeReference, Objective, WidgetBuilder, WidgetLogic, WidgetQueryResult,
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

impl Default for WindowHandler {
    fn default() -> Self {
        Self::new()
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

impl WidgetLogic for WindowHandler {
    fn layout(&mut self, _query: &LayoutQuery) -> LayoutResponse {
        // Computing the layout of each window
        // We don't need to modify the size and position of each
        // window, as they manage this themselves
        for node in &self.windows {
            node.borrow_mut().layout(&LayoutQuery {
                available_space: (None, None),
                objectives: (Objective::Minimize, Objective::Minimize),
            });
        }

        // The response is irrelevant here
        LayoutResponse {
            size: (0., 0.),
            status: (LayoutStatus::Ok, LayoutStatus::Ok),
        }
    }

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

    fn draw(
        &self,
        _metadata: &NodeMetadata, /*, position: Point3<f32>, size: (f32, f32)*/
    ) -> DrawList {
        // let unit_y = Vector3::new(0., 1., 0.);

        // let widget_size = (size.0, size.1 / (self.windows.len()) as f32);

        // let mut current_pos = {
        //     let top_side = position + unit_y * size.1 / 2.;
        //     top_side - unit_y * widget_size.1 / 2.
        // };

        let mut list = DrawList::new();
        self.windows.iter().for_each(|node| {
            list.list
                .push(node.borrow_mut().draw(/*current_pos, widget_size*/));
            // current_pos -= unit_y * widget_size.1
        });
        list
    }
}
