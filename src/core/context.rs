use super::draw_commands::*;
use super::user_state::*;
use super::widget::*;
use std::collections;
use std::mem;

pub struct Context<W: Widget> {
    old_widgets: collections::HashMap<u32, W>,
    widgets: collections::HashMap<u32, W>,
    user_state: UserState,
}

impl<W: Widget> Context<W> {
    pub fn new(user_state: UserState) -> Context<W> {
        Context {
            old_widgets: collections::HashMap::new(),
            widgets: collections::HashMap::new(),
            user_state: user_state,
        }
    }
    pub fn register_widget(&mut self, mut widget: W) -> () {
        let id = widget.id();
        match self.old_widgets.get_mut(&id) {
            None => {}
            Some(widget2) => {
                widget.send_predecessor(widget2);
                widget.receive_event(&self.user_state);
            }
        };
        self.widgets.insert(id, widget);
    }

    pub fn draw(&mut self) -> Vec<DrawCommand> {
        let mut r = Vec::new();
        for (id, widget) in self.widgets.iter_mut() {
            r.push(widget.draw_command());
        }
        mem::swap(&mut self.old_widgets, &mut self.widgets);
        self.widgets = collections::HashMap::new();
        r
    }

    pub fn get(&self, id: &u32) -> Option<&W> {
        self.widgets.get(id)
    }
}
