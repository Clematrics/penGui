use pengui::core::*;

pub trait Mesh {
    fn draw_list(&self) -> &'_ DrawList;
}
