use std::time::Instant;

use glium::glutin::{
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    ContextBuilder,
};
use glium::Display;

/// Max delay in nanoseconds between two frames to get 60 frames per second.
pub const MAX_FRAME_DELAY_NS: u64 = 16_666_667;

/// Main window.
///
/// This structure is mainly an helper to tidy the window, event loop and openGL context creation.
///
/// Holds the time at which the window was created, and the time of the last frame.
pub struct MainWindow {
    pub start_time: Instant,
    pub last_frame_time: Instant,

    pub alt_pressed: bool,
    pub ctrl_pressed: bool,
}

impl MainWindow {
    /// Creates a new window, its associated event loop and the drawing surface.
    pub fn new() -> (Self, EventLoop<()>, Display) {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new();
        let context = ContextBuilder::new().with_srgb(false);
        let display = Display::new(window, context, &event_loop).unwrap();

        let start_time = std::time::Instant::now();
        let last_frame_time = start_time;

        (
            MainWindow {
                start_time,
                last_frame_time,
                alt_pressed: false,
                ctrl_pressed: false,
            },
            event_loop,
            display,
        )
    }

    /// Returns the time elapsed since the last frame was created.
    pub fn get_delta_time(&self) -> u64 {
        let delta_t = Instant::now() - self.last_frame_time;
        let delta_t = delta_t.as_nanos() as u64;

        delta_t
    }

    /// Updates the last frame time and returns the time elapsed since the beginning.
    pub fn new_frame_time(&mut self) -> f32 {
        self.last_frame_time = Instant::now();

        let time = Instant::now() - self.start_time;
        let time = time.as_secs_f32();

        time
    }

    /// Updates the control flow to redraw the frame in 16ms if necessary.
    pub fn end_frame(&self, control_flow: &mut ControlFlow) {
        if *control_flow == ControlFlow::Exit {
            return;
        }

        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(MAX_FRAME_DELAY_NS);

        *control_flow = ControlFlow::WaitUntil(next_frame_time);
    }

    /// Handles events related to the window.
    /// Only react to the close button and the `Q` key, which closes the window.
    pub fn handle_events(&mut self, event: &Event<()>, control_flow: &mut ControlFlow) {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                WindowEvent::ModifiersChanged(modifiers) => {
                    self.alt_pressed = modifiers.alt();
                    self.ctrl_pressed = modifiers.ctrl();
                }
                WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(VirtualKeyCode::W) = input.virtual_keycode {
                        if self.ctrl_pressed {
                            *control_flow = ControlFlow::Exit;
                        }
                    }
                }
                _ => (),
            },
            _ => (),
        }
    }
}
