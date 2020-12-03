use nalgebra::Point3;

use crate::core::{
    CodeLocation, ComponentId, DrawCommand, DrawList, DrawMode, NodeMetadata, NodeReference,
    TextureId, Uniforms, Vertex, Node, WidgetBase, WidgetBuilder, WidgetQueryResult,
};

pub struct Padding<ContentType: WidgetBuilder + WidgetBase> {
    content: Option<ContentType>,
    padding: (f32, f32),
}

impl<ContentType: WidgetBuilder + WidgetBase> Padding<ContentType> {
    pub fn new(content: ContentType) -> Self {
        Self {
            content: Some(content),
            padding: (0., 0.),
        }
    }

    pub fn padding(mut self, padding: (f32, f32)) -> Self {
        self.padding = padding;
        self
    }

    /*pub fn position(mut self, pos: (f32, f32, f32)) -> Self {
        self.position = pos;
        self
    }*/
}

impl<ContentType: WidgetBuilder + WidgetBase + 'static> WidgetBuilder for Padding<ContentType> {
    type AchievedType = Padding<ContentType>;
    type BuildFeedback = ContentType::BuildFeedback;

    fn update(self, _metadata: &NodeMetadata, old: &mut Self::AchievedType) {
        old.content = self.content;
        old.padding = self.padding;
    }

    fn create(self) -> Self::AchievedType {
        self
    }

    fn build(self, loc: CodeLocation, parent: NodeReference) -> Self::BuildFeedback {
        let id = ComponentId::new::<Self::AchievedType>(loc);
        let content = self.content.take();
        let node_ref = parent
            .borrow_mut()
            .query::<Self::AchievedType>(id)
            .update(self);
        content.unwrap().build(loc, node_ref)
    }
}

impl<ContentType: WidgetBuilder + WidgetBase> WidgetBase for Padding<ContentType> {
    fn draw(&self, position: Point3<f32>, size: (f32, f32)) -> DrawList {
        self.content.take().unwrap()
            .draw(position, (size.0 - self.padding.0, size.1 - self.padding.1))
    }
}
