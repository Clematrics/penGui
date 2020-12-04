use std::f32::consts::PI;

use glium::glutin::event::{DeviceEvent, Event, MouseScrollDelta, WindowEvent};
use nalgebra as na;

/// An arc-ball camera
pub struct Camera {
    distance: f32,
    yaw: f32,
    pitch: f32,
    ratio: f32,
}

impl Camera {
    /// Create a new arc-ball camera looking at the origin toward the -z axis,
    /// at a distance of 1.5 by default. The initial ratio is 1.
    pub fn new() -> Self {
        Self {
            distance: 1.5,
            yaw: 0.,
            pitch: 0.,
            ratio: 1.,
        }
    }

    /// Handles events to make the camera respond to mouse and keyboard inputs.
    /// Moving the mouse will make the camera rotate around its point of focus.
    /// Using the mouse wheel will change the distance of the camera.
    pub fn handle_events(&mut self, event: &Event<()>) {
        match event {
            Event::DeviceEvent { event, .. } => match event {
                DeviceEvent::MouseMotion { delta: (dx, dy) } => {
                    self.yaw -= *dx as f32 / 800.;
                    self.pitch += *dy as f32 / 800.;
                    if self.pitch < -(PI / 2.) + 0.1 {
                        self.pitch = -(PI / 2.) + 0.1;
                    }
                    if self.pitch > (PI / 2.) - 0.1 {
                        self.pitch = (PI / 2.) - 0.1;
                    }
                }
                _ => (),
            },
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::MouseWheel {
                    delta: MouseScrollDelta::LineDelta(_, dy),
                    ..
                } => {
                    self.distance += dy;
                    if self.distance <= 0.1 {
                        self.distance = 0.1;
                    }
                }
                _ => (),
            },
            _ => (),
        }
    }

    /// Sets the dimensions of the camera given the width and height of the drawing surface.
    pub fn set_dimensions(&mut self, width: u32, height: u32) {
        self.ratio = width as f32 / height as f32;
    }

    /// Returns the perspective-view matrix of the camera.
    pub fn perspective_view_matrix(&self) -> [[f32; 4]; 4] {
        let perspective = {
            let fovy = PI / 2. / self.ratio;
            let near = 0.1;
            let far = 1000.;

            na::Perspective3::new(self.ratio, fovy, near, far).into_inner()
        };

        let view = {
            let x = self.distance * -self.yaw.sin() * self.pitch.cos();
            let y = self.distance * self.pitch.sin();
            let z = self.distance * -self.yaw.cos() * self.pitch.cos();

            let target = na::Point3::new(0., 0., 0.);
            let eye = na::Point3::new(x, y, z);
            let view = na::Isometry3::look_at_rh(&eye, &target, &na::Vector3::y());

            view.to_homogeneous()
        };

        super::helpers::to_array(&(perspective * view))
    }
}
