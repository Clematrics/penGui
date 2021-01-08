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
    /// The boolean `should_move` is an additional guard for the camera to move.
    pub fn handle_events(&mut self, event: &Event<()>, should_move: bool) {
        match event {
            Event::DeviceEvent { event, .. } => match event {
                DeviceEvent::MouseMotion { delta: (dx, dy) } if should_move => {
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

    pub fn position(&self) -> na::Point3<f32> {
        let x = self.distance * -self.yaw.sin() * self.pitch.cos();
        let y = self.distance * self.pitch.sin();
        let z = self.distance * -self.yaw.cos() * self.pitch.cos();

        na::Point3::new(x, y, z)
    }

    /// Returns the perspective matrix of the camera.
    fn perspective_matrix(&self) -> na::Perspective3<f32> {
        let fovy = PI / 2. / self.ratio;
        let near = 0.1;
        let far = 1000.;

        na::Perspective3::new(self.ratio, fovy, near, far)
    }

    /// Returns the view matrix of the camera.
    fn view_matrix(&self) -> na::Matrix4<f32> {
        let target = na::Point3::new(0., 0., 0.);
        let eye = self.position();
        let view = na::Isometry3::look_at_rh(&eye, &target, &na::Vector3::y());

        view.to_homogeneous()
    }

    /// Returns the perspective-view matrix of the camera.
    pub fn perspective_view_matrix(&self) -> na::Matrix4<f32> {
        self.perspective_matrix().into_inner() * self.view_matrix()
    }

    /// Returns a normalized ray representing the mouse position on screen,
    /// given its normalized position.
    pub fn ray_from(&self, x: f32, y: f32) -> na::Unit<na::Matrix3x1<f32>> {
        let projection = self.perspective_matrix();

        // Compute two points in clip-space.
        // "ndc" = normalized device coordinates.
        let near_ndc_point = na::Point3::new(x, y, -1.0);
        let far_ndc_point = na::Point3::new(x, y, 1.0);

        // Unproject them to view-space.
        let near_view_point = projection.unproject_point(&near_ndc_point);
        let far_view_point = projection.unproject_point(&far_ndc_point);

        // Compute the view-space line parameters.
        let line_direction = na::Unit::new_normalize(far_view_point - near_view_point);

        line_direction
    }
}
