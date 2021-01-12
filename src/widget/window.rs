use nalgebra::*;
use std::rc::Rc;

use crate::core::*;

/// The builder for a window, that can hold an unlimited number of widgets.
/// Display them in a finite space from top to bottom.
pub struct WindowBuilder {
    title: String,
    size: (f32, f32),
    generator: Option<Box<dyn FnMut(NodeReference)>>,
}

impl WindowBuilder {
    pub fn new<F: 'static + FnMut(NodeReference)>(generator: F) -> Self {
        WindowBuilder {
            title: "".to_string(),
            size: (5., 5.),
            generator: Some(Box::new(generator)),
        }
    }

    pub fn title(self, title: String) -> Self {
        Self { title, ..self }
    }

    pub fn size(self, size: (f32, f32)) -> Self {
        Self { size, ..self }
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
            valid_index: 0,
        }
    }

    fn build(mut self, loc: CodeLocation, parent: NodeReference) -> Self::BuildFeedback {
        let id = ComponentId::new::<Self::AchievedType>(loc);
        let mut generator = self.generator.take().unwrap_or_else(|| Box::new(|_| ()));
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
            window.valid_index = 0;
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

/// Internal window structure
pub struct Window {
    title: String,
    size: (f32, f32),
    content: Vec<NodeReference>,
    valid_index: usize,
}

const WIDGET_SEPARATOR: f32 = 0.5;

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
                let metadata = &mut node.borrow_mut().metadata;
                metadata.size = response.size;
                metadata.position = (0., vertical_space - response.size.1, 0.);
            }

            vertical_space -= response.size.1 + WIDGET_SEPARATOR;
            status.0 = LayoutStatus::and(status.0, response.status.0);
            status.1 = LayoutStatus::and(status.1, response.status.1);

            if vertical_space < 0. {
                // The window will ignore the components that cannot fit
                return LayoutResponse {
                    size: self.size,
                    status: (
                        status.0,
                        LayoutStatus::and(LayoutStatus::Inconsistencies, status.1),
                    ),
                };
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
            .enumerate()
            .find(|(_, other)| (*other).borrow().metadata.id == id)
            .map(|(index, other)| (index, Rc::clone(other)));
        let (index, result) = match child {
            Some((index, node_ref)) => (index, WidgetQueryResult::Initialized(node_ref)),
            None => {
                let node_ref = Node::new_reference(id);
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
            list.list.push(node.borrow_mut().draw());
        });
        let color = [42. / 256., 60. / 256., 101. / 256., 1.];
        let tex_uv = [0., 0.];
        let (x, y, z) = metadata.position;
        list.list_transform =
            nalgebra::Translation3::from(nalgebra::Vector3::new(x, y, z + 0.001)).to_homogeneous();
        let mut uniforms = Uniforms::new();
        uniforms.model_matrix =
            nalgebra::Translation3::from(nalgebra::Vector3::new(x, y, z)).to_homogeneous();

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
            uniforms,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    use widget::*;
    #[test]
    fn window_layout_error_1() {
        let mut ui = Interface::new();
        for _ in 0..12 {
            ui.new_frame();
            WindowBuilder::new(move |ui| {
                PaddingBuilder::new((0.2, 100.2), FrameCounter::new()).build(loc!(), ui.clone());
                FrameCounter::new().build(loc!(), ui.clone());
                FrameCounter::new().build(loc!(), ui.clone());
            })
            .size((1., 1.))
            .build(loc!(), ui.root.clone());
            let response = ui.generate_layout();
            ui.end_frame();
            assert_eq!(response.status.0, LayoutStatus::Ok);
            assert_eq!(response.status.1, LayoutStatus::Inconsistencies);
        }
    }
    #[test]
    fn window_layout_error_2() {
        let mut ui = Interface::new();
        for _ in 0..12 {
            ui.new_frame();
            WindowBuilder::new(move |ui| {
                PaddingBuilder::new((100.2, 0.2), FrameCounter::new()).build(loc!(), ui.clone());
                FrameCounter::new().build(loc!(), ui.clone());
                FrameCounter::new().build(loc!(), ui.clone());
            })
            .size((1., 2.))
            .build(loc!(), ui.root.clone());
            let response = ui.generate_layout();
            ui.end_frame();
            assert_eq!(response.status.0, LayoutStatus::Inconsistencies);
            assert_eq!(response.status.1, LayoutStatus::Ok);
        }
    }
    #[test]
    fn window_layout_error_3() {
        let mut ui = Interface::new();
        for _ in 0..12 {
            ui.new_frame();
            WindowBuilder::new(move |ui| {
                PaddingBuilder::new((0.2, 0.2), FrameCounter::new()).build(loc!(), ui.clone());
                FrameCounter::new().build(loc!(), ui.clone());
                FrameCounter::new().build(loc!(), ui.clone());
            })
            .size((1., 2.))
            .build(loc!(), ui.root.clone());
            let response = ui.generate_layout();
            ui.end_frame();
            assert_eq!(response.status.0, LayoutStatus::Ok);
            assert_eq!(response.status.1, LayoutStatus::Ok);
        }
    }
    #[test]
    fn window_layout_error_4() {
        let mut ui = Interface::new();
        for _ in 0..12 {
            ui.new_frame();
            WindowBuilder::new(move |ui| {
                PaddingBuilder::new((10.2, 10.2), FrameCounter::new()).build(loc!(), ui.clone());
                FrameCounter::new().build(loc!(), ui.clone());
                FrameCounter::new().build(loc!(), ui.clone());
            })
            .size((1., 1.))
            .build(loc!(), ui.root.clone());
            let response = ui.generate_layout();
            ui.end_frame();
            assert_eq!(response.status.0, LayoutStatus::Inconsistencies);
            assert_eq!(response.status.1, LayoutStatus::Inconsistencies);
        }
    }
    #[test]
    fn window_layout_error_5() {
        let mut ui = Interface::new();
        for _ in 0..12 {
            ui.new_frame();
            WindowBuilder::new(move |ui| {
                PaddingBuilder::new((0.2, 0.2), FrameCounter::new()).build(loc!(), ui.clone());
                PaddingBuilder::new((0.2, 0.2), FrameCounter::new()).build(loc!(), ui.clone());
                FrameCounter::new().build(loc!(), ui.clone());
                PaddingBuilder::new((0.2, 0.2), FrameCounter::new()).build(loc!(), ui.clone());
                PaddingBuilder::new((0.2, 0.2), FrameCounter::new()).build(loc!(), ui.clone());
                PaddingBuilder::new((0.2, 0.2), FrameCounter::new()).build(loc!(), ui.clone());
                PaddingBuilder::new((0.2, 0.2), FrameCounter::new()).build(loc!(), ui.clone());
                FrameCounter::new().build(loc!(), ui.clone());
            })
            .size((1., 1.))
            .build(loc!(), ui.root.clone());
            let response = ui.generate_layout();
            ui.end_frame();
            assert_eq!(response.status.0, LayoutStatus::Ok);
            assert_eq!(response.status.1, LayoutStatus::Inconsistencies);
        }
    }
}
