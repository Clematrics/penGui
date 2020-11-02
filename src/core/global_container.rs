use std::collections::HashMap;
use std::boxed::Box;

use super::{DrawCommand, DrawMode, Container, Widget};

type ComponentId = u128;

pub struct GlobalContainer {
	map: HashMap<ComponentId, Box<dyn Widget>>
}

impl GlobalContainer {
	pub fn new() -> Self {
		GlobalContainer {
			map: HashMap::<ComponentId, Box<dyn Widget>>::new()
		}
	}
}

impl Widget for GlobalContainer {
	fn mark_valid(&self) -> () {
		// The global container cannot be marked
	}
	fn mark_invalid(&self) -> () {
		// The global container cannot be marked
	}
	fn is_valid(&self) -> bool {
		true // The global container is always valid
	}

	fn draw_commands(&self) -> DrawCommand {
		DrawCommand {
			vertex_buffer: vec![],
			index_buffer: vec![],
			clipping: [[-1., 1.], [1., 1.]],
			draw_mode: DrawMode::TriangleFan,
			texture: None,
			uniforms: vec![],
		}
	}
}

impl Container for GlobalContainer {
	fn validate_content(&mut self) -> () {
		self.map.retain(|_, widget| {
			widget.is_valid()
		});
	}

	fn invalidate_content(&self) -> () {
		self.map.iter().for_each(|(_, widget)| {
			widget.mark_invalid();
		});
	}

	fn add(&self, widget: &dyn Widget) -> () {
		// TODO: implement
	}
}