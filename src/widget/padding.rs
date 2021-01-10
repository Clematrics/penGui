use nalgebra::*;
use std::rc::Rc;

use crate::core::*;
use crate::loc;

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

    pub fn padding(mut self, size: (f32, f32)) -> Self {
        self.padding = size;
        self
    }
}

impl<T: WidgetBuilder> WidgetBuilder for PaddingBuilder<T> {
    type AchievedType = Padding;
    type BuildFeedback = T::BuildFeedback;

    fn update(self, _metadata: &NodeMetadata, old: &mut Self::AchievedType) {
        old.padding = self.padding;
    }

    fn create(self) -> Self::AchievedType {
        Padding {
            padding: self.padding,
            content: None,
        }
    }

    fn build(mut self, loc: CodeLocation, parent: NodeReference) -> Self::BuildFeedback {
        let id = ComponentId::new::<Self::AchievedType>(loc);
        let content = self.content.take().unwrap();
        let node_ref = parent
            .borrow_mut()
            .query::<Self::AchievedType>(id)
            .update(self);

        content.build(loc!(), node_ref)
    }
}

pub struct Padding {
    padding: (f32, f32),
    content: Option<NodeReference>,
}

impl WidgetLogic for Padding {
    fn query(&mut self, id: ComponentId) -> WidgetQueryResult {
        let child = {
            match &self.content {
                Some(other) => {
                    if other.borrow().metadata.id == id {
                        Some(Rc::clone(&other))
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
                let node_ref = Node::new_reference(id);
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

        let response = self
            .content
            .as_ref()
            .unwrap()
            .borrow_mut()
            .layout(&LayoutQuery {
                available_space: inner_space,
                objectives: query.objectives,
            });

        {
            let ref mut metadata = self.content.as_ref().unwrap().borrow_mut().metadata;
            metadata.size = response.size;
            metadata.position = (self.padding.0, self.padding.1, 0.);
        }

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
        list.list
            .push(self.content.as_ref().unwrap().borrow().draw());
        let color = [1., 0., 0., 1.];
        let tex_uv = [0., 0.];
        let mut uniforms = Uniforms::new();
        let (x, y, z) = metadata.position;
        list.list_transform =
            nalgebra::Translation3::from(nalgebra::Vector3::new(x, y, z)).to_homogeneous();
        uniforms.model_matrix = list.list_transform;
        list.commands.push(DrawCommand {
            vertex_buffer: vec![
                Vertex {
                    position: [0., 0., 0.],
                    color,
                    tex_uv,
                },
                Vertex {
                    position: [metadata.size.0, 0., 0.],
                    color,
                    tex_uv,
                },
                Vertex {
                    position: [0., metadata.size.1, 0.],
                    color,
                    tex_uv,
                },
                Vertex {
                    position: [metadata.size.0, metadata.size.1, 0.],
                    color,
                    tex_uv,
                },
            ],
            index_buffer: vec![0, 1, 2, 1, 2, 3],
            draw_mode: DrawMode::Lines,
            uniforms: uniforms,
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
        self.content
            .iter()
            .map(|content| {
                content
                    .borrow()
                    .interaction_distance(&new_ray, content.clone())
            })
            .flatten()
            .collect()
    }
}
