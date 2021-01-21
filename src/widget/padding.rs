use nalgebra::*;

use crate::core::*;
use crate::loc;

/// A widget that can contain another one, but adjusting its layout with a small padding
/// on each side. An outline is displayed to show it
pub struct PaddingBuilder<T: WidgetBuilder> {
    padding: (f32, f32),
    content: Option<T>,
}

impl<T: WidgetBuilder> PaddingBuilder<T> {
    pub fn new(padding: (f32, f32), content: T) -> Self {
        PaddingBuilder {
            padding,
            content: Some(content),
        }
    }

    pub fn padding(self, padding: (f32, f32)) -> Self {
        Self { padding, ..self }
    }
}

impl<T: WidgetBuilder> WidgetBuilder for PaddingBuilder<T> {
    type AchievedType = Padding;
    type UpdateFeedback = ();
    type BuildFeedback = T::BuildFeedback;

    fn update(
        self,
        _metadata: &NodeMetadata,
        widget: &mut Self::AchievedType,
    ) -> Self::UpdateFeedback {
        widget.padding = self.padding;
    }

    fn create(self) -> Self::AchievedType {
        Padding {
            padding: self.padding,
            content: None,
        }
    }

    fn build(mut self, loc: CodeLocation, parent: &NodeReference) -> Self::BuildFeedback {
        let id = ComponentId::new::<Self::AchievedType>(loc);
        let content = self.content.take().unwrap();
        let (node_ref, _) = parent.query::<Self::AchievedType>(id).update(self);

        content.build(loc!(), &node_ref)
    }
}

pub struct Padding {
    padding: (f32, f32),
    content: Option<NodeReference>,
}

impl WidgetLogic for Padding {
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
        let mut status = (LayoutStatus::Ok, LayoutStatus::Ok);
        let inner_space = (
            query.available_space.0.map(|x| {
                if x - 2. * self.padding.0 >= 0. {
                    x - 2. * self.padding.0
                } else {
                    status.0 = LayoutStatus::Inconsistencies;
                    0.
                }
            }),
            query.available_space.1.map(|y| {
                if y - 2. * self.padding.1 >= 0. {
                    y - 2. * self.padding.1
                } else {
                    status.1 = LayoutStatus::Inconsistencies;
                    0.
                }
            }),
        );

        let response = self.content.as_ref().unwrap().layout(&LayoutQuery {
            available_space: inner_space,
            objectives: query.objectives,
        });

        let node = self.content.as_ref().unwrap();
        node.set_size(response.size);
        node.set_transform(
            Similarity3::identity() * Translation3::new(self.padding.0, self.padding.1, 0.),
        );

        let width = response.size.0 + 2. * self.padding.0;
        let height = response.size.1 + 2. * self.padding.1;

        LayoutResponse {
            size: (width, height),
            status: (
                LayoutStatus::and(response.status.0, status.0),
                LayoutStatus::and(response.status.1, status.1),
            ),
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
