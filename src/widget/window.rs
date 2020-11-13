use std::rc::Rc;

use crate::core::{
    CodeLocation, ComponentId, Node, NodeMetadata, NodeReference, WidgetBase, WidgetBuilder,
    WidgetQueryResult,
};

struct WindowBuilder<F: Fn(NodeReference) -> ()> {
    title: String,
    size: (f32, f32),
    generator: Option<F>,
}

impl<F> WindowBuilder<F>
where
    F: Fn(NodeReference) -> (),
{
    pub fn new(generator: F) -> Self {
        WindowBuilder {
            title: "".to_string(),
            size: (400., 400.),
            generator: Some(generator),
        }
    }

    pub fn title(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    pub fn size(mut self, size: (f32, f32)) -> Self {
        self.size = size;
        self
    }
}

impl<F> WidgetBuilder for WindowBuilder<F>
where
    F: Fn(NodeReference) -> (),
{
    type AchievedType = Window;
    type BuildFeedback = ();

    fn update(self, metadata: &NodeMetadata, old: &mut Self::AchievedType) {}

    fn create(self) -> Self::AchievedType {
        Window {
            title: self.title,
            content: Vec::new(),
        }
    }

    fn build(mut self, loc: CodeLocation, parent: NodeReference) -> Self::BuildFeedback {
        let id = ComponentId::new::<Self::AchievedType>(loc);
        let generator = self.generator.take().expect("Bug detecteds");
        let node_ref = parent
            .borrow_mut()
            .query::<Self::AchievedType>(id)
            .update(self);
        // .update(&|_, _| {})
        // .or_create(Window {
        //     title: self.title,
        //     content: Vec::new(),
        // });
        (generator)(node_ref);
    }
}

struct Window {
    title: String,
    content: Vec<NodeReference>,
}

impl WidgetBase for Window {
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
}
