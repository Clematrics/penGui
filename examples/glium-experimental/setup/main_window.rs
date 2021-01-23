use std::time::Instant;

use glium::glutin::{
    dpi::LogicalSize,
    event::{Event, Touch, TouchPhase, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    ContextBuilder,
};
use glium::Display;

/// Default width for the window
pub const DEFAULT_WINDOW_WIDTH: u32 = 1024;
pub const DEFAULT_WINDOW_HEIGHT: u32 = 768;

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

    pub window_size: (u32, u32),
    pub mouse_pos: (f32, f32),
    pub mouse_inside: bool,
    pub alt_pressed: bool,
    pub ctrl_pressed: bool,
}

impl MainWindow {
    /// Creates a new window, its associated event loop and the drawing surface.
    pub fn new() -> (Self, EventLoop<()>, Display) {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title("Hello world!")
            .with_inner_size(LogicalSize::new(
                DEFAULT_WINDOW_WIDTH,
                DEFAULT_WINDOW_HEIGHT,
            ));
        let context = ContextBuilder::new().with_srgb(false);
        let display = Display::new(window, context, &event_loop).unwrap();

        let start_time = std::time::Instant::now();
        let last_frame_time = start_time;

        (
            MainWindow {
                start_time,
                last_frame_time,
                window_size: (DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT),
                mouse_pos: (0., 0.),
                mouse_inside: false,
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
        delta_t.as_nanos() as u64
    }

    /// Updates the last frame time and returns the time elapsed since the beginning.
    pub fn new_frame_time(&mut self) -> f32 {
        self.last_frame_time = Instant::now();

        let time = Instant::now() - self.start_time;
        time.as_secs_f32()
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
    /// React to the close button and the `Ctrl+W` key, which closes the window.
    pub fn handle_events(&mut self, event: &Event<()>, control_flow: &mut ControlFlow) {
        if let Event::WindowEvent { event, .. } = event {
            match event {
                WindowEvent::Resized(size) => {
                    self.window_size = (size.width, size.height);
                }
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                WindowEvent::CursorMoved { position, .. } => {
                    self.mouse_pos = (position.x as f32, position.y as f32);
                }
                WindowEvent::CursorEntered { .. } => self.mouse_inside = true,
                WindowEvent::CursorLeft { .. } => self.mouse_inside = false,
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
                WindowEvent::Touch(Touch {
                    phase, location, ..
                }) => {
                    if let TouchPhase::Moved = phase {
                        self.mouse_pos = (location.x as f32, location.y as f32);
                    }
                }
                _ => (),
            }
        }
    }
}
