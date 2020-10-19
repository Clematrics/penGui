use super::draw_commands::*;
use super::user_state::*;
use dynamic::Dynamic;
use std::any::Any;
pub trait Widget: Any + 'static {
    fn id(&self) -> u32;
    fn data_mut(&mut self) -> Box<Dynamic>;
    fn data(&self) -> Box<Dynamic>;
    fn receive_event(&mut self, user_state: &UserState);
    fn draw_command(&self) -> DrawCommand;
    fn send_predecessor(&mut self, old: &dyn Widget);
    fn min_size(&self) -> (f32, f32);
    fn max_size(&self) -> (f32, f32);
    fn preferred_size(&self) -> (f32, f32);
}
