use super::draw_commands::*;
use super::user_state::*;

pub trait Widget {
    fn id(&self) -> u32;
    fn receive_event(&mut self, user_state: &UserState);
    fn draw_command(&self) -> DrawCommand;
    fn send_predecessor(&mut self, old: &Self);
    fn min_size(&self) -> (f32, f32);
    fn max_size(&self) -> (f32, f32);
    fn preferred_size(&self) -> (f32, f32);
}
