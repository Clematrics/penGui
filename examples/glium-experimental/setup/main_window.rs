use glium::glutin::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    ContextBuilder,
};
use glium::{glutin, Display};
use std::time::Instant;

pub const MAX_FRAME_DELAY_NS: u64 = 16_666_667;

pub struct MainWindow {
    pub start_time: Instant,
    pub last_frame_time: Instant,
}

impl MainWindow {
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
            },
            event_loop,
            display,
        )
    }

    pub fn get_delta_time(&self) -> (u64, bool) {
        let delta_t = Instant::now() - self.last_frame_time;
        let delta_t = delta_t.as_nanos() as u64;

        (delta_t, delta_t < MAX_FRAME_DELAY_NS)
    }

    pub fn new_frame_time(&mut self) -> f32 {
        self.last_frame_time = Instant::now();

        let time = Instant::now() - self.start_time;
        let time = time.as_secs_f32();

        time
    }

    pub fn end_frame(&self, control_flow: &mut ControlFlow) {
        if *control_flow == ControlFlow::Exit {
            return;
        }

        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(MAX_FRAME_DELAY_NS);

        *control_flow = ControlFlow::WaitUntil(next_frame_time);
    }

    pub fn handle_events(&self, event: &Event<()>, control_flow: &mut ControlFlow) {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(key) = input.virtual_keycode {
                        if key == glutin::event::VirtualKeyCode::Q {
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
