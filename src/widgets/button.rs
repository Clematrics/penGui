use crate::core::draw_commands::*;
use crate::core::user_state::*;
use crate::core::widget::*;
use nalgebra::*;
use std::any::Any;

#[derive(Copy, Clone)]
pub struct Button {
    id: Id,
    data: Option<u32>,
    activated: bool,
	color: [f32; 4],
	texture: Option<TextureId>,
}

impl Widget for Button {
    fn id(&self) -> Id {
        self.id
    }

    fn set_meta_id(&mut self, id: u32) {
        self.id.meta_id = Some(id);
    }

    fn data(&self) -> &dyn Any {
        &self.data
    }
    fn data_mut(&mut self) -> &mut dyn Any {
        &mut self.data
    }

    fn receive_event(&mut self, user_state: &UserEvent) {
        //TODO condition
        self.activated = false;
        self.activated = true;
        *self.data_mut().downcast_mut().unwrap() = Some(5)
    }
    fn draw_commands(
        &mut self,
        unit_x: Vector3<f32>,
        unit_y: Vector3<f32>,
        position: Point3<f32>,
        size: (f32, f32),
		uniforms: &Uniforms
    ) -> Vec<DrawCommand> {
        let pos0 = position - (unit_x * size.0 / 2.) - (unit_y * size.1 / 2.);
        let pos1 = position + (unit_x * size.0 / 2.) - (unit_y * size.1 / 2.);
        let pos2 = position - (unit_x * size.0 / 2.) + (unit_y * size.1 / 2.);
		let pos3 = position + (unit_x * size.0 / 2.) + (unit_y * size.1 / 2.);

		let uniforms = Uniforms {
			texture_0: self.texture,
			.. uniforms.clone()
		};

        vec![DrawCommand {
            vertex_buffer: vec![
                Vertex {
                    position: [pos0.x, pos0.y, pos0.z],
					color: self.color,
					tex_uv: [0., 0.],
                },
                Vertex {
                    position: [pos1.x, pos1.y, pos1.z],
					color: self.color,
					tex_uv: [1., 0.],
                },
                Vertex {
					position: [pos2.x, pos2.y, pos2.z],
                    color: self.color,
					tex_uv: [0., 1.],
                },
                Vertex {
					position: [pos3.x, pos3.y, pos3.z],
                    color: self.color,
					tex_uv: [1., 1.],
                },
            ],
            index_buffer: vec![0, 1, 2, 1, 2, 3],
            draw_mode: DrawMode::TriangleFan,
            clipping: [[0., 0.], [0., 0.]],
            uniforms: uniforms
        }]
    }
    fn send_predecessor(&mut self, old: &mut dyn Widget) {
        match old.data().downcast_ref::<Option<u32>>().unwrap() {
            Some(i) => *self.data_mut().downcast_mut().unwrap() = Some(i - 1),
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

impl UsableWidget for Button {
    type Result = bool;
    type Input = (u32, [f32; 4], Option<TextureId>);

    fn new((id, color, texture): Self::Input) -> Self {
        Button {
            id: Id {
                name: None,
                meta_id: Some(id),
            },
            data: None,
            activated: false,
			color,
			texture
        }
    }

    fn as_dyn_widget(&mut self) -> Box<dyn Widget> {
        Box::new(*self)
    }

    fn result(&self) -> bool {
        self.activated
    }
}
