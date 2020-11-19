use crate::core::{
    CodeLocation, ComponentId, NodeMetadata, NodeReference, WidgetBase, WidgetBuilder,
};

struct Button {
    label: String,
    color: (f32, f32, f32, f32),
}

impl Button {
    fn _new(label: String) -> Self {
        Self {
            label,
            color: (0., 0.4, 1., 1.),
        }
    }

    fn _color(mut self, color: (f32, f32, f32, f32)) -> Self {
        self.color = color;
        self
    }
}

impl WidgetBuilder for Button {
    type AchievedType = Button;
    type BuildFeedback = bool;

    fn update(self, _metadata: &NodeMetadata, old: &mut Self::AchievedType) {
        old.label = self.label;
        old.color = self.color;
    }

    fn create(self) -> Self::AchievedType {
        self
    }

    fn build(self, loc: CodeLocation, parent: NodeReference) -> Self::BuildFeedback {
        let id = ComponentId::new::<Self::AchievedType>(loc);
        // let update_fn = |_, button: &mut Self::AchievedType| {
        // 	button.label = self.label;
        //     button.color = self.color;
        // };

        parent
            .borrow_mut()
            .query::<Self::AchievedType>(id)
            .update(self);
        // .update(&|_, button: &mut Self::AchievedType| {
        // })
        // .or_create(self);
        true
    }
}

impl WidgetBase for Button {}
