use super::container::*;
use super::draw_commands::*;
use super::user_state::*;
use nalgebra::*;
use std::any::Any;
use std::hash::Hash;
#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct Id {
    pub name: Option<u32>,
    pub meta_id: Option<u32>,
}

pub trait Widget: Any + 'static {
    fn id(&self) -> Id;
    fn set_meta_id(&mut self, id: u32);
    fn data_mut(&mut self) -> &mut dyn Any;
    fn data(&self) -> &dyn Any;
    fn receive_event(&mut self, event: &UserEvent);
    fn draw_commands(
        &mut self,
        unit_x: Vector3<f32>,
        unit_y: Vector3<f32>,
        position: Point3<f32>,
        size: (f32, f32),
        uniforms: &Uniforms,
    ) -> Vec<DrawCommand>;
    fn send_predecessor(&mut self, old: &mut dyn Widget);
    fn min_size(&self) -> (f32, f32);
    fn max_size(&self) -> (f32, f32);
    fn preferred_size(&self) -> (f32, f32);
}

pub trait UsableWidget
where
    Self: Widget + Copy,
{
    type Result;
    type Input;

    fn new(input: Self::Input) -> Self;

    fn as_dyn_widget(&mut self) -> Box<dyn Widget> {
        Box::new(*self)
    }

    fn result(&self) -> Self::Result;

    fn build<C: UsableContainer>(&mut self, container: &mut C) -> Self::Result {
        container.add_widget(self)
    }
}
