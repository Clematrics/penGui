use std::rc::Rc;

use nalgebra::*;

use crate::core::*;

/// A handler able to place windows in space
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
        self
    }

    fn build(self, _loc: CodeLocation, _parent: NodeReference) -> Self::BuildFeedback {}
}

impl WidgetLogic for WindowHandler {
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

    fn layout(&mut self, _query: &LayoutQuery) -> LayoutResponse {
        // Computing the layout of each window
        // We don't need to modify the size and position of each
        // window, as they manage this themselves
        let mut status = (LayoutStatus::Ok, LayoutStatus::Ok);
        for node in &self.windows {
            let response = node.borrow_mut().layout(&LayoutQuery {
                available_space: (None, None),
                objectives: (Objective::Minimize, Objective::Minimize),
            });
            status.0 = LayoutStatus::and(status.0, response.status.0);
            status.1 = LayoutStatus::and(status.1, response.status.1);

            node.borrow_mut().metadata.position =
                (-response.size.0 / 2., -response.size.1 / 2., 3.);
        }
        // The response is irrelevant here
        LayoutResponse {
            size: (0., 0.),
            status,
        }
    }

    fn draw(&self, _metadata: &NodeMetadata) -> DrawList {
        let mut list = DrawList::new();
        self.windows.iter().for_each(|node| {
            list.list.push(node.borrow_mut().draw());
        });
        list
    }

    fn interaction_distance(
        &self,
        metadata: &NodeMetadata,
        ray: &Ray,
        _self_node: NodeReference,
    ) -> Vec<(f32, NodeReference)> {
        let (x, y, z) = metadata.position;
        let transformation = Translation3::new(x, y, z).inverse();
        let new_ray = Ray::new(ray.direction(), transformation * ray.origin());
        self.windows
            .iter()
            .map(|window| {
                window
                    .borrow()
                    .interaction_distance(&new_ray, window.clone())
            })
            .flatten()
            .collect()
    }
}
