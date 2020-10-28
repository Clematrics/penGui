use super::container::*;
use super::draw_commands::*;
use super::user_state::*;
use super::widget::*;
use nalgebra;
use std::any::Any;
use std::collections;
use std::mem;

pub struct Window {
    id: Id,
    widgets: collections::HashMap<Id, WidgetOrContainer>,
    widgets_order: Vec<Id>,
}

impl Window {
    pub fn new(id: Id) -> Window {
        Window {
            id,
            widgets: collections::HashMap::new(),
            widgets_order: Vec::new(),
        }
    }
}

impl Widget for Window {
    fn id(&self) -> Id {
        self.id
    }

    fn set_meta_id(&mut self, id: u32) {
        self.id.meta_id = Some(id);
    }

    fn data_mut(&mut self) -> &mut dyn Any {
        &mut self.widgets
    }

    fn data(&self) -> &dyn Any {
        &self.widgets
    }

    fn receive_event(&mut self, event: &UserEvent) {
        for (_, v) in self.widgets.iter_mut() {
            //TODO: condition here
            v.receive_event(event);
        }
    }

    fn min_size(&self) -> (f32, f32) {
        (0., 0.)
    }

    fn max_size(&self) -> (f32, f32) {
        (0., 0.)
    }

    fn preferred_size(&self) -> (f32, f32) {
        (0., 0.)
    }

    fn draw_commands(
        &mut self,
        unit_x: nalgebra::Vector3<f32>,
        unit_y: nalgebra::Vector3<f32>,
        position: nalgebra::Point3<f32>,
        size: (f32, f32),
        uniforms: &Vec<Uniform>,
    ) -> Vec<DrawCommand> {
        let mut r = Vec::new();
        for id in self.widgets_order.iter_mut() {
            let mut commands =
                self.widgets.get_mut(id).unwrap().draw_commands(unit_x, unit_y, position, size, uniforms);
            r.append(&mut commands);
        }
        //mem::swap(&mut self.old_widgets, &mut self.widgets);
        self.widgets = collections::HashMap::new();
        r
    }

    fn send_predecessor(&mut self, old: &mut dyn Widget) {
        match old
            .data_mut()
            .downcast_mut::<collections::HashMap<Id, WidgetOrContainer>>()
        {
            Some(w) => {
                let d = w.drain();
                self.widgets = d.collect();
            }
            None => (),
        }
    }
}

impl Container for Window {
    fn get(&self, id: &Id) -> Option<&WidgetOrContainer> {
        self.widgets.get(id)
    }

    fn get_mut(&mut self, id: &Id) -> Option<&mut WidgetOrContainer> {
        self.widgets.get_mut(id)
    }
}

impl UsableContainer for Window {
    fn add_widget<W: UsableWidget>(&mut self, w: &mut W) -> W::Result {
        let id = w.id();
        match self.get_mut(&id) {
            None => {}
            Some(old_widget) => w.send_predecessor(old_widget),
        };
        let r = *w;
        self.widgets
            .insert(id, WidgetOrContainer::Widget(w.as_dyn_widget()));
        self.widgets_order.push(id);
        r.result()
    }
}
