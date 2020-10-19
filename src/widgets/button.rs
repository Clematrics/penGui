use crate::core::context::*;
use crate::core::draw_commands::*;
use crate::core::user_state::*;
use crate::core::widget::*;

pub struct Button {
    id: u32,
    activated_timer: Option<u32>,
    pub activated: bool,
}

impl Button {
    pub fn new(id: u32, activated: bool) -> Button {
        Button {
            id,
            activated_timer: None,
            activated,
        }
    }
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

#[macro_export]
macro_rules! button_m {
    ($name:ident, $enum:ident) => {
        pub fn $name(context: &mut context::Context<$enum>, id: u32) -> bool {
            let button = Button::new(id, false);
            context.register_widget($enum::Button(button));
            match context.get(&id) {
                Some($enum::Button(b)) => b.activated,
                _ => false,
            }
        }
    };
}

#[macro_export]
macro_rules! button_m_m {
    ($name:ident, $enum:ident) => {
        #[macro_export]
        macro_rules! $name {
            ($name_:ident, $enum_:ident) => {
                pub fn $name_(context: &mut context::Context<$enum_>, id: u32) -> bool {
                    let button = Button::new(id, false);
                    context.register_widget($enum_::$enum::Button(button));
                    match context.get(&id) {
                        Some($enum_::$enum::Button(b)) => b.activated,
                        _ => false,
                    }
                }
            };
        }
    };
}


pub fn button(context: &mut Context<Button>, id: u32) -> bool {
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
