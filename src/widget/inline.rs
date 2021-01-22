use nalgebra::*;

use crate::core::*;

/// The builder for a window, that can hold an unlimited number of widgets.
/// Display them in a finite space from top to bottom.
pub struct InlineBuilder<'a> {
    generator: Option<Box<dyn 'a + FnMut(&NodeReference)>>,
}

impl<'a> InlineBuilder<'a> {
    pub fn new<F: 'a + FnMut(&NodeReference)>(generator: F) -> Self {
        Self {
            generator: Some(Box::new(generator)),
        }
    }
}

impl<'a> WidgetBuilder for InlineBuilder<'a> {
    type AchievedType = Inline;
    type UpdateFeedback = ();
    type BuildFeedback = ();

    fn update(
        self,
        _metadata: &NodeMetadata,
        widget: &mut Self::AchievedType,
    ) -> Self::UpdateFeedback {
        widget.content.iter().for_each(|child| child.invalidate());
        widget.valid_index = 0;
    }

    fn create(self) -> Self::AchievedType {
        Self::AchievedType {
            content: Vec::new(),
            valid_index: 0,
        }
    }

    fn build(mut self, loc: CodeLocation, parent: &NodeReference) -> Self::BuildFeedback {
        let id = ComponentId::new::<Self::AchievedType>(loc);
        let mut generator = self.generator.take().unwrap();
        let (node_ref, _) = parent.query::<Self::AchievedType>(id).update(self);
        (generator)(&node_ref);
        node_ref.apply_to_widget::<Self::AchievedType, _>(|_, widget| {
            widget.content.retain(|child| child.is_valid())
        });
    }
}

/// Internal window structure
pub struct Inline {
    content: Vec<NodeReference>,
    valid_index: usize,
}

const WIDGET_SEPARATOR: f32 = 0.5;

impl WidgetLogic for Inline {
    fn layout(&mut self, query: &LayoutQuery) -> LayoutResponse {
        let (mut horizontal_space, vertical_space) = query.available_space;
        let mut status = (LayoutStatus::Ok, LayoutStatus::Ok);
        let mut cursor = 0.;
        let mut height = 0.;
        // For each component, compute the layout with all available space
        // but with the `Minimize` objective. The vertical space taken is
        // substracted from the remaining space
        for node in &mut self.content {
            let response = {
                node.layout(&LayoutQuery {
                    available_space: (horizontal_space, vertical_space),
                    objectives: (Objective::Minimize, Objective::Minimize),
                })
            };

            node.set_size(response.size);
            node.set_transform(Similarity3::identity() * Translation3::new(cursor, 0., 0.));
            cursor += response.size.0 + WIDGET_SEPARATOR;
            height = height.max(response.size.1);

            horizontal_space = horizontal_space.map(|x| x - response.size.0 + WIDGET_SEPARATOR);
            status.0 = LayoutStatus::and(status.0, response.status.0);
            status.1 = LayoutStatus::and(status.1, response.status.1);

            if horizontal_space.unwrap_or(0.) < 0. {
                // The window will ignore the components that cannot fit
                return LayoutResponse {
                    size: (cursor, height),
                    status: (
                        status.0,
                        LayoutStatus::and(LayoutStatus::Inconsistencies, status.1),
                    ),
                };
            }
        }

        LayoutResponse {
            size: (cursor, height),
            status,
        }
    }

    fn query(&mut self, metadata: &NodeMetadata, id: ComponentId) -> WidgetQueryResult {
        let child = self
            .content
            .iter()
            .enumerate()
            .find(|(_, other)| other.has_id(id))
            .map(|(index, other)| (index, other.clone()));
        let (index, result) = match child {
            Some((index, node_ref)) => (index, WidgetQueryResult::Initialized(node_ref)),
            None => {
                let node_ref = Node::new_reference(id, &metadata.ui_properties);
                self.content.push(node_ref.clone());
                (
                    self.content.len() - 1,
                    WidgetQueryResult::Uninitialized(node_ref),
                )
            }
        };
        self.content.swap(self.valid_index, index);
        self.valid_index += 1;
        result
    }

    fn draw(&self, metadata: &NodeMetadata) -> DrawList {
        let mut list = DrawList::new();
        self.content.iter().for_each(|node| {
            list.list.push(node.draw());
        });
        list.list_transform = metadata.transform.to_homogeneous();

        // list.commands.push(quad(
        //     metadata.size.0,
        //     metadata.size.1,
        //     None,
        //     (1., 0., 0., 1.),
        //     metadata.transform,
        // ));

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
