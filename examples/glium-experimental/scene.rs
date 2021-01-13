use super::mesh::Mesh;
use super::meshes::TestCube;
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

    test_cube: TestCube,
}

impl Scene {
    pub fn new(mut backend: GliumBackend) -> Self {
        let ui = Ui::new(&mut backend);
        Self {
            backend,
            camera: Default::default(),
            ui,
            test_cube: Default::default(),
        }
    }

    pub fn handle_events(&mut self, event: &GlutinEvent<()>, main_window: &MainWindow) {
        self.camera.handle_events(&event, main_window.alt_pressed);

        match &event {
            GlutinEvent::WindowEvent { event, .. } => match event {
                WindowEvent::ReceivedCharacter(c) => match c {
                    '\u{8}' => {
                        self.ui.editable_text.pop();
                    }
                    _ if *c != '\u{7f}' => self.ui.editable_text.push(*c),
                    _ => {}
                },
                WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(glium::glutin::event::VirtualKeyCode::D) = input.virtual_keycode {
                        if let glium::glutin::event::ElementState::Released = input.state {
                            if main_window.alt_pressed {
                                self.backend.switch_debug_rendering();
                            }
                        }
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
                _ => (),
            },
            _ => (),
        }
    }

    pub fn render(&mut self, time: f32) {
        let mut target = self.backend.new_frame();
        let blue = (1. + f32::sin(time + std::f32::consts::PI)) / 2.;
        let red = (1. + f32::sin(time)) / 2.;
        target.clear_color_and_depth((red, 0.0, blue, 1.0), 1.0);

        let (width, height) = target.get_dimensions();
        self.camera.set_dimensions(width, height);

        let view_matrix = self.camera.perspective_view_matrix();

        self.backend
            .draw_list(
                &mut target,
                view_matrix,
                Mat4x4::identity(),
                self.test_cube.draw_list(),
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
