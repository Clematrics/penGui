use crate::core::*;


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
            Widget::Checkbox(c) => c.id()
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
            _ => assert!(false)
        }
    }
}

#[macro_export]
macro_rules! expend {
    ( $name:ident, $( $x:ident ),* ) => {
        
            enum $name {
                $(
                    $x($x),
                )*
            }

            impl widget::Widget for $name {
                fn id(&self) -> u32 {
                    match self {
                        $(
                        $name::$x(e) => e.id(),
                        )*                    
                    }
                }
                fn receive_event(&mut self, user_state: &user_state::UserState) {
                    match self {
                        $($name::$x(e) => e.receive_event(user_state),)*
                    }
                }

                fn draw_command(&self) -> draw_commands::DrawCommand {
                    match self {
                        $($name::$x(e) => e.draw_command(),)*
                    }
                }

                fn min_size(&self) -> (f32, f32) {
                    match self {
                        $($name::$x(e) => e.min_size(),)*
                    }
                }

                fn max_size(&self) -> (f32, f32) {
                    match self {
                        $($name::$x(e) => e.max_size(),)*
                    }
                }

                fn preferred_size(&self) -> (f32, f32) {
                    match self {
                        $($name::$x(e) => e.preferred_size(),)*
                    }
                }

                fn send_predecessor(&mut self, old: &Self) {
                    match (self, old) {
                        $(($name::$x(e_self), $name::$x(e_old)) => e_self.send_predecessor(e_old),)*
                        _ => assert!(false)
                    }
                }
            }
            
        
    };
}

 expend![Hey, Checkbox, Widget];

 pub fn button(context: &mut context::Context<Widget>, id: u32) -> bool {
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