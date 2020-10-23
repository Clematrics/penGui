use super::container::*;
use super::draw_commands::*;
use super::user_state::*;
use super::widget::*;
use std::collections;
use std::mem;
use std::any::Any;

pub struct Window {
    id : Id,
    widgets: collections::HashMap<Id, WidgetOrContainer>,
}

impl Window {
    pub fn new(id: Id) -> Window {
        Window {
            id,
            widgets: collections::HashMap::new(),
        }
    }
}

impl Widget for Window {
    fn id(&self) -> Id {
        self.id
    }

    fn set_meta_id(&mut self, id: u32) {

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

    fn draw_command(&mut self) -> Vec<DrawCommand> {
        let mut r = Vec::new();
        for (_, v) in self.widgets.iter_mut() {
            let mut commands = v.draw_command();
            r.append(&mut commands);
        }
        //mem::swap(&mut self.old_widgets, &mut self.widgets);
        self.widgets = collections::HashMap::new();
        r
    }

    fn send_predecessor(&mut self, old: &dyn Widget) {
        match old.data().downcast_ref::<Option<u32>>().unwrap() {
            Some(i) => *self.data_mut().downcast_mut().unwrap() = Some(i - 1),
            None => (),
        }
    }

}

impl Container for Window {
    
    fn widgets(&mut self) -> &mut collections::HashMap<Id, WidgetOrContainer> {
        &mut self.widgets
    }

    fn get(&self, id: &Id) -> Option<&WidgetOrContainer> {
        self.widgets.get(id)
    }

    fn get_mut(&mut self, id: &Id) -> Option<&mut WidgetOrContainer> {
        self.widgets.get_mut(id)
    }
}
