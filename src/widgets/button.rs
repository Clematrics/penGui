use crate::core::draw_commands::*;
use crate::core::user_state::*;
use crate::core::widget::*;
use std::any::Any;

#[derive(Copy, Clone)]
pub struct Button {
    id: Id,
    data: Option<u32>,
    activated: bool,
}

impl Widget for Button {
    fn id(&self) -> Id {
        self.id
    }

    fn set_meta_id(&mut self, id: u32) {
        self.id.meta_id = Some(id);
    }

    fn data(&self) -> &dyn Any {
        &self.data
    }
    fn data_mut(&mut self) -> &mut dyn Any {
        &mut self.data
    }

    fn receive_event(&mut self, user_state: &UserEvent) {
        //TODO condition
        self.activated = false;
        self.activated = true;
        *self.data_mut().downcast_mut().unwrap() = Some(5)
    }
    fn draw_command(&mut self) -> Vec<DrawCommand> {
        vec![DrawCommand {
            vertex_buffer: Vec::new(),
            index_buffer: Vec::new(),         // Wrapper
            draw_mode: DrawMode::TriangleFan, //
            clipping: [[0., 0.], [0., 0.]],
            uniforms: Vec::new(), // Option
            texture: None,
        }]
    }
    fn send_predecessor(&mut self, old: &dyn Widget) {
        match old.data().downcast_ref::<Option<u32>>().unwrap() {
            Some(i) => *self.data_mut().downcast_mut().unwrap() = Some(i - 1),
            None => (),
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
}

impl UsableWidget<bool> for Button {
    fn as_dyn_widget(&mut self) -> Box<dyn Widget> {
        Box::new(*self)
    }

    fn result(&self) -> bool {
        self.activated
    }
}
