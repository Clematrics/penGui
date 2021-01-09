use nalgebra::*;
use std::rc::Rc;

use crate::core::{
    CodeLocation, ComponentId, DrawCommand, DrawList, DrawMode, LayoutQuery, LayoutResponse,
    LayoutStatus, Node, NodeMetadata, NodeReference, Objective, Vertex, WidgetBuilder, WidgetLogic,
    WidgetQueryResult,
};

pub struct WindowBuilder {
    title: String,
    size: (f32, f32),
    generator: Option<Box<dyn Fn(NodeReference)>>,
}

impl WindowBuilder {
    pub fn new<F: 'static + Fn(NodeReference)>(generator: F) -> Self {
        WindowBuilder {
            title: "".to_string(),
            size: (5., 5.),
            generator: Some(Box::new(generator)),
        }
    }

    pub fn _title(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    pub fn _size(mut self, size: (f32, f32)) -> Self {
        self.size = size;
        self
    }
}

impl WidgetBuilder for WindowBuilder {
    type AchievedType = Window;
    type BuildFeedback = ();

    fn update(self, _metadata: &NodeMetadata, old: &mut Self::AchievedType) {
        old.title = self.title;
    }

    fn create(self) -> Self::AchievedType {
        Window {
            title: self.title,
            size: self.size,
            content: Vec::new(),
        }
    }

    fn build(mut self, loc: CodeLocation, parent: NodeReference) -> Self::BuildFeedback {
        let id = ComponentId::new::<Self::AchievedType>(loc);
        let generator = self.generator.take().unwrap_or_else(|| Box::new(|_| ()));
        let node_ref = parent
            .borrow_mut()
            .query::<Self::AchievedType>(id)
            .update(self);

        {
            let node_bis = node_ref.clone();
            let mut node = node_bis.borrow_mut();
            let (_, content) = node.borrow_parts();
            let window = content
                .as_any_mut()
                .downcast_mut::<Self::AchievedType>()
                .unwrap();
            window
                .content
                .iter()
                .for_each(|node_ref| node_ref.borrow_mut().metadata.invalid = true);
        }
        (generator)(node_ref.clone());
        {
            let mut node = node_ref.borrow_mut();
            let (_, content) = node.borrow_parts();
            let window = content
                .as_any_mut()
                .downcast_mut::<Self::AchievedType>()
                .unwrap();
            window
                .content
                .retain(|node_ref| !node_ref.borrow_mut().metadata.invalid);
        }
    }
}

pub struct Window {
    title: String,
    size: (f32, f32),
    content: Vec<NodeReference>,
}

impl WidgetLogic for Window {
    fn layout(&mut self, _query: &LayoutQuery) -> LayoutResponse {
        let (horizontal_space, mut vertical_space) = self.size;
        let mut status = (LayoutStatus::Ok, LayoutStatus::Ok);
        // For each component, compute the layout with all available space
        // but with the `Minimize` objective. The vertical space taken is
        // substracted from the remaining space
        for node in &mut self.content {
            let response = {
                node.borrow_mut().layout(&LayoutQuery {
                    available_space: (Some(horizontal_space), Some(vertical_space)),
                    objectives: (Objective::Minimize, Objective::Minimize),
                })
            };

            {
                let ref mut metadata = node.borrow_mut().metadata;
                metadata.size = response.size;
                metadata.position = (0., vertical_space - response.size.1, 0.);

            }

            vertical_space -= response.size.1;
            status.0 = LayoutStatus::and(status.0, response.status.0);
            status.1 = LayoutStatus::and(status.1, response.status.1);

            if vertical_space < 0. {
                // The window will ignore the components that cannot fit
                return LayoutResponse {
                    size: self.size,
                    status: (status.0, LayoutStatus::and(LayoutStatus::Inconsistencies, status.1)),
                }
            }
        }

        LayoutResponse {
            size: self.size,
            status,
        }
    }

    fn query(&mut self, id: ComponentId) -> WidgetQueryResult {
        let child = self
            .content
            .iter()
            .find(|&other| (*other).borrow().metadata.id == id)
            .map(Rc::clone);
        match child {
            Some(node_ref) => WidgetQueryResult::Initialized(node_ref),
            None => {
                let node_ref = Node::new_reference(id);
                self.content.push(node_ref.clone());
                WidgetQueryResult::Uninitialized(node_ref)
            }
        }
    }

    fn draw(&self, _metadata: &NodeMetadata) -> DrawList {
        let mut list = DrawList::new();
        self.content.iter().for_each(|node| {
            list.list.push(node.borrow_mut().draw());
        });
        let color = [42. / 256., 60. / 256., 101. / 256., 1.];
        let tex_uv = [0., 0.];
        list.list_transform =
            nalgebra::Translation3::from(nalgebra::Vector3::new(0., 0., 0.001)).to_homogeneous();
        list.commands.push(DrawCommand {
            vertex_buffer: vec![
                Vertex {
                    position: [0., 0., 0.],
                    color,
                    tex_uv,
                },
                Vertex {
                    position: [self.size.0, 0., 0.],
                    color,
                    tex_uv,
                },
                Vertex {
                    position: [0., self.size.1, 0.],
                    color,
                    tex_uv,
                },
                Vertex {
                    position: [self.size.0, self.size.1, 0.],
                    color,
                    tex_uv,
                },
            ],
            index_buffer: vec![0, 1, 2, 1, 2, 3],
            draw_mode: DrawMode::Lines,
            uniforms: Default::default(),
        });
        list
    }
}
