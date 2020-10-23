use super::draw_commands::*;
use super::user_state::*;
use super::widget::*;
use std::any::Any;
use std::collections;

pub enum WidgetOrContainer {
    Widget(Box<dyn Widget>),
    Container(Box<dyn Container>),
}

impl Widget for WidgetOrContainer {
    fn id(&self) -> Id {
        match self {
            WidgetOrContainer::Widget(w) => w.id(),
            WidgetOrContainer::Container(c) => c.id(),
        }
    }
    fn set_meta_id(&mut self, id: u32) {
        match self {
            WidgetOrContainer::Widget(w) => w.set_meta_id(id),
            WidgetOrContainer::Container(c) => c.set_meta_id(id),
        }
    }
    fn data_mut(&mut self) -> &mut dyn Any {
        match self {
            WidgetOrContainer::Widget(w) => w.data_mut(),
            WidgetOrContainer::Container(c) => c.data_mut(),
        }
    }
    fn data(&self) -> &dyn Any {
        match self {
            WidgetOrContainer::Widget(w) => w.data(),
            WidgetOrContainer::Container(c) => c.data(),
        }
    }
    fn receive_event(&mut self, event: &UserEvent) {
        match self {
            WidgetOrContainer::Widget(w) => w.receive_event(event),
            WidgetOrContainer::Container(c) => c.receive_event(event),
        }
    }
    fn draw_command(&mut self) -> Vec<DrawCommand> {
        match self {
            WidgetOrContainer::Widget(w) => w.draw_command(),
            WidgetOrContainer::Container(c) => c.draw_command(),
        }
    }
    fn send_predecessor(&mut self, old: &dyn Widget) {
        match self {
            WidgetOrContainer::Widget(w) => w.send_predecessor(old),
            WidgetOrContainer::Container(c) => c.send_predecessor(old),
        }
    }
    fn min_size(&self) -> (f32, f32) {
        match self {
            WidgetOrContainer::Widget(w) => w.min_size(),
            WidgetOrContainer::Container(c) => c.min_size(),
        }
    }
    fn max_size(&self) -> (f32, f32) {
        match self {
            WidgetOrContainer::Widget(w) => w.max_size(),
            WidgetOrContainer::Container(c) => c.max_size(),
        }
    }
    fn preferred_size(&self) -> (f32, f32) {
        match self {
            WidgetOrContainer::Widget(w) => w.preferred_size(),
            WidgetOrContainer::Container(c) => c.preferred_size(),
        }
    }
}

pub trait Container: Widget {
    fn widgets(&mut self) -> &mut collections::HashMap<Id, WidgetOrContainer>;
    fn get(&self, id: &Id) -> Option<&WidgetOrContainer>;
    fn get_mut(&mut self, id: &Id) -> Option<&mut WidgetOrContainer>;
}
