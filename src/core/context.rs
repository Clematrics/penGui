use super::container::*;
use super::draw_commands::*;
use super::user_state::*;
use super::widget::*;
use std::collections;
use std::mem;

pub struct Context {
    old_widgets: collections::HashMap<Id, WidgetOrContainer>,
    widgets: collections::HashMap<Id, WidgetOrContainer>,
}

impl Context {
    pub fn new() -> Context {
        Context {
            old_widgets: collections::HashMap::new(),
            widgets: collections::HashMap::new(),
        }
    }
}

impl Container for Context {
    fn old_widgets(&mut self) -> &mut collections::HashMap<Id, WidgetOrContainer> {
        &mut self.old_widgets
    }
    fn widgets(&mut self) -> &mut collections::HashMap<Id, WidgetOrContainer> {
        &mut self.widgets
    }

    fn new_event(&mut self, event: &UserEvent) {
        for (_, v) in self.widgets.iter_mut() {
            //TODO: condition here
            match v {
                WidgetOrContainer::Widget(widget) => widget.receive_event(event),
                WidgetOrContainer::Container(container) => container.new_event(event),
            }
        }
    }

    fn draw(&mut self) -> Vec<DrawCommand> {
        let mut r = Vec::new();
        for (_, v) in self.widgets.iter_mut() {
            match v {
                WidgetOrContainer::Widget(widget) => r.push(widget.draw_command()),
                WidgetOrContainer::Container(container) => {
                    let mut commands = container.draw();
                    r.append(&mut commands)
                }
            }
        }
        mem::swap(&mut self.old_widgets, &mut self.widgets);
        self.widgets = collections::HashMap::new();
        r
    }

    fn get(&self, id: &Id) -> Option<&WidgetOrContainer> {
        self.widgets.get(id)
    }

    fn get_mut(&mut self, id: &Id) -> Option<&mut WidgetOrContainer> {
        self.widgets.get_mut(id)
    }
}
