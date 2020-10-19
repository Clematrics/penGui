use crate::core::context::*;
use crate::core::draw_commands::*;
use crate::core::user_state::*;
use crate::core::widget::*;
use dynamic::*;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Copy, Clone)]
pub struct Button {
    id: u32,
    data: Option<u32>,
    activated: bool,
}

impl Widget for Button {
    fn id(&self) -> u32 {
        self.id
    }

    fn data(&self) -> Box<Dynamic> {
        Dynamic::new(Described::new(self.data))
    }
    fn data_mut(&mut self) -> Box<Dynamic> {
        Dynamic::new(Described::new(self.data))
    }

    fn receive_event(&mut self, user_state: &UserState) {
        self.activated = false;
        if user_state.mouse_state.left_click
            && user_state.mouse_state.position.0 < 0.1
            && user_state.mouse_state.position.1 < 0.1
        {
            self.activated = true;
            *self.data().downcast_mut().unwrap() = Some(5)
        }
    }
    fn draw_command(&self) -> DrawCommand {
        DrawCommand {
            vertex_buffer: Vec::new(),
            index_buffer: Vec::new(),         // Wrapper
            draw_mode: DrawMode::TriangleFan, //
            clipping: [[0., 0.], [0., 0.]],
            uniforms: Vec::new(), // Option
            texture: None,
        }
    }
    fn send_predecessor(&mut self, old: &dyn  Widget) {
        match old.data().downcast_ref::<Option<u32>>().unwrap() {
            Some(i) => *self.data().downcast_mut().unwrap() = Some(i - 1),
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

fn button<Texture_>(context: &mut Context, id: u32) -> bool {
    let button = Button {
        id: id,
        data: None,
        activated: false,
    };

    context.register_widget(button).activated
}
