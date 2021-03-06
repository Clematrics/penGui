use nalgebra::*;

use crate::core::*;

/// The builder for a window, that can hold an unlimited number of widgets.
/// Display them in a finite space from top to bottom.
pub struct WindowBuilder<'a> {
    title: String,
    size: (f32, f32),
    transform: Similarity3<f32>,
    generator: Option<Box<dyn 'a + FnMut(&NodeReference)>>,
}

const BACKGROUND: (f32, f32, f32, f32) = (0.106, 0.125, 0.173, 1.);

impl<'a> WindowBuilder<'a> {
    pub fn new<F: 'a + FnMut(&NodeReference)>(generator: F) -> Self {
        WindowBuilder {
            title: "".to_string(),
            size: (5., 5.),
            transform: Similarity3::identity(),
            generator: Some(Box::new(generator)),
        }
    }

    pub fn title(self, title: String) -> Self {
        Self { title, ..self }
    }

    pub fn size(self, size: (f32, f32)) -> Self {
        Self { size, ..self }
    }

    pub fn transform(self, transform: Similarity3<f32>) -> Self {
        Self { transform, ..self }
    }
}

impl<'a> WidgetBuilder for WindowBuilder<'a> {
    type AchievedType = Window;
    type UpdateFeedback = ();
    type BuildFeedback = ();

    fn update(
        self,
        _metadata: &NodeMetadata,
        widget: &mut Self::AchievedType,
    ) -> Self::UpdateFeedback {
        widget.size = self.size;
        widget.title = self.title;

        widget.content.iter().for_each(|child| child.invalidate());
        widget.valid_index = 0;
    }

    fn create(self) -> Self::AchievedType {
        Window {
            title: self.title,
            size: self.size,
            content: Vec::new(),
            valid_index: 0,
        }
    }

    fn build(mut self, loc: CodeLocation, parent: &NodeReference) -> Self::BuildFeedback {
        let id = ComponentId::new::<Self::AchievedType>(loc);
        let mut generator = self.generator.take().unwrap();
        let transform = self.transform;
        let (node_ref, _) = parent.query::<Self::AchievedType>(id).update(self);
        node_ref.set_transform(transform);
        (generator)(&node_ref);
        node_ref.apply_to_widget::<Self::AchievedType, _>(|_, widget| {
            widget.content.retain(|child| child.is_valid())
        });
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
                node.layout(&LayoutQuery {
                    available_space: (Some(horizontal_space), Some(vertical_space)),
                    objectives: (Objective::Minimize, Objective::Minimize),
                })
            };

            node.set_size(response.size);
            node.set_transform(
                Similarity3::identity()
                    * Translation3::new(0., vertical_space - response.size.1, 0.),
            );

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
        list.list_transform =
            (metadata.transform * Translation3::new(0., 0., 0.01)).to_homogeneous();

        list.commands.push(quad(
            self.size.0,
            self.size.1,
            None,
            BACKGROUND,
            metadata.transform,
        ));

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
                PaddingBuilder::new((0.2, 100.2), FrameCounter::new()).build(loc!(), &ui);
                FrameCounter::new().build(loc!(), &ui);
                FrameCounter::new().build(loc!(), &ui);
            })
            .size((1., 1.))
            .build(loc!(), &ui.root);
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
                PaddingBuilder::new((100.2, 0.2), FrameCounter::new()).build(loc!(), &ui);
                FrameCounter::new().build(loc!(), &ui);
                FrameCounter::new().build(loc!(), &ui);
            })
            .size((1., 2.))
            .build(loc!(), &ui.root);
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
                PaddingBuilder::new((0.2, 0.2), FrameCounter::new()).build(loc!(), &ui);
                FrameCounter::new().build(loc!(), &ui);
                FrameCounter::new().build(loc!(), &ui);
            })
            .size((1., 2.))
            .build(loc!(), &ui.root);
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
                PaddingBuilder::new((10.2, 10.2), FrameCounter::new()).build(loc!(), &ui);
                FrameCounter::new().build(loc!(), &ui);
                FrameCounter::new().build(loc!(), &ui);
            })
            .size((1., 1.))
            .build(loc!(), &ui.root);
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
                PaddingBuilder::new((0.2, 0.2), FrameCounter::new()).build(loc!(), &ui);
                PaddingBuilder::new((0.2, 0.2), FrameCounter::new()).build(loc!(), &ui);
                FrameCounter::new().build(loc!(), &ui);
                PaddingBuilder::new((0.2, 0.2), FrameCounter::new()).build(loc!(), &ui);
                PaddingBuilder::new((0.2, 0.2), FrameCounter::new()).build(loc!(), &ui);
                PaddingBuilder::new((0.2, 0.2), FrameCounter::new()).build(loc!(), &ui);
                PaddingBuilder::new((0.2, 0.2), FrameCounter::new()).build(loc!(), &ui);
                FrameCounter::new().build(loc!(), &ui);
            })
            .size((1., 1.))
            .build(loc!(), &ui.root);
            let response = ui.generate_layout();
            ui.end_frame();
            assert_eq!(response.status.0, LayoutStatus::Ok);
            assert_eq!(response.status.1, LayoutStatus::Inconsistencies);
        }
    }
}
