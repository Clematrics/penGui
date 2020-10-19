use super::draw_commands::*;
use super::user_state::*;
use super::widget::*;
use std::collections;
use std::mem;
use std::rc::Rc;


pub struct Context {
    old_widgets: collections::HashMap<u32, Rc<dyn Widget>>,
    widgets: collections::HashMap<u32, Rc<dyn Widget>>,
    user_state: UserState,
}

impl Context {
    pub fn new(user_state: UserState) -> Context {
        Context {
            old_widgets: collections::HashMap::new(),
            widgets: collections::HashMap::new(),
            user_state: user_state,
        }
    }
    pub fn register_widget(&mut self, mut widget: Rc<dyn Widget>) -> () {
        let id = widget.id();
        match self.old_widgets.get(&id) {
            None => {}
            Some(widget2) => {
                widget.send_predecessor(widget2.as_ref());
                widget.receive_event(&self.user_state);
            }
        };
        self.widgets.insert(id, widget);
    }

    pub fn draw(&mut self) -> Vec<DrawCommand> {
        let mut r = Vec::new();
        for (_, widget) in self.widgets.iter_mut() {
            r.push(widget.draw_command());
        }
        mem::swap(&mut self.old_widgets, &mut self.widgets);
        self.widgets = collections::HashMap::new();
        r
    }

    pub fn get(&self, id: &u32) -> Option<&dyn Widget> {
        match self.widgets.get(id) {
            None => None,
            Some(v) => Some(v.as_ref()),
        }
    }
}
