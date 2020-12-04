use nalgebra::*;
use std::rc::Rc;

use crate::core::{
    CodeLocation, ComponentId, DrawList, Node, NodeMetadata, NodeReference, WidgetBuilder,
    WidgetLogic, WidgetQueryResult,
};
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

    fn draw(&self, position: Point3<f32>, size: (f32, f32)) -> DrawList {
        let widget_size = (size.0 - self.padding.0, size.1 - self.padding.1);
        self.content
            .as_ref()
            .unwrap()
            .borrow()
            .draw(position, widget_size)
    }
}
