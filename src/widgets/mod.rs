use crate::core::*;

#[macro_use]
pub mod button;
pub mod checkbox;
use button::*;
use checkbox::*;

pub enum Widget {
    Button(button::Button),
    Checkbox(checkbox::Checkbox),
}

impl widget::Widget for Widget {
    fn id(&self) -> u32 {
        match self {
            Widget::Button(b) => b.id(),
            Widget::Checkbox(c) => c.id(),
        }
    }
    fn receive_event(&mut self, user_state: &user_state::UserState) {
        match self {
            Widget::Button(b) => b.receive_event(user_state),
            Widget::Checkbox(c) => c.receive_event(user_state),
        }
    }

    fn draw_command(&self) -> draw_commands::DrawCommand {
        match self {
            Widget::Button(b) => b.draw_command(),
            Widget::Checkbox(c) => c.draw_command(),
        }
    }

    fn min_size(&self) -> (f32, f32) {
        match self {
            Widget::Button(b) => b.min_size(),
            Widget::Checkbox(c) => c.min_size(),
        }
    }

    fn max_size(&self) -> (f32, f32) {
        match self {
            Widget::Button(b) => b.max_size(),
            Widget::Checkbox(c) => c.max_size(),
        }
    }

    fn preferred_size(&self) -> (f32, f32) {
        match self {
            Widget::Button(b) => b.preferred_size(),
            Widget::Checkbox(c) => c.preferred_size(),
        }
    }

    fn send_predecessor(&mut self, old: &Self) {
        match (self, old) {
            (Widget::Button(b_self), Widget::Button(b_old)) => b_self.send_predecessor(b_old),
            (Widget::Checkbox(c_self), Widget::Checkbox(c_old)) => c_self.send_predecessor(c_old),
            _ => assert!(false),
        }
    }
}

#[macro_export]
macro_rules! combine {
    ( $name:ident, $( ($type:ident, $macro:ident) ),* ) => {
            pub enum $name {
                $(
                    $type($type),
                )*
            }

            impl widget::Widget for $name {
                fn id(&self) -> u32 {
                    match self {
                        $(
                        $name::$type(e) => e.id(),
                        )*
                    }
                }
                fn receive_event(&mut self, user_state: &user_state::UserState) {
                    match self {
                        $($name::$type(e) => e.receive_event(user_state),)*
                    }
                }

                fn draw_command(&self) -> draw_commands::DrawCommand {
                    match self {
                        $($name::$type(e) => e.draw_command(),)*
                    }
                }

                fn min_size(&self) -> (f32, f32) {
                    match self {
                        $($name::$type(e) => e.min_size(),)*
                    }
                }

                fn max_size(&self) -> (f32, f32) {
                    match self {
                        $($name::$type(e) => e.max_size(),)*
                    }
                }

                fn preferred_size(&self) -> (f32, f32) {
                    match self {
                        $($name::$type(e) => e.preferred_size(),)*
                    }
                }

                fn send_predecessor(&mut self, old: &Self) {
                    match (self, old) {
                        $(($name::$type(e_self), $name::$type(e_old)) => e_self.send_predecessor(e_old),)*
                        _ => assert!(false)
                    }
                }
            }
            #[macro_export]
            macro_rules!  {
                () => {
                };
            }
            #[macro_export]
            macro_rules! $name {
                ($name:ident, $enum:ident) => {
            $(
                $macro!
                pub fn $name(context: &mut context::Context<$enum>, id: u32) -> bool {
                    let button = Button::new(id, false);
                    context.register_widget($enum::Button(button));
                    match context.get(&id) {
                        Some($enum::Button(b)) => b.activated,
                        _ => false,
                    }
                }
            )*
                    };
                }
    };
}

combine![Hey, (Checkbox, button_m_m), (Button, button_m_m)];
