use crate::core::context::*;
use crate::core::draw_commands::*;
use crate::core::user_state::*;
use crate::core::widget::*;

pub struct Button {
    id: u32,
    activated_timer: Option<u32>,
    activated: bool,
}

impl Widget for Button {
    fn id(&self) -> u32 {
        self.id
    }
    fn receive_event(&mut self, user_state: &UserState) {
        self.activated = false;
        if user_state.mouse_state.left_click
            && user_state.mouse_state.position.0 < 0.1
            && user_state.mouse_state.position.1 < 0.1
        {
            self.activated = true;
            self.activated_timer = Some(5)
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
    fn send_predecessor(&mut self, old: &Self) {
        match old.activated_timer {
            Some(i) => self.activated_timer = Some(i - 1),
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

fn button<Texture_>(context: &mut Context<Button>, id: u32) -> bool {
    let button = Button {
        id: id,
        activated_timer: None,
        activated: false,
    };
    context.register_widget(button);
    match context.get(&id) {
        None => false,
        Some(b) => b.activated,
    }
}
