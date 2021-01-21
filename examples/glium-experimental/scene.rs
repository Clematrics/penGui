use super::mesh::Mesh;
use super::meshes::NoiseFloor;
use super::setup::camera::*;
use super::setup::main_window::*;
use super::ui::Ui;

use pengui::backend::glium::*;
use pengui::core::*;
use pengui::frontend::glutin::Input;

use glium::glutin::event::{Event as GlutinEvent, WindowEvent};
use glium::Surface;

pub struct Scene {
    backend: GliumBackend,
    camera: Camera,
    pub ui: Ui,

    noise_floor: NoiseFloor,
}

impl Scene {
    pub fn new(mut backend: GliumBackend) -> Self {
        let ui = Ui::new(&mut backend);
        Self {
            backend,
            camera: Default::default(),
            ui,
            noise_floor: Default::default(),
        }
    }

    pub fn handle_events(&mut self, event: &GlutinEvent<()>, main_window: &MainWindow) {
        self.camera.handle_events(&event, main_window.alt_pressed);

        match &event {
            GlutinEvent::WindowEvent { event, .. } => match event {
                WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(glium::glutin::event::VirtualKeyCode::D) = input.virtual_keycode {
                        if let glium::glutin::event::ElementState::Released = input.state {
                            if main_window.alt_pressed {
                                self.backend.switch_debug_rendering();
                            }
                        }
                    }
                    if let Some(event) = Input::from(event) {
                        self.ui.register_event(event, None);
                    }
                }
                WindowEvent::Touch(_)
                | WindowEvent::CursorMoved { .. }
                | WindowEvent::MouseInput { .. }
                | WindowEvent::MouseWheel { .. } => {
                    if let Some(event) = Input::from(event) {
                        let ray = {
                            let (x, y) = main_window.mouse_pos;
                            Some(self.camera.ray_from(x, y))
                        };
                        self.ui.register_event(event, ray.as_ref());
                    }
                }
                event => {
                    if let Some(event) = Input::from(event) {
                        self.ui.register_event(event, None);
                    }
                }
            },
            _ => (),
        }
    }

    pub fn render(&mut self, _time: f32) {
        if self.ui.radius != self.noise_floor.radius {
            self.noise_floor.change_radius(self.ui.radius);
        }

        let mut target = self.backend.new_frame();
        target.clear_color_and_depth((0.447, 0.549, 0.6, 1.0), 1.0);

        let (width, height) = target.get_dimensions();
        self.camera.set_dimensions(width, height);

        let view_matrix = self.camera.perspective_view_matrix();

        self.backend
            .draw_list(
                &mut target,
                view_matrix,
                Mat4x4::identity(),
                self.noise_floor.draw_list(),
            )
            .unwrap();

        self.backend
            .draw_list(
                &mut target,
                view_matrix,
                Mat4x4::identity(),
                &self.ui.draw_list(),
            )
            .expect("error while rendering ui");

        target.finish().unwrap();
    }
}
