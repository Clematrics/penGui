use super::container::*;
use super::draw_commands::*;
use super::user_state::*;
use std::any::Any;
use std::hash::Hash;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct Id {
    pub name: Option<u32>,
    pub meta_id: Option<u32>,
}

pub trait Widget: Any + 'static {
    fn id(&self) -> Id;
    fn set_meta_id(&mut self, id: u32);
    fn data_mut(&mut self) -> Box<dyn Any>;
    fn data(&self) -> Box<dyn Any>;
    fn receive_event(&mut self, event: &UserEvent);
    fn draw_command(&self) -> DrawCommand;
    fn send_predecessor(&mut self, old: &dyn Widget);
    fn min_size(&self) -> (f32, f32);
    fn max_size(&self) -> (f32, f32);
    fn preferred_size(&self) -> (f32, f32);
}

pub trait UsableWidget<Result>
where
    Self: Widget + Copy,
{
    fn as_dyn_widget(&mut self) -> Box<dyn Widget> {
        Box::new(*self)
    }

    fn result(&self) -> Result;

    fn build<C: Container>(&mut self, container: &mut C) -> Result {
        let id = self.id();
        match container.old_widgets().get(&id) {
            None => {}
            Some(v) => match v {
                WidgetOrContainer::Widget(old_widget) => self.send_predecessor(old_widget.as_ref()),
                WidgetOrContainer::Container(_) => (),
            },
        };
        let r = *self;
        container
            .widgets()
            .insert(id, WidgetOrContainer::Widget(self.as_dyn_widget()));
        r.result()
    }
}
