use crate::core::context::*;
use crate::core::draw_commands::*;
use crate::core::user_state::*;
use crate::core::widget::*;

pub struct Checkbox {
    id: u32,
    checked: Box<bool>
}

impl Widget for Checkbox {
    fn id(&self) -> u32 {
        self.id
    }
    fn receive_event(&mut self, user_state: &UserState) {
        
        if user_state.mouse_state.left_click
            && user_state.mouse_state.position.0 < 0.1
            && user_state.mouse_state.position.1 < 0.1
        {
            *self.checked = !*self.checked;
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

pub fn checkbox(context: &mut Context<Checkbox>, id: u32, checked: Box<bool>) -> bool {
    let checkbox = Checkbox {
        id,
        checked,
    };
    context.register_widget(checkbox);
    match context.get(&id) {
        None => false,
        Some(b) => *b.checked,
    }
}


