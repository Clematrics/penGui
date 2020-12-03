use nalgebra::*;
use std::rc::Rc;

use crate::core::{
    CodeLocation, ComponentId, DrawList, Node, NodeMetadata, NodeReference, WidgetBase,
    WidgetBuilder, WidgetQueryResult,
};

pub struct PaddingBuilder {
    padding: (f32, f32),
    generator: Option<Box<dyn Fn(NodeReference)>>,
}

impl PaddingBuilder {
    pub fn new<F: 'static + Fn(NodeReference)>(padding: (f32, f32), generator: F) -> Self {
        PaddingBuilder {
            padding,
            generator: Some(Box::new(generator)),
        }
    }

    pub fn padding(mut self, size: (f32, f32)) -> Self {
        self.padding = size;
        self
    }
}

impl WidgetBuilder for PaddingBuilder {
    type AchievedType = Padding;
    type BuildFeedback = ();

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
        let generator = self.generator.take().unwrap_or(Box::new(|_| ()));
        let node_ref = parent
            .borrow_mut()
            .query::<Self::AchievedType>(id)
            .update(self);
        // .update(&|_, _| {})
        // .or_create(Padding {
        //     title: self.title,
        //     content: Vec::new(),
        // });
        (generator)(node_ref);
    }
}

pub struct Padding {
    padding: (f32, f32),
    content: Option<NodeReference>,
}

impl WidgetBase for Padding {
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

    fn draw(&self, position: Point3<f32>, size: (f32, f32)) -> DrawList {
        let widget_size = (size.0 - self.padding.0, size.1 - self.padding.1);
        self.content
            .as_ref()
            .unwrap()
            .borrow()
            .draw(position, widget_size)
    }
}
