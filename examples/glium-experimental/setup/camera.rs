use std::f32::consts::{FRAC_PI_2, PI};

use glium::glutin::event::{DeviceEvent, Event, MouseScrollDelta, WindowEvent};
use nalgebra as na;
use pengui::core::Ray;

/// An arc-ball camera
pub struct Camera {
    position: na::Vector3<f32>,
    speed: f32,
    yaw: f32,
    pitch: f32,
    width: f32,
    height: f32,
    ratio: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self::new()
    }
}

impl Camera {
    /// Create a new arc-ball camera looking at the origin toward the -z axis,
    /// at a distance of 1.5 by default. The initial ratio is 1.
    pub fn new() -> Self {
        let width = super::main_window::DEFAULT_WINDOW_WIDTH as f32;
        let height = super::main_window::DEFAULT_WINDOW_HEIGHT as f32;

        Self {
            position: na::Vector3::new(0., 0., 0.),
            speed: 0.1,
            yaw: 0.,
            pitch: 0.,
            width,
            height,
            ratio: width / height,
        }
    }

    /// Handles events to make the camera respond to mouse and keyboard inputs.
    /// Moving the mouse will make the camera rotate around its point of focus.
    /// Using the mouse wheel will change the distance of the camera.
    /// The boolean `should_move` is an additional guard for the camera to move.
    pub fn handle_events(&mut self, event: &Event<()>, should_move: bool) {
        match event {
            Event::DeviceEvent { event, .. } => match event {
                DeviceEvent::MouseMotion { delta: (dx, dy) } if should_move => {
                    self.yaw -= *dx as f32 / 800.;
                    self.pitch += *dy as f32 / 800.;
                    if self.pitch < -FRAC_PI_2 + 0.1 {
                        self.pitch = -FRAC_PI_2 + 0.1;
                    }
                    if self.pitch > FRAC_PI_2 - 0.1 {
                        self.pitch = FRAC_PI_2 - 0.1;
                    }
                }
                _ => (),
            },
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::MouseWheel {
                    delta: MouseScrollDelta::LineDelta(_, dy),
                    ..
                } => {
                    self.speed -= dy;
                    if self.speed <= 0.1 {
                        self.speed = 0.1;
                    }
                    if self.speed >= 2. {
                        self.speed = 2.;
                    }
                }
                WindowEvent::KeyboardInput { input, .. } => match input.virtual_keycode {
                    Some(glium::glutin::event::VirtualKeyCode::LShift) if should_move => {
                        self.position += self.speed * na::Vector3::y()
                    }
                    Some(glium::glutin::event::VirtualKeyCode::LControl) if should_move => {
                        self.position -= self.speed * na::Vector3::y()
                    }
                    Some(glium::glutin::event::VirtualKeyCode::Up) => {
                        self.position += self.speed * self.direction()
                    }
                    Some(glium::glutin::event::VirtualKeyCode::Down) => {
                        self.position -= self.speed * self.direction()
                    }
                    Some(glium::glutin::event::VirtualKeyCode::Left) => {
                        let rotation = na::UnitQuaternion::from_axis_angle(
                            &na::Vector3::<f32>::y_axis(),
                            FRAC_PI_2,
                        );
                        self.position += self.speed
                            * (rotation * na::Vector3::new(-self.yaw.sin(), 0., -self.yaw.cos()));
                    }
                    Some(glium::glutin::event::VirtualKeyCode::Right) => {
                        let rotation = na::UnitQuaternion::from_axis_angle(
                            &na::Vector3::<f32>::y_axis(),
                            FRAC_PI_2,
                        );
                        self.position -= self.speed
                            * (rotation * na::Vector3::new(-self.yaw.sin(), 0., -self.yaw.cos()));
                    }
                    _ => (),
                },
                _ => (),
            },
            _ => (),
        }
    }

    /// Sets the dimensions of the camera given the width and height of the drawing surface.
    pub fn set_dimensions(&mut self, width: u32, height: u32) {
        self.width = width as f32;
        self.height = height as f32;
        self.ratio = self.width / self.height;
    }

    pub fn direction(&self) -> na::Vector3<f32> {
        let x = -self.yaw.sin() * self.pitch.cos();
        let y = -self.pitch.sin();
        let z = -self.yaw.cos() * self.pitch.cos();

        na::Vector3::new(x, y, z)
    }

    /// Returns the perspective matrix of the camera.
    fn perspective_matrix(&self) -> na::Perspective3<f32> {
        let fovy = PI / 2. / self.ratio;
        let near = 0.1;
        let far = 1000.;

        na::Perspective3::new(self.ratio, fovy, near, far)
    }

    /// Returns the view matrix of the camera.
    fn view_matrix(&self) -> na::Isometry3<f32> {
        let target = na::Point3::from(self.position + self.direction());
        let eye = na::Point3::from(self.position);
        na::Isometry3::look_at_rh(&eye, &target, &na::Vector3::y())
    }

    /// Returns the perspective-view matrix of the camera.
    pub fn perspective_view_matrix(&self) -> na::Matrix4<f32> {
        self.perspective_matrix().into_inner() * self.view_matrix().to_homogeneous()
    }

    /// Returns a normalized ray representing the mouse position on screen,
    /// given its normalized position.
    pub fn ray_from(&self, x: f32, y: f32) -> Ray {
        let projection = self.perspective_matrix();

        // Compute two points in clip-space.
        // "ndc" = normalized device coordinates.
        let near_ndc_point =
            na::Point3::new(2. * x / self.width - 1., 1. - 2. * y / self.height, -1.0);
        let far_ndc_point =
            na::Point3::new(2. * x / self.width - 1., 1. - 2. * y / self.height, 1.0);

        // Unproject them to view-space.
        let near_view_point = projection.unproject_point(&near_ndc_point);
        let far_view_point = projection.unproject_point(&far_ndc_point);

        // Compute the view-space line parameters.
        let line_direction = na::Unit::new_normalize(far_view_point - near_view_point);
        let line_location = near_view_point;

        let mut view = self.view_matrix();
        view.inverse_mut();

        Ray::new((view * line_direction).into_inner(), view * line_location)
    }
}
