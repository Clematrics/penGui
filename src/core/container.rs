use super::draw_commands::*;
use super::user_state::*;
use super::widget::*;
use nalgebra::*;
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
    fn draw_commands(
        &mut self,
        unit_x: Vector3<f32>,
        unit_y: Vector3<f32>,
        position: Point3<f32>,
        size: (f32, f32),
        uniforms: &Vec<Uniform>,
    ) -> Vec<DrawCommand> {
        match self {
            WidgetOrContainer::Widget(w) => {
                w.draw_commands(unit_x, unit_y, position, size, uniforms)
            }
            WidgetOrContainer::Container(c) => {
                c.draw_commands(unit_x, unit_y, position, size, uniforms)
            }
        }
    }
    fn send_predecessor(&mut self, old: &mut dyn Widget) {
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
    //fn widgets(&mut self) -> &mut collections::HashMap<Id, WidgetOrContainer>;
    fn get(&self, id: &Id) -> Option<&WidgetOrContainer>;
    fn get_mut(&mut self, id: &Id) -> Option<&mut WidgetOrContainer>;
    /*fn send_predecessor(&mut self, old: &mut dyn Widget) {
        match old.data_mut().downcast_mut::<collections::HashMap<Id, WidgetOrContainer>>() {
            Some(w) => {
                let d = w.drain();
                *self.widgets() = d.collect();
            },
            None => (),
        }
    }*/
}

pub trait UsableContainer: Container {
    fn add_widget<W: UsableWidget>(&mut self, w: &mut W) -> W::Result;
}
