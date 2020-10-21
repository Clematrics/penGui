use super::draw_commands::*;
use super::user_state::*;
use super::widget::*;
use std::collections;
use std::mem;

pub enum WidgetOrContainer {
    Widget(Box<dyn Widget>),
    Container(Box<dyn Container>),
}

pub trait Container {
    fn old_widgets(&mut self) -> &mut collections::HashMap<Id, WidgetOrContainer>;
    fn widgets(&mut self) -> &mut collections::HashMap<Id, WidgetOrContainer>;
    fn new_event(&mut self, event: &UserEvent);

    fn draw(&mut self) -> Vec<DrawCommand>;

    fn get(&self, id: &Id) -> Option<&WidgetOrContainer>;

    fn get_mut(&mut self, id: &Id) -> Option<&mut WidgetOrContainer>;
}
