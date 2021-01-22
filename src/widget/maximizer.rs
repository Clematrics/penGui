use nalgebra::*;

use crate::core::*;
use crate::loc;

/// A widget that can contain another one, but adjusting its layout with a small padding
/// on each side. An outline is displayed to show it
pub struct MaximizeLayout<T: WidgetBuilder> {
    content: Option<T>,
}

impl<T: WidgetBuilder> MaximizeLayout<T> {
    pub fn new(content: T) -> Self {
        MaximizeLayout {
            content: Some(content),
        }
    }
}

impl<T: WidgetBuilder> WidgetBuilder for MaximizeLayout<T> {
    type AchievedType = Maximize;
    type UpdateFeedback = ();
    type BuildFeedback = T::BuildFeedback;

    fn update(
        self,
        _metadata: &NodeMetadata,
        _widget: &mut Self::AchievedType,
    ) -> Self::UpdateFeedback {
    }

    fn create(self) -> Self::AchievedType {
        Self::AchievedType { content: None }
    }

    fn build(mut self, loc: CodeLocation, parent: &NodeReference) -> Self::BuildFeedback {
        let id = ComponentId::new::<Self::AchievedType>(loc);
        let content = self.content.take().unwrap();
        let (node_ref, _) = parent.query::<Self::AchievedType>(id).update(self);

        content.build(loc!(), &node_ref)
    }
}

pub struct Maximize {
    content: Option<NodeReference>,
}

impl WidgetLogic for Maximize {
    fn query(&mut self, metadata: &NodeMetadata, id: ComponentId) -> WidgetQueryResult {
        let child = {
            match &self.content {
                Some(other) => {
                    if other.has_id(id) {
                        Some(other.clone())
                    } else {
                        None
                    }
                }
                None => None,
            }
        };
        match child {
            Some(node_ref) => WidgetQueryResult::Initialized(node_ref),
            None => {
                let node_ref = Node::new_reference(id, &metadata.ui_properties);
                self.content = Some(node_ref.clone());
                WidgetQueryResult::Uninitialized(node_ref)
            }
        }
    }

    fn layout(&mut self, query: &LayoutQuery) -> LayoutResponse {
        let new_query = LayoutQuery {
            objectives: (Objective::Maximize, query.objectives.1),
            ..*query
        };
        let response = self.content.as_ref().unwrap().layout(&new_query);

        let node = self.content.as_ref().unwrap();
        node.set_size(response.size);
        node.set_transform(Similarity3::identity());

        LayoutResponse {
            size: response.size,
            status: response.status,
        }
    }

    fn draw(&self, metadata: &NodeMetadata) -> DrawList {
        let mut list = DrawList::new();
        list.list.push(self.content.as_ref().unwrap().draw());
        list.list_transform = metadata.transform.to_homogeneous();
        // let debug_color = (1., 0., 0., 1.);
        // let debug_command = debug_quad(
        //     metadata.size.0,
        //     metadata.size.1,
        //     debug_color,
        //     metadata.transform,
        // );
        // list.commands.push(debug_command);
        list
    }

    fn interaction_distance(
        &self,
        metadata: &NodeMetadata,
        ray: &Ray,
        _self_node: NodeReference,
    ) -> Vec<(f32, NodeReference)> {
        let transformation = metadata.transform.inverse();
        let new_ray = Ray::new(ray.direction(), transformation * ray.origin());
        self.content
            .iter()
            .map(|content| content.interaction_distance(&new_ray, content.clone()))
            .flatten()
            .collect()
    }
}
