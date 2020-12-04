use nalgebra::*;
use std::rc::Rc;

use crate::core::{
    CodeLocation, ComponentId, DrawList, Node, NodeMetadata, NodeReference, WidgetBuilder,
    WidgetLogic, WidgetQueryResult,
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
            size: (400., 400.),
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
            content: Vec::new(),
        }
    }

    fn build(mut self, loc: CodeLocation, parent: NodeReference) -> Self::BuildFeedback {
        let id = ComponentId::new::<Self::AchievedType>(loc);
        let generator = self.generator.take().unwrap_or(Box::new(|_| ()));
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
                .retain(|node_ref| node_ref.borrow_mut().metadata.invalid == false);
        }
    }
}

pub struct Window {
    title: String,
    content: Vec<NodeReference>,
}

impl WidgetLogic for Window {
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

    fn draw(&self, position: Point3<f32>, size: (f32, f32)) -> DrawList {
        let unit_y = Vector3::new(0., 1., 0.);

        let widget_size = (size.0, size.1 / (self.content.len()) as f32);

        let mut current_pos = {
            let top_side = position + unit_y * size.1 / 2.;
            top_side - unit_y * widget_size.1 / 2.
        };

        let mut list = DrawList::new();
        self.content.iter().for_each(|node| {
            list.list
                .push(node.borrow_mut().draw(current_pos, widget_size));
            current_pos -= unit_y * widget_size.1
        });
        list
    }
}
